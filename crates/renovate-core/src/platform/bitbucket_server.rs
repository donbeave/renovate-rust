//! Bitbucket Server platform client.
//!
//! Implements [`PlatformClient`] against the Bitbucket Server REST API 1.0.
//!
//! Renovate reference: `lib/modules/platform/bitbucket-server/index.ts`.
//!
//! ## Authentication
//!
//! Bitbucket Server uses Bearer token authentication (personal access token).

use serde::{Deserialize, Serialize};

use crate::http::{HttpClient, HttpError};
use crate::platform::{CombinedBranchStatus, CurrentUser, PlatformClient, PlatformError, RawFile};

#[derive(Debug, Clone)]
pub struct BitbucketServerClient {
    http: HttpClient,
    api_base: String,
}

impl BitbucketServerClient {
    pub fn new(
        server_url: impl Into<String>,
        token: impl Into<String>,
    ) -> Result<Self, HttpError> {
        Self::with_endpoint(server_url, token)
    }

    pub fn with_endpoint(
        server_url: impl Into<String>,
        token: impl Into<String>,
    ) -> Result<Self, HttpError> {
        let base = server_url.into().trim_end_matches('/').to_owned();
        let api_base = format!("{base}/rest/api/1.0");
        Ok(Self {
            http: HttpClient::with_token(token)?,
            api_base,
        })
    }

}

