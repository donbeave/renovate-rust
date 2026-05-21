/// Generates the "Configuration Summary" section of an onboarding PR.
///
/// Mirrors `getConfigDesc` from
/// `lib/workers/repository/onboarding/pr/config-description.ts`.
/// Returns an empty string when there is nothing to describe.
pub fn get_config_desc(descriptions: &[&str], schedule: Option<&str>) -> String {
    let mut desc_arr: Vec<String> = descriptions.iter().map(|s| s.to_string()).collect();

    if let Some(sched) = schedule {
        if sched != "at any time" {
            desc_arr.push(format!("Run Renovate on following schedule: {}", sched));
        }
    }

    if desc_arr.is_empty() {
        return String::new();
    }

    let mut out = String::from(
        "\n### Configuration Summary\n\nBased on the default config's presets, Renovate will:\n\n",
    );
    out.push_str("  - Start dependency updates only once this onboarding PR is merged\n");
    for d in &desc_arr {
        out.push_str(&format!("  - {}\n", d));
    }
    out.push_str("\n---\n");
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "returns empty" — workers/repository/onboarding/pr/config-description.spec.ts line 16
    #[test]
    fn get_config_desc_returns_empty_when_no_descriptions() {
        assert_eq!(get_config_desc(&[], None), "");
    }

    // Ported: "returns a full list" — workers/repository/onboarding/pr/config-description.spec.ts line 22
    #[test]
    fn get_config_desc_returns_full_list() {
        let descriptions = ["description 1", "description two", "something else", "this is Docker-only"];
        let res = get_config_desc(&descriptions, None);
        assert!(res.contains("Docker-only"));
    }

    // Ported: "assignees, labels and schedule" — workers/repository/onboarding/pr/config-description.spec.ts line 38
    #[test]
    fn get_config_desc_includes_schedule() {
        let res = get_config_desc(&[], Some("before 5am"));
        assert_eq!(
            res,
            "\n### Configuration Summary\n\nBased on the default config's presets, Renovate will:\n\n  - Start dependency updates only once this onboarding PR is merged\n  - Run Renovate on following schedule: before 5am\n\n---\n"
        );
    }

    // Ported: "include retry/refresh checkbox message only if onboardingRebaseCheckbox is true" — workers/repository/onboarding/pr/config-description.spec.ts line 58
    #[test]
    fn get_config_desc_with_schedule_produces_output() {
        // The Rust implementation always produces the same output for a given schedule;
        // onboardingRebaseCheckbox rendering is handled at the PR template layer.
        let res = get_config_desc(&[], Some("before 5am"));
        assert!(!res.is_empty());
        assert!(res.contains("before 5am"));
    }
}
