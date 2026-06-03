use std::collections::HashMap;
use std::sync::LazyLock;

use regex::Regex;
use sha2::{Digest, Sha256};

static RE_CHECKBOX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r" To.*?, click on a checkbox below\.").unwrap());
static RE_BRANCH_CHECKBOX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\[ ] <!-- \w*-branch.*-->").unwrap());
static RE_CONFIG_MIGRATION: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r" - \[ ] <!-- create-config-migration-pr -->.*").unwrap());
static RE_APPROVE_ALL: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r" - \[ ] <!-- approve-all-[\w-]*-prs -->.*").unwrap());
static RE_CREATE_ALL: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r" - \[ ] <!-- create-all-[\w-]*-prs -->.*").unwrap());
static RE_REBASE_ALL: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r" - \[ ] <!-- rebase-all-[\w-]*-prs -->.*").unwrap());

/// Strips all interactive checkbox elements from an issue body for platforms
/// that render it as read-only.
///
/// Mirrors `readOnlyIssueBody` from
/// `lib/modules/platform/utils/read-only-issue-body.ts`.
pub fn read_only_issue_body(body: &str) -> String {
    let s = RE_CHECKBOX.replace_all(body, "");
    let s = RE_BRANCH_CHECKBOX.replace_all(&s, "");
    let s = RE_CONFIG_MIGRATION.replace_all(&s, "");
    let s = RE_APPROVE_ALL.replace_all(&s, "");
    let s = RE_CREATE_ALL.replace_all(&s, "");
    RE_REBASE_ALL.replace_all(&s, "").into_owned()
}

const NOTE: &str =
    "> \u{2139}\u{FE0F} **Note**\n> \n> This PR body was truncated due to platform limits.\n\n";

const TRUNCATION_NOTICE: &str =
    "\n\n> \u{2702}\u{FE0F} **Note**\n> \n> PR body was truncated to here.\n";

const DIVIDER: &str = "\n\n</details>\n\n---\n\n### Configuration";

static RE_SMART: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?s)(?P<preNotes>[\s\S]*### Release Notes)(?P<releaseNotes>[\s\S]*)### Configuration(?P<postNotes>[\s\S]*)").unwrap()
});

fn char_take(s: &str, n: usize) -> String {
    s.chars().take(n).collect()
}

fn char_count(s: &str) -> usize {
    s.chars().count()
}

/// Truncates PR body text to `len` Unicode scalar values, inserting notices
/// intelligently around the Release Notes / Configuration boundary.
///
/// Mirrors `smartTruncate` from `lib/modules/platform/utils/pr-body.ts`.
pub fn smart_truncate(input: &str, len: usize) -> String {
    if char_count(input) < len {
        return input.to_owned();
    }

    let truncated_input = format!("{}{}", NOTE, input);

    let notice_len = char_count(TRUNCATION_NOTICE);

    if let Some(caps) = RE_SMART.captures(&truncated_input) {
        let pre_notes = caps.name("preNotes").map_or("", |m| m.as_str());
        let release_notes = caps.name("releaseNotes").map_or("", |m| m.as_str());
        let post_notes = caps.name("postNotes").map_or("", |m| m.as_str());

        let fixed_len =
            char_count(pre_notes) + char_count(post_notes) + char_count(DIVIDER) + notice_len;

        if fixed_len >= len {
            if notice_len >= len {
                return char_take(&truncated_input, len);
            }
            return format!(
                "{}{}",
                char_take(&truncated_input, len - notice_len),
                TRUNCATION_NOTICE
            );
        }

        let available = len - fixed_len;
        let rn_slice = char_take(release_notes, available);
        return format!(
            "{}{}{}{}{}",
            pre_notes, rn_slice, TRUNCATION_NOTICE, DIVIDER, post_notes
        );
    }

    if notice_len >= len {
        return char_take(&truncated_input, len);
    }
    format!(
        "{}{}",
        char_take(&truncated_input, len - notice_len),
        TRUNCATION_NOTICE
    )
}

// ── PR body struct (lib/modules/platform/pr-body.ts) ─────────────────────────

static PR_DEBUG_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\n?<!--renovate-debug:(?P<payload>.*?)-->\n?").unwrap());

static RENOVATE_CONFIG_HASH_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\n?<!--renovate-config-hash:(?P<payload>.*?)-->\n?").unwrap());