#[derive(Debug, Clone, Deserialize)]
pub struct BbsUser {
    pub name: String,
    pub display_name: Option<String>,
    pub slug: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BbsRepo {
    pub id: Option<i64>,
    pub slug: Option<String>,
    pub name: Option<String>,
    pub project: Option<BbsProject>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BbsProject {
    pub key: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BbsPr {
    pub id: i64,
    pub version: Option<i64>,
    pub title: String,
    pub description: Option<String>,
    pub state: Option<String>,
    pub open: Option<bool>,
    pub closed: Option<bool>,
    pub from_ref: Option<BbsRef>,
    pub to_ref: Option<BbsRef>,
    pub author: Option<BbsPrUser>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BbsRef {
    pub id: Option<String>,
    pub display_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BbsPrUser {
    pub user: Option<BbsUser>,
    pub role: Option<String>,
    pub approved: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BbsComment {
    pub id: Option<i64>,
    pub version: Option<i64>,
    pub text: Option<String>,
    pub author: Option<BbsUser>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BbsPrPage {
    pub values: Vec<BbsPr>,
    pub is_last_page: Option<bool>,
    pub next_page_start: Option<i64>,
}

#[derive(Debug, Serialize)]
struct CreatePrRequest {
    title: String,
    description: String,
    from_ref: RefSpec,
    to_ref: RefSpec,
}

#[derive(Debug, Serialize)]
struct RefSpec {
    id: String,
}

#[derive(Debug, Serialize)]
struct CommentRequest {
    text: String,
}

pub async fn create_pr(
    client: &BitbucketServerClient,
    project: &str,
    repo: &str,
    source_branch: &str,
    target_branch: &str,
    title: &str,
    body: &str,
) -> Result<Option<i64>, PlatformError> {
    let url = format!(
        "{}/projects/{}/repos/{}/pull-requests",
        client.api_base, project, repo
    );
    let request = CreatePrRequest {
        title: title.to_owned(),
        description: body.to_owned(),
        from_ref: RefSpec {
            id: format!("refs/heads/{source_branch}"),
        },
        to_ref: RefSpec {
            id: format!("refs/heads/{target_branch}"),
        },
    };
    let request_json = serde_json::to_string(&request)
        .map_err(|e| PlatformError::Unexpected(format!("JSON serialize: {e}")))?;

    match client
        .http
        .post_json::<BbsPr>(&url, &request_json)
        .await
    {
        Ok(pr) => Ok(Some(pr.id)),
        Err(HttpError::Status { status, .. })
            if status == reqwest::StatusCode::CONFLICT
                || status == reqwest::StatusCode::BAD_REQUEST =>
        {
            Ok(None)
        }
        Err(e) => Err(PlatformError::Http(e)),
    }
}

pub async fn list_prs(
    client: &BitbucketServerClient,
    project: &str,
    repo: &str,
    state: Option<&str>,
) -> Result<Vec<BbsPr>, PlatformError> {
    let state_param = state.unwrap_or("OPEN");
    let url = format!(
        "{}/projects/{}/repos/{}/pull-requests?state={}&limit=100",
        client.api_base, project, repo, state_param
    );
    let page: BbsPrPage = client
        .http
        .get_json(&url)
        .await
        .map_err(PlatformError::Http)?;
    Ok(page.values)
}

pub async fn merge_pr(
    client: &BitbucketServerClient,
    project: &str,
    repo: &str,
    pr_id: i64,
    version: Option<i64>,
) -> Result<(), PlatformError> {
    let url = format!(
        "{}/projects/{}/repos/{}/pull-requests/{}/merge?version={}",
        client.api_base,
        project,
        repo,
        pr_id,
        version.unwrap_or(0)
    );
    #[derive(Serialize)]
    struct MergeRequest {
        version: i64,
    }
    let request = MergeRequest {
        version: version.unwrap_or(0),
    };
    let request_json = serde_json::to_string(&request)
        .map_err(|e| PlatformError::Unexpected(format!("JSON serialize: {e}")))?;

    let rb = client
        .http
        .inner
        .post(&url)
        .header("Content-Type", "application/json")
        .body(request_json);
    let rb = match &client.http.token {
        Some(t) => rb.bearer_auth(t),
        None => rb,
    };
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
    client: &BitbucketServerClient,
    project: &str,
    repo: &str,
    pr_id: i64,
    text: &str,
) -> Result<BbsComment, PlatformError> {
    let url = format!(
        "{}/projects/{}/repos/{}/pull-requests/{}/comments",
        client.api_base, project, repo, pr_id
    );
    let request = CommentRequest {
        text: text.to_owned(),
    };
    let request_json = serde_json::to_string(&request)
        .map_err(|e| PlatformError::Unexpected(format!("JSON serialize: {e}")))?;

    client
        .http
        .post_json::<BbsComment>(&url, &request_json)
        .await
        .map_err(PlatformError::Http)
}

pub async fn get_pr(
    client: &BitbucketServerClient,
    project: &str,
    repo: &str,
    pr_id: i64,
) -> Result<BbsPr, PlatformError> {
    let url = format!(
        "{}/projects/{}/repos/{}/pull-requests/{}",
        client.api_base, project, repo, pr_id
    );
    let pr: BbsPr = client
        .http
        .get_json(&url)
        .await
        .map_err(|e| match e {
            HttpError::Status { status, .. } if status == reqwest::StatusCode::NOT_FOUND => {
                PlatformError::Unexpected("PR not found".to_owned())
            }
            other => PlatformError::Http(other),
        })?;
    Ok(pr)
}

impl PlatformClient for BitbucketServerClient {
    async fn get_current_user(&self) -> Result<CurrentUser, PlatformError> {
        let user_url = format!("{}/users/current", self.api_base);
        let resp = self
            .http
            .get_retrying(&user_url)
            .await
            .map_err(PlatformError::Http)?;
        if resp.status() == reqwest::StatusCode::UNAUTHORIZED {
            return Err(PlatformError::Unauthorized);
        }
        if resp.status() == reqwest::StatusCode::NOT_FOUND {
            let admin_url = format!("{}/admin/users", self.api_base);
            let resp2 = self
                .http
                .get_retrying(&admin_url)
                .await
                .map_err(PlatformError::Http)?;
            if resp2.status() == reqwest::StatusCode::UNAUTHORIZED {
                return Err(PlatformError::Unauthorized);
            }
            return Ok(CurrentUser {
                login: "bbs-user".to_owned(),
            });
        }
        if !resp.status().is_success() {
            return Ok(CurrentUser {
                login: "bbs-user".to_owned(),
            });
        }
        let user: BbsUser = resp.json().await.map_err(HttpError::Request)?;
        Ok(CurrentUser { login: user.name })
    }

    async fn get_raw_file(
        &self,
        owner: &str,
        repo: &str,
        path: &str,
    ) -> Result<Option<RawFile>, PlatformError> {
        let url = format!(
            "{}/projects/{}/repos/{}/raw/{}?at=refs/heads/main",
            self.api_base, owner, repo, path
        );
        let resp = self
            .http
            .get_retrying(&url)
            .await
            .map_err(PlatformError::Http)?;
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
        owner: &str,
        repo: &str,
    ) -> Result<Vec<String>, PlatformError> {
        let url = format!(
            "{}/projects/{}/repos/{}/files?limit=10000",
            self.api_base, owner, repo
        );
        #[derive(Deserialize)]
        struct FileListResponse {
            values: Vec<String>,
        }
        let list: FileListResponse = self
            .http
            .get_json(&url)
            .await
            .map_err(PlatformError::Http)?;
        Ok(list.values)
    }

    async fn create_pr(
        &self,
        owner: &str,
        repo: &str,
        source_branch: &str,
        target_branch: &str,
        title: &str,
        body: &str,
    ) -> Result<Option<i64>, PlatformError> {
        create_pr(self, owner, repo, source_branch, target_branch, title, body).await
    }

    async fn update_pr(
        &self,
        owner: &str,
        repo: &str,
        pr_number: i64,
        title: Option<&str>,
        body: Option<&str>,
        state: Option<&str>,
    ) -> Result<(), PlatformError> {
        let url = format!(
            "{}/projects/{}/repos/{}/pull-requests/{}",
            self.api_base, owner, repo, pr_number
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
            state: Option<String>,
            version: i64,
        }
        let request = UpdatePrRequest {
            title: title.map(|s| s.to_owned()),
            description: body.map(|s| s.to_owned()),
            state: state.map(|s| s.to_owned()),
            version: 0,
        };
        let request_json = serde_json::to_string(&request)
            .map_err(|e| PlatformError::Unexpected(format!("JSON serialize: {e}")))?;
        let resp = self.http.put_json::<BbsPr>(&url, &request_json).await;
        match resp {
            Ok(_) => Ok(()),
            Err(e) => Err(PlatformError::Http(e)),
        }
    }

    async fn get_branch_status(
        &self,
        _owner: &str,
        _repo: &str,
        _branch: &str,
    ) -> Result<CombinedBranchStatus, PlatformError> {
        Err(PlatformError::NotSupported(
            "Bitbucket Server branch status not yet implemented".to_owned(),
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
            "Bitbucket Server write_file not yet implemented".to_owned(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    fn make_client(server_uri: &str) -> BitbucketServerClient {
        BitbucketServerClient::new(server_uri, "test-token").unwrap()
    }

    #[tokio::test]
    async fn get_current_user_returns_login() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/rest/api/1.0/users/current"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "name": "admin",
                "display_name": "Admin User",
                "slug": "admin"
            })))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let user = client.get_current_user().await.unwrap();
        assert_eq!(user.login, "admin");
    }

    #[tokio::test]
    async fn get_current_user_unauthorized() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/rest/api/1.0/users/current"))
            .respond_with(ResponseTemplate::new(401))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let err = client.get_current_user().await.unwrap_err();
        assert!(matches!(err, PlatformError::Unauthorized));
    }

    #[tokio::test]
    async fn create_pr_returns_id() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/rest/api/1.0/projects/PROJ/repos/repo/pull-requests"))
            .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
                "id": 7,
                "title": "Test",
                "version": 1,
                "from_ref": {"id": "refs/heads/feature"},
                "to_ref": {"id": "refs/heads/main"}
            })))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let pr_id = create_pr(&client, "PROJ", "repo", "feature", "main", "Test", "body")
            .await
            .unwrap();
        assert_eq!(pr_id, Some(7));
    }

