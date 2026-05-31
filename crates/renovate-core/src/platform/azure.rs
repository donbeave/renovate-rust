//! Azure DevOps platform client.
//!
//! Implements [`PlatformClient`] against the Azure DevOps REST API.
//!
//! Renovate reference: `lib/modules/platform/azure/index.ts`.
//!
//! ## Authentication
//!
//! Azure DevOps uses HTTP Basic Auth with `:PAT` (empty username, PAT as password).

use base64::Engine as _;
use serde::{Deserialize, Serialize};

use crate::http::{HttpClient, HttpError};
use crate::platform::{CombinedBranchStatus, CurrentUser, PlatformClient, PlatformError, RawFile};

#[derive(Debug, Clone)]
pub struct AzureClient {
    http: HttpClient,
    api_base: String,
    #[allow(dead_code)]
    org: String,
    #[allow(dead_code)]
    project: String,
    pat: String,
    endpoint_base: String,
}

impl AzureClient {
    pub fn new(
        org: impl Into<String>,
        project: impl Into<String>,
        pat: impl Into<String>,
    ) -> Result<Self, HttpError> {
        let org = org.into();
        let project = project.into();
        let pat = pat.into();
        let api_base = format!("https://dev.azure.com/{org}/{project}/_apis/git");
        let endpoint_base = format!("https://dev.azure.com/{org}");
        Ok(Self {
            http: HttpClient::new()?,
            api_base,
            org,
            project,
            pat,
            endpoint_base,
        })
    }

    pub fn with_endpoint(
        endpoint: impl Into<String>,
        org: impl Into<String>,
        project: impl Into<String>,
        pat: impl Into<String>,
    ) -> Result<Self, HttpError> {
        let endpoint = endpoint.into().trim_end_matches('/').to_owned();
        let org = org.into();
        let project = project.into();
        let pat = pat.into();
        let api_base = format!("{endpoint}/{org}/{project}/_apis/git");
        let endpoint_base = format!("{endpoint}/{org}");
        Ok(Self {
            http: HttpClient::new()?,
            api_base,
            org,
            project,
            pat,
            endpoint_base,
        })
    }

