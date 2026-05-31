use crate::package_rule::{DepContext, PackageRule};

pub fn apply_package_rules<'a>(
    rules: &'a [PackageRule],
    ctx: &DepContext<'_>,
) -> Vec<&'a PackageRule> {
    rules
        .iter()
        .filter(|rule| rule.matches_context(ctx))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::package_rule::PackageRule;

    #[test]
    fn no_rules_returns_empty() {
        let ctx = DepContext::for_dep("lodash");
        let matching = apply_package_rules(&[], &ctx);
        assert!(matching.is_empty());
    }

    #[test]
    fn matching_rule_returned() {
        let rule = PackageRule {
            match_package_names: vec!["lodash".to_owned()],
            has_name_constraint: true,
            ..Default::default()
        };
        let rules = [rule];
        let ctx = DepContext::for_dep("lodash");
        let matching = apply_package_rules(&rules, &ctx);
        assert_eq!(matching.len(), 1);
    }

    #[test]
    fn non_matching_rule_excluded() {
        let rule = PackageRule {
            match_package_names: vec!["express".to_owned()],
            has_name_constraint: true,
            ..Default::default()
        };
        let rules = [rule];
        let ctx = DepContext::for_dep("lodash");
        let matching = apply_package_rules(&rules, &ctx);
        assert!(matching.is_empty());
    }
}
