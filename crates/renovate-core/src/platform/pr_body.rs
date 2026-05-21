use std::sync::LazyLock;

use regex::Regex;

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
        return input.to_string();
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

#[cfg(test)]
mod tests {
    use super::*;

    const PR_BODY: &str = include_str!("../../tests/fixtures/platform/pr-body.txt");

    // Ported: "truncates to 1000" — modules/platform/utils/pr-body.spec.ts line 9
    #[test]
    fn smart_truncate_to_1000() {
        let body = smart_truncate(PR_BODY, 1000);
        assert!(char_count(&body) < char_count(PR_BODY));
        assert!(char_count(&body) <= 1000);
    }

    // Ported: "truncates to 300 not smart" — modules/platform/utils/pr-body.spec.ts line 18
    #[test]
    fn smart_truncate_to_300_not_smart() {
        let body = smart_truncate(PR_BODY, 300);
        assert_eq!(char_count(&body), 300);
    }

    // Ported: "includes truncation notice at end of truncated content (when "not smart")" — modules/platform/utils/pr-body.spec.ts line 27
    #[test]
    fn smart_truncate_notice_at_end_not_smart() {
        let body = smart_truncate(PR_BODY, 300);
        assert!(body.contains("PR body was truncated to here"));
        assert_eq!(char_count(&body), 300);
    }

    // Ported: "includes truncation notice before Configuration section (when "smart")" — modules/platform/utils/pr-body.spec.ts line 33
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

    // Ported: "truncates content without release notes structure when notice fits" — modules/platform/utils/pr-body.spec.ts line 43
    #[test]
    fn smart_truncate_no_release_notes_structure_notice_fits() {
        let body = smart_truncate(&"x".repeat(500), 200);
        assert_eq!(char_count(&body), 200);
        assert!(body.contains("PR body was truncated to here"));
    }

    // Ported: "truncates to below notice length with release notes structure" — modules/platform/utils/pr-body.spec.ts line 49
    #[test]
    fn smart_truncate_below_notice_length() {
        let body = smart_truncate(PR_BODY, 50);
        assert_eq!(char_count(&body), 50);
        assert!(!body.contains("PR body was truncated to here"));
    }

    // Ported: "truncates to 10" — modules/platform/utils/pr-body.spec.ts line 55
    #[test]
    fn smart_truncate_to_10() {
        let body = smart_truncate("Lorem ipsum dolor sit amet", 10);
        assert_eq!(body, "> \u{2139}\u{FE0F} **Not");
    }

    // Ported: "does not truncate" — modules/platform/utils/pr-body.spec.ts line 63
    #[test]
    fn smart_truncate_no_truncation() {
        assert_eq!(smart_truncate(PR_BODY, 60000), PR_BODY);
    }
}