    fn auth_header(&self) -> String {
        let creds = format!(":{}", self.pat);
        format!(
            "Basic {}",
            base64::engine::general_purpose::STANDARD.encode(creds)
        )
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct AzureIdentity {
    pub id: Option<String>,
    pub display_name: Option<String>,
    pub unique_name: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AzureRepo {
    pub id: Option<String>,
    pub name: Option<String>,
    pub url: Option<String>,
    pub project: Option<AzureProject>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AzureProject {
    pub id: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AzurePr {
    pub pull_request_id: i64,
    pub title: String,
    pub description: Option<String>,
    pub status: Option<AzurePrStatus>,
    pub source_ref_name: Option<String>,
    pub target_ref_name: Option<String>,
    pub created_by: Option<AzureIdentity>,
    pub creation_date: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AzurePrList {
    pub value: Vec<AzurePr>,
    pub count: Option<i64>,
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AzurePrStatus {
    NotSet,
    Active,
    Abandoned,
    Completed,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AzureComment {
    pub id: Option<i64>,
    pub content: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AzureCommentThread {
    pub id: Option<i64>,
    pub comments: Option<Vec<AzureComment>>,
}

#[derive(Debug, Serialize)]
struct CreatePrRequest {
    source_ref_name: String,
    target_ref_name: String,
    title: String,
    description: String,
}

#[derive(Debug, Serialize)]
struct CommentRequest {
    comments: Vec<CommentEntry>,
}

#[derive(Debug, Serialize)]
struct CommentEntry {
    content: String,
    comment_type: String,
}

pub async fn create_pr(
    client: &AzureClient,
    repo_id: &str,
    source_branch: &str,
    target_branch: &str,
    title: &str,
    body: &str,
) -> Result<Option<i64>, PlatformError> {
    let url = format!(
        "{}/repositories/{}/pullrequests?api-version=7.0",
        client.api_base, repo_id
    );
    let request = CreatePrRequest {
        source_ref_name: format!("refs/heads/{source_branch}"),
        target_ref_name: format!("refs/heads/{target_branch}"),
        title: title.to_owned(),
        description: body.to_owned(),
    };
    let request_json = serde_json::to_string(&request)
        .map_err(|e| PlatformError::Unexpected(format!("JSON serialize: {e}")))?;

    let rb = client
        .http
        .inner
        .post(&url)
        .header("Authorization", client.auth_header())
        .header("Content-Type", "application/json")
        .body(request_json);
    let resp = rb.send().await.map_err(HttpError::Request)?;
    if !resp.status().is_success() {
        if resp.status() == reqwest::StatusCode::CONFLICT {
            return Ok(None);
        }
        return Err(PlatformError::Http(HttpError::Status {
            status: resp.status(),
            url,
        }));
    }
    let pr: AzurePr = resp.json().await.map_err(HttpError::Request)?;
    Ok(Some(pr.pull_request_id))
}

pub async fn list_prs(
    client: &AzureClient,
    repo_id: &str,
    status: Option<&str>,
) -> Result<Vec<AzurePr>, PlatformError> {
    let status_filter = status.unwrap_or("active");
    let url = format!(
        "{}/repositories/{}/pullrequests?searchCriteria.status={}&api-version=7.0",
        client.api_base, repo_id, status_filter
    );
    let rb = client
        .http
        .inner
        .get(&url)
        .header("Authorization", client.auth_header());
    let resp = rb.send().await.map_err(HttpError::Request)?;
    if !resp.status().is_success() {
        return Err(PlatformError::Http(HttpError::Status {
            status: resp.status(),
            url,
        }));
    }
    let list: AzurePrList = resp.json().await.map_err(HttpError::Request)?;
    Ok(list.value)
}

pub async fn merge_pr(
    client: &AzureClient,
    repo_id: &str,
    pr_id: i64,
) -> Result<(), PlatformError> {
    let url = format!(
        "{}/repositories/{}/pullrequests/{}?api-version=7.0",
        client.api_base, repo_id, pr_id
    );
    #[derive(Serialize)]
    struct MergeRequest {
        status: String,
        last_merge_source_commit: LastMergeCommit,
    }
    #[derive(Serialize)]
    struct LastMergeCommit {
        commit_id: String,
    }
    let request = MergeRequest {
        status: "completed".to_owned(),
        last_merge_source_commit: LastMergeCommit {
            commit_id: String::new(),
        },
    };
    let request_json = serde_json::to_string(&request)
        .map_err(|e| PlatformError::Unexpected(format!("JSON serialize: {e}")))?;

    let rb = client
        .http
        .inner
        .patch(&url)
        .header("Authorization", client.auth_header())
        .header("Content-Type", "application/json")
        .body(request_json);
    let resp = rb.send().await.map_err(HttpError::Request)?;
    if !resp.status().is_success() {
        return Err(PlatformError::Http(HttpError::Status {
            status: resp.status(),
            url,
        }));
    }
    Ok(())
}

pub async fn add_comment(
    client: &AzureClient,
    repo_id: &str,
    pr_id: i64,
    text: &str,
) -> Result<AzureCommentThread, PlatformError> {
    let url = format!(
        "{}/repositories/{}/pullrequests/{}/threads?api-version=7.0",
        client.api_base, repo_id, pr_id
    );
    let request = CommentRequest {
        comments: vec![CommentEntry {
            content: text.to_owned(),
            comment_type: "text".to_owned(),
        }],
    };
    let request_json = serde_json::to_string(&request)
        .map_err(|e| PlatformError::Unexpected(format!("JSON serialize: {e}")))?;

    let rb = client
        .http
        .inner
        .post(&url)
        .header("Authorization", client.auth_header())
        .header("Content-Type", "application/json")
        .body(request_json);
    let resp = rb.send().await.map_err(HttpError::Request)?;
    if !resp.status().is_success() {
        return Err(PlatformError::Http(HttpError::Status {
            status: resp.status(),
            url,
        }));
    }
    let thread: AzureCommentThread = resp.json().await.map_err(HttpError::Request)?;
    Ok(thread)
}

pub async fn get_pr(
    client: &AzureClient,
    repo_id: &str,
    pr_id: i64,
) -> Result<AzurePr, PlatformError> {
    let url = format!(
        "{}/repositories/{}/pullrequests/{}?api-version=7.0",
        client.api_base, repo_id, pr_id
    );
    let rb = client
        .http
        .inner
        .get(&url)
        .header("Authorization", client.auth_header());
    let resp = rb.send().await.map_err(HttpError::Request)?;
    if resp.status() == reqwest::StatusCode::NOT_FOUND {
        return Err(PlatformError::Unexpected("PR not found".to_owned()));
    }
    if !resp.status().is_success() {
        return Err(PlatformError::Http(HttpError::Status {
            status: resp.status(),
            url,
        }));
    }
    let pr: AzurePr = resp.json().await.map_err(HttpError::Request)?;
    Ok(pr)
}

impl PlatformClient for AzureClient {
    async fn get_current_user(&self) -> Result<CurrentUser, PlatformError> {
        let connection_url = format!(
            "{}/_apis/connectionData?api-version=7.0",
            self.endpoint_base
        );
        let rb = self
            .http
            .inner
            .get(&connection_url)
            .header("Authorization", self.auth_header());
        let resp = rb.send().await.map_err(HttpError::Request)?;
        if resp.status() == reqwest::StatusCode::UNAUTHORIZED {
            return Err(PlatformError::Unauthorized);
        }
        if !resp.status().is_success() {
            return Ok(CurrentUser {
                login: "azure-user".to_owned(),
            });
        }
        #[derive(Deserialize)]
        struct ConnectionData {
            authenticated_user: Option<AuthenticatedUser>,
        }
        #[derive(Deserialize)]
        struct AuthenticatedUser {
            properties: Option<UserProperties>,
        }
        #[derive(Deserialize)]
        struct UserProperties {
            account: Option<AccountValue>,
        }
        #[derive(Deserialize)]
        struct AccountValue {
            value: Option<String>,
        }
        let data: ConnectionData = resp.json().await.map_err(HttpError::Request)?;
        let login = data
            .authenticated_user
            .and_then(|u| u.properties)
            .and_then(|p| p.account)
            .and_then(|a| a.value)
            .unwrap_or_else(|| "azure-user".to_owned());
        Ok(CurrentUser { login })
    }

    async fn get_raw_file(
        &self,
        _owner: &str,
        repo: &str,
        path: &str,
    ) -> Result<Option<RawFile>, PlatformError> {
        let url = format!(
            "{}/repositories/{}/items?path={}&api-version=7.0",
            self.api_base, repo, path
        );
        let rb = self
            .http
            .inner
            .get(&url)
            .header("Authorization", self.auth_header());
        let resp = rb.send().await.map_err(HttpError::Request)?;
        if resp.status() == reqwest::StatusCode::NOT_FOUND {
            return Ok(None);
        }
        if !resp.status().is_success() {
            return Err(PlatformError::Http(HttpError::Status {
                status: resp.status(),
                url,
            }));
        }
        let content = resp.text().await.map_err(HttpError::Request)?;
        Ok(Some(RawFile {
            path: path.to_owned(),
            content,
        }))
    }

    async fn get_file_list(
        &self,
        _owner: &str,
        repo: &str,
    ) -> Result<Vec<String>, PlatformError> {
        let url = format!(
            "{}/repositories/{}/items?recursionLevel=Full&api-version=7.0",
            self.api_base, repo
        );
        #[derive(Deserialize)]
        struct ItemList {
            value: Vec<Item>,
        }
        #[derive(Deserialize)]
        struct Item {
            path: Option<String>,
            #[serde(rename = "gitObjectType")]
            object_type: Option<String>,
        }
        let rb = self
            .http
            .inner
            .get(&url)
            .header("Authorization", self.auth_header());
        let resp = rb.send().await.map_err(HttpError::Request)?;
        if !resp.status().is_success() {
            return Err(PlatformError::Http(HttpError::Status {
                status: resp.status(),
                url,
            }));
        }
        let list: ItemList = resp.json().await.map_err(HttpError::Request)?;
        let files = list
            .value
            .into_iter()
            .filter(|i| i.object_type.as_deref() == Some("blob"))
            .filter_map(|i| i.path)
            .collect();
        Ok(files)
    }

    async fn create_pr(
        &self,
        _owner: &str,
        repo: &str,
        source_branch: &str,
        target_branch: &str,
        title: &str,
        body: &str,
    ) -> Result<Option<i64>, PlatformError> {
        create_pr(self, repo, source_branch, target_branch, title, body).await
    }

    async fn update_pr(
        &self,
        _owner: &str,
        repo: &str,
        pr_number: i64,
        title: Option<&str>,
        body: Option<&str>,
        state: Option<&str>,
    ) -> Result<(), PlatformError> {
        let url = format!(
            "{}/repositories/{}/pullrequests/{}?api-version=7.0",
            self.api_base, repo, pr_number
        );
        if title.is_none() && body.is_none() && state.is_none() {
            return Ok(());
        }
        #[derive(Serialize)]
        struct UpdatePrRequest {
            #[serde(skip_serializing_if = "Option::is_none")]
            title: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            description: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            status: Option<String>,
        }
        let request = UpdatePrRequest {
            title: title.map(|s| s.to_owned()),
            description: body.map(|s| s.to_owned()),
            status: state.map(|s| s.to_owned()),
        };
        let request_json = serde_json::to_string(&request)
            .map_err(|e| PlatformError::Unexpected(format!("JSON serialize: {e}")))?;
        let rb = self
            .http
            .inner
            .patch(&url)
            .header("Authorization", self.auth_header())
            .header("Content-Type", "application/json")
            .body(request_json);
        let resp = rb.send().await.map_err(HttpError::Request)?;
        if !resp.status().is_success() {
            return Err(PlatformError::Http(HttpError::Status {
                status: resp.status(),
                url,
            }));
        }
        Ok(())
    }

    async fn get_branch_status(
        &self,
        _owner: &str,
        _repo: &str,
        _branch: &str,
    ) -> Result<CombinedBranchStatus, PlatformError> {
        Err(PlatformError::NotSupported(
            "Azure DevOps branch status not yet implemented".to_owned(),
        ))
    }

    async fn write_file(
        &self,
        _owner: &str,
        _repo: &str,
        _path: &str,
        _content: &str,
    ) -> Result<(), PlatformError> {
        Err(PlatformError::NotSupported(
            "Azure DevOps write_file not yet implemented".to_owned(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    fn make_client(server_uri: &str) -> AzureClient {
        AzureClient::with_endpoint(server_uri, "myorg", "myproject", "test-pat").unwrap()
    }

    // Rust-specific: azure behavior test
    #[test]
    fn auth_header_uses_basic_with_pat() {
        let client = AzureClient::new("org", "proj", "mypat").unwrap();
        let auth = client.auth_header();
        assert!(auth.starts_with("Basic "));
        let encoded = &auth["Basic ".len()..];
        let decoded = base64::engine::general_purpose::STANDARD
            .decode(encoded)
            .unwrap();
        assert_eq!(String::from_utf8(decoded).unwrap(), ":mypat");
    }

    #[tokio::test]
    async fn get_current_user_returns_login() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/myorg/_apis/connectionData"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "authenticated_user": {
                    "properties": {
                        "account": {"value": "testuser@org.com"}
                    }
                }
            })))
            .mount(&server)
            .await;

        let client = AzureClient::with_endpoint(
            server.uri(),
            "myorg",
            "myproject",
            "test-pat",
        )
        .unwrap();
        let user = client.get_current_user().await.unwrap();
        assert_eq!(user.login, "testuser@org.com");
    }

    #[tokio::test]
    async fn get_current_user_unauthorized() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/myorg/_apis/connectionData"))
            .respond_with(ResponseTemplate::new(401))
            .mount(&server)
            .await;

        let client = AzureClient::with_endpoint(
            server.uri(),
            "myorg",
            "myproject",
            "bad-pat",
        )
        .unwrap();
        let err = client.get_current_user().await.unwrap_err();
        assert!(matches!(err, PlatformError::Unauthorized));
    }

    #[tokio::test]
    async fn create_pr_returns_id() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/myorg/myproject/_apis/git/repositories/myrepo/pullrequests".to_owned()))
            .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
                "pull_request_id": 99,
                "title": "Test PR",
                "status": "active"
            })))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let pr_id = create_pr(&client, "myrepo", "feature", "main", "Test PR", "body")
            .await
            .unwrap();
        assert_eq!(pr_id, Some(99));
    }

    #[tokio::test]
    async fn list_prs_returns_values() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/myorg/myproject/_apis/git/repositories/myrepo/pullrequests".to_owned()))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "value": [
                    {"pull_request_id": 1, "title": "PR 1"},
                    {"pull_request_id": 2, "title": "PR 2"}
                ],
                "count": 2
            })))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let prs = list_prs(&client, "myrepo", None).await.unwrap();
        assert_eq!(prs.len(), 2);
        assert_eq!(prs[0].pull_request_id, 1);
    }

    #[tokio::test]
    async fn merge_pr_succeeds() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/myorg/myproject/_apis/git/repositories/myrepo/pullrequests/5".to_owned()))
            .respond_with(ResponseTemplate::new(200))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        merge_pr(&client, "myrepo", 5).await.unwrap();
    }

    #[tokio::test]
    async fn add_comment_returns_thread() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/myorg/myproject/_apis/git/repositories/myrepo/pullrequests/1/threads".to_owned()))
            .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
                "id": 10,
                "comments": [{"id": 1, "content": "hello"}]
            })))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let thread = add_comment(&client, "myrepo", 1, "hello")
            .await
            .unwrap();
        assert_eq!(thread.id, Some(10));
    }

    #[tokio::test]
    async fn get_pr_returns_pr() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/myorg/myproject/_apis/git/repositories/myrepo/pullrequests/5".to_owned()))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "pull_request_id": 5,
                "title": "My PR",
                "status": "active"
            })))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let pr = get_pr(&client, "myrepo", 5).await.unwrap();
        assert_eq!(pr.pull_request_id, 5);
        assert_eq!(pr.title, "My PR");
    }

    #[tokio::test]
    async fn get_pr_not_found() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/myorg/myproject/_apis/git/repositories/myrepo/pullrequests/999".to_owned()))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let err = get_pr(&client, "myrepo", 999).await.unwrap_err();
        assert!(matches!(err, PlatformError::Unexpected(_)));
    }

    #[tokio::test]
    async fn create_pr_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/myorg/myproject/_apis/git/repositories/myrepo/pullrequests".to_owned()))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let result = create_pr(&client, "myrepo", "title", "body", "source", "target").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn list_prs_empty() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/myorg/myproject/_apis/git/repositories/myrepo/pullrequests".to_owned()))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({"value": []})))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let prs = list_prs(&client, "myrepo", None).await.unwrap();
        assert!(prs.is_empty());
    }

    #[tokio::test]
    async fn merge_pr_error() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/myorg/myproject/_apis/git/repositories/myrepo/pullrequests/5".to_owned()))
            .respond_with(ResponseTemplate::new(409))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let result = merge_pr(&client, "myrepo", 5).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn add_comment_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/myorg/myproject/_apis/git/repositories/myrepo/pullrequests/1/threads".to_owned()))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let result = add_comment(&client, "myrepo", 1, "hello").await;
        assert!(result.is_err());
    }
}