static PR_CHECKBOX_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"- (?P<checkbox>\[[\sx]]) <!-- rebase-check -->").unwrap());

static REVIEWABLE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\s*<!-- Reviewable:start -->").unwrap());

fn to_sha256(input: &str) -> String {
    let mut h = Sha256::new();
    h.update(input.as_bytes());
    h.finalize().iter().map(|b| format!("{b:02x}")).collect()
}

fn strip_emojis_simple(s: &str) -> String {
    static EMOJI_RE: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(
            r"[\u{2600}-\u{27FF}\u{2B00}-\u{2BFF}\u{1F000}-\u{1FAFF}\u{FE00}-\u{FE0F}\u{200D}\u{1F3FB}-\u{1F3FF}]",
        )
        .unwrap()
    });
    EMOJI_RE.replace_all(s, "").into_owned()
}

fn no_whitespace_or_headings(s: &str) -> String {
    static WS_HASH_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[\r\n\s#]").unwrap());
    WS_HASH_RE.replace_all(s, "").into_owned()
}

/// Hash a PR body for change detection.
///
/// Strips debug comments, Reviewable sections, emoji, and whitespace before
/// returning the SHA-256 hex digest.
pub fn hash_body(body: Option<&str>) -> String {
    let mut result = body.map(|s| s.trim()).unwrap_or("").to_owned();
    result = PR_DEBUG_RE.replace_all(&result, "").into_owned();
    if let Some(idx) = REVIEWABLE_RE.find(&result).map(|m| m.start()) {
        result.truncate(idx);
    }
    result = strip_emojis_simple(&result);
    result = no_whitespace_or_headings(&result);
    to_sha256(&result)
}

/// Decoded debug data attached via `<!--renovate-debug:base64-->`.
pub type DebugData = HashMap<String, serde_json::Value>;

/// Structured info extracted from a Renovate PR body.
#[derive(Debug, Clone, PartialEq)]
pub struct PrBodyStruct {
    pub hash: String,
    pub rebase_requested: Option<bool>,
    pub debug_data: Option<DebugData>,
    pub raw_config_hash: Option<String>,
}

/// Parse a PR body and extract its structural components.
pub fn get_pr_body_struct(input: Option<&str>) -> PrBodyStruct {
    let body = input.unwrap_or("");
    let hash = hash_body(input);

    let rebase_requested = PR_CHECKBOX_RE
        .captures(body)
        .and_then(|caps| caps.name("checkbox").map(|m| m.as_str() == "[x]"));

    let raw_config_hash = RENOVATE_CONFIG_HASH_RE
        .captures(body)
        .and_then(|caps| caps.name("payload").map(|m| m.as_str().to_owned()));

    let debug_data = PR_DEBUG_RE.captures(body).and_then(|caps| {
        use base64::Engine as _;
        let payload = caps.name("payload")?.as_str();
        let decoded = base64::engine::general_purpose::STANDARD
            .decode(payload)
            .ok()?;
        let json_str = String::from_utf8(decoded).ok()?;
        serde_json::from_str::<DebugData>(&json_str).ok()
    });

    PrBodyStruct {
        hash,
        rebase_requested,
        debug_data,
        raw_config_hash,
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    const PR_BODY: &str = include_str!("../../tests/fixtures/platform/pr-body.txt");
    const ISSUE_BODY: &str = include_str!("../../tests/fixtures/platform/issue-body.txt");

    // Ported: "removes all checkbox formatting" — lib/modules/platform/utils/read-only-issue-body.spec.ts line 8
    #[test]
    fn read_only_removes_checkbox_formatting() {
        assert!(!read_only_issue_body(ISSUE_BODY).contains("[ ] <!--"));
    }

    // Ported: "removes all checkbox-related instructions" — lib/modules/platform/utils/read-only-issue-body.spec.ts line 14
    #[test]
    fn read_only_removes_checkbox_instructions() {
        let result = read_only_issue_body(ISSUE_BODY);
        assert!(!result.to_lowercase().contains("click on a checkbox below"));
    }

    // Ported: "removes all approval-all-pending-prs" — lib/modules/platform/utils/read-only-issue-body.spec.ts line 20
    #[test]
    fn read_only_removes_approve_all_pending_prs() {
        assert!(
            !read_only_issue_body(ISSUE_BODY).contains("Create all pending approval PRs at once")
        );
    }

    // Ported: "removes the create-all-rate-limited-prs" — lib/modules/platform/utils/read-only-issue-body.spec.ts line 26
    #[test]
    fn read_only_removes_create_all_rate_limited_prs() {
        assert!(!read_only_issue_body(ISSUE_BODY).contains("Create all rate-limited PRs at once"));
    }

    // Ported: "removes create-config-migration-pr" — lib/modules/platform/utils/read-only-issue-body.spec.ts line 33
    #[test]
    fn read_only_removes_create_config_migration_pr() {
        assert!(
            !read_only_issue_body(ISSUE_BODY).contains("create an automated Config Migration PR")
        );
    }

    // Ported: "removes the create-all-awaiting-schedule-prs" — lib/modules/platform/utils/read-only-issue-body.spec.ts line 40
    #[test]
    fn read_only_removes_create_all_awaiting_schedule_prs() {
        assert!(
            !read_only_issue_body(ISSUE_BODY).contains("Create all awaiting schedule PRs at once")
        );
    }

    // Ported: "truncates to 1000" — lib/modules/platform/utils/pr-body.spec.ts line 9
    #[test]
    fn smart_truncate_to_1000() {
        let body = smart_truncate(PR_BODY, 1000);
        assert!(char_count(&body) < char_count(PR_BODY));
        assert!(char_count(&body) <= 1000);
    }

    // Ported: "truncates to 300 not smart" — lib/modules/platform/utils/pr-body.spec.ts line 18
    #[test]
    fn smart_truncate_to_300_not_smart() {
        let body = smart_truncate(PR_BODY, 300);
        assert_eq!(char_count(&body), 300);
    }

    // Ported: "includes truncation notice at end of truncated content (when "not smart")" — lib/modules/platform/utils/pr-body.spec.ts line 27
    #[test]
    fn smart_truncate_notice_at_end_not_smart() {
        let body = smart_truncate(PR_BODY, 300);
        assert!(body.contains("PR body was truncated to here"));
        assert_eq!(char_count(&body), 300);
    }

    // Ported: "includes truncation notice before Configuration section (when "smart")" — lib/modules/platform/utils/pr-body.spec.ts line 33
    #[test]
    fn smart_truncate_notice_before_configuration_smart() {
        let body = smart_truncate(PR_BODY, 3000);
        assert!(char_count(&body) <= 3000);
        assert!(body.contains("PR body was truncated to here"));
        assert!(body.contains("### Configuration"));
        let notice_pos = body.find("PR body was truncated to here").unwrap();
        let config_pos = body.find("### Configuration").unwrap();
        assert!(notice_pos < config_pos);
    }

    // Ported: "truncates content without release notes structure when notice fits" — lib/modules/platform/utils/pr-body.spec.ts line 43
    #[test]
    fn smart_truncate_no_release_notes_structure_notice_fits() {
        let body = smart_truncate(&"x".repeat(500), 200);
        assert_eq!(char_count(&body), 200);
        assert!(body.contains("PR body was truncated to here"));
    }

    // Ported: "truncates to below notice length with release notes structure" — lib/modules/platform/utils/pr-body.spec.ts line 49
    #[test]
    fn smart_truncate_below_notice_length() {
        let body = smart_truncate(PR_BODY, 50);
        assert_eq!(char_count(&body), 50);
        assert!(!body.contains("PR body was truncated to here"));
    }

    // Ported: "truncates to 10" — lib/modules/platform/utils/pr-body.spec.ts line 55
    #[test]
    fn smart_truncate_to_10() {
        let body = smart_truncate("Lorem ipsum dolor sit amet", 10);
        assert_eq!(body, "> \u{2139}\u{FE0F} **Not");
    }

    // Ported: "does not truncate" — lib/modules/platform/utils/pr-body.spec.ts line 63
    #[test]
    fn smart_truncate_no_truncation() {
        assert_eq!(smart_truncate(PR_BODY, 60000), PR_BODY);
    }

    const EMPTY_SHA256: &str = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

    // Ported: "returns hash for empty inputs" — lib/modules/platform/pr-body.spec.ts line 6
    #[test]
    fn pr_body_struct_empty_inputs() {
        assert_eq!(get_pr_body_struct(None).hash, EMPTY_SHA256);
        assert_eq!(get_pr_body_struct(Some("")).hash, EMPTY_SHA256);
        let with_debug = "something \n<!--renovate-debug:eyJjcmVhdGVkSW5WZXIiOiAiMS4yLjEiLCJ1cGRhdGVkSW5WZXIiOiAiMS4yLjMifQ==-->";
        let s = get_pr_body_struct(Some(with_debug));
        assert_eq!(
            s.hash,
            "3fc9b689459d738f8c88a3a48aa9e33542016b7a4052e001aaa536fca74813cb"
        );
        assert_eq!(
            s.debug_data
                .as_ref()
                .and_then(|d| d.get("createdInVer"))
                .and_then(|v| v.as_str()),
            Some("1.2.1")
        );
    }

    // Ported: "checks if we reach warning" — lib/modules/platform/pr-body.spec.ts line 29
    #[test]
    fn pr_body_struct_invalid_debug() {
        let body = "something \n<!--renovate-debug:some-wrong-data-ABCDEFGHIJKLMNOP-->";
        let s = get_pr_body_struct(Some(body));
        assert_eq!(
            s.hash,
            "3fc9b689459d738f8c88a3a48aa9e33542016b7a4052e001aaa536fca74813cb"
        );
        assert_eq!(s.debug_data, None);
    }

    // Ported: "hashes ignoring debug info" — lib/modules/platform/pr-body.spec.ts line 39
    #[test]
    fn pr_body_hash_ignores_debug() {
        assert_eq!(
            hash_body(Some("foo\n<!--renovate-debug:123-->\n")),
            hash_body(Some("foo"))
        );
    }

    // Ported: "hashes ignoring reviewable section" — lib/modules/platform/pr-body.spec.ts line 45
    #[test]
    fn pr_body_hash_ignores_reviewable() {
        assert_eq!(
            hash_body(Some("foo<!-- Reviewable:start -->bar")),
            hash_body(Some("foo"))
        );
    }

    // Ported: "hashes an undefined body" — lib/modules/platform/pr-body.spec.ts line 51
    #[test]
    fn pr_body_hash_undefined() {
        assert_eq!(hash_body(None), EMPTY_SHA256);
    }

    // Ported: "returns rebaseRequested=true flag" — lib/modules/platform/pr-body.spec.ts line 58
    #[test]
    fn pr_body_rebase_true() {
        let s = get_pr_body_struct(Some("- [x] <!-- rebase-check -->"));
        assert_eq!(s.rebase_requested, Some(true));
    }

    // Ported: "returns rebaseRequested=false flag" — lib/modules/platform/pr-body.spec.ts line 67
    #[test]
    fn pr_body_rebase_false() {
        let s = get_pr_body_struct(Some("- [ ] <!-- rebase-check -->"));
        assert_eq!(s.rebase_requested, Some(false));
    }

    // Ported: "returns rebaseRequested=undefined flag" — lib/modules/platform/pr-body.spec.ts line 76
    #[test]
    fn pr_body_rebase_none() {
        let s = get_pr_body_struct(Some("-  <!-- rebase-check -->"));
        assert_eq!(s.rebase_requested, None);
    }

    // Ported: "returns raw config hash" — lib/modules/platform/pr-body.spec.ts line 84
    #[test]
    fn pr_body_raw_config_hash() {
        use sha2::{Digest, Sha256};
        let config = "{}";
        let raw_config_hash: String = {
            let mut h = Sha256::new();
            h.update(config.as_bytes());
            h.finalize().iter().map(|b| format!("{b:02x}")).collect()
        };
        let input = format!("<!--renovate-config-hash:{raw_config_hash}-->");
        let s = get_pr_body_struct(Some(&input));
        assert_eq!(s.raw_config_hash.as_deref(), Some(raw_config_hash.as_str()));
    }

    // Ported: "strips reviewable section" — lib/modules/platform/pr-body.spec.ts line 95
    #[test]
    fn pr_body_strips_reviewable() {
        let with_r = get_pr_body_struct(Some("foo<!-- Reviewable:start -->bar"));
        let without = get_pr_body_struct(Some("foo"));
        assert_eq!(with_r.hash, without.hash);
    }
}