    #[tokio::test]
    async fn list_prs_returns_values() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/rest/api/1.0/projects/PROJ/repos/myrepo/pull-requests"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "values": [
                    {"id": 1, "title": "PR 1"},
                    {"id": 2, "title": "PR 2"}
                ],
                "is_last_page": true
            })))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let prs = list_prs(&client, "PROJ", "myrepo", None).await.unwrap();
        assert_eq!(prs.len(), 2);
    }

    #[tokio::test]
    async fn merge_pr_succeeds() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/rest/api/1.0/projects/PROJ/repos/myrepo/pull-requests/3/merge"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        merge_pr(&client, "PROJ", "myrepo", 3, Some(1)).await.unwrap();
    }

    #[tokio::test]
    async fn add_comment_returns_comment() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/rest/api/1.0/projects/PROJ/repos/myrepo/pull-requests/1/comments"))
            .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
                "id": 42,
                "version": 1,
                "text": "hello",
                "author": {"name": "admin"}
            })))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let comment = add_comment(&client, "PROJ", "myrepo", 1, "hello")
            .await
            .unwrap();
        assert_eq!(comment.id, Some(42));
    }

    #[tokio::test]
    async fn get_pr_returns_pr() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/rest/api/1.0/projects/PROJ/repos/myrepo/pull-requests/5"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": 5,
                "title": "My PR",
                "state": "OPEN"
            })))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let pr = get_pr(&client, "PROJ", "myrepo", 5).await.unwrap();
        assert_eq!(pr.id, 5);
        assert_eq!(pr.title, "My PR");
    }

    #[tokio::test]
    async fn get_pr_not_found() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/rest/api/1.0/projects/PROJ/repos/myrepo/pull-requests/999"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let err = get_pr(&client, "PROJ", "myrepo", 999).await.unwrap_err();
        assert!(matches!(err, PlatformError::Unexpected(_)));
    }
}
