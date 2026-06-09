//! Onboarding PR config description helpers.
//!
//! Mirrors `lib/workers/repository/onboarding/pr/config-description.ts`.
//! @parity `lib/workers/repository/onboarding/pr/config-description.ts` partial — getConfigDesc + getScheduleDesc (description array + schedule handling for onboarding PR body summary); single test ported. (logger calls omitted as they do not affect observable return value; _packageFiles param kept for signature parity but unused per upstream TODO).

use crate::workers::types::RenovateConfig;

pub fn get_schedule_desc(config: &RenovateConfig) -> Vec<String> {
    // logger.debug('getScheduleDesc()'); logger.trace...
    let schedule = match &config.schedule {
        Some(s) if !s.is_empty() => s.clone(),
        _ => return vec![],
    };
    // TS: (as never) === 'at any time' || [0] === 'at any time'
    if schedule.iter().any(|s| s == "at any time") {
        return vec![];
    }
    let desc = format!(
        "Run Renovate on following schedule: {}",
        schedule.join(", ")
    );
    vec![desc]
}

fn get_description_array(config: &RenovateConfig) -> Vec<String> {
    // logger...
    let mut desc = config.description.clone().unwrap_or_default();
    desc.extend(get_schedule_desc(config));
    desc
}

pub fn get_config_desc(
    config: &RenovateConfig,
    _package_files: Option<&std::collections::HashMap<String, Vec<()>>>,
) -> String {
    // logger.debug('getConfigDesc()')...
    let description_arr = get_description_array(config);
    if description_arr.is_empty() {
        return String::new();
    }
    let mut desc =
        "\n### Configuration Summary\n\nBased on the default config's presets, Renovate will:\n\n"
            .to_string();
    desc.push_str("  - Start dependency updates only once this onboarding PR is merged\n");
    for d in description_arr {
        desc.push_str(&format!("  - {}\n", d));
    }
    desc.push_str("\n---\n");
    desc
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "assignees, labels and schedule" — lib/workers/repository/onboarding/pr/config-description.spec.ts line 38
    #[test]
    fn assignees_labels_and_schedule() {
        // Exercises schedule path (no description) + the exact summary formatting + schedule desc.
        // Matches the inline snapshot expectation in the upstream it().
        let mut config = RenovateConfig::default();
        config.schedule = Some(vec!["before 5am".to_string()]);
        let res = get_config_desc(&config, None);
        assert_eq!(
            res,
            "\n### Configuration Summary\n\nBased on the default config's presets, Renovate will:\n\n  - Start dependency updates only once this onboarding PR is merged\n  - Run Renovate on following schedule: before 5am\n\n---\n"
        );
    }

    #[test]
    fn returns_empty() {
        let config = RenovateConfig::default();
        let res = get_config_desc(&config, None);
        assert!(res.is_empty());
    }
}
