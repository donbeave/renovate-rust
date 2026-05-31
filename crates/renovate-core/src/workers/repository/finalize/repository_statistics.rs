//! Repository statistics collection.
//!
//! Mirrors `lib/workers/repository/finalize/repository-statistics.ts`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PrStats {
    pub total: u32,
    pub open: u32,
    pub closed: u32,
    pub merged: u32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RepositoryStatistics {
    pub pr_stats: PrStats,
    pub branch_count: usize,
    pub base_branches: Vec<String>,
    pub inactive_branches: Vec<String>,
    pub cache_modified: bool,
}

pub fn collect_statistics(
    pr_titles: &[(u64, String, String)],
    onboarding_pr_title: &str,
) -> RepositoryStatistics {
    let mut stats = RepositoryStatistics::default();

    for (_number, title, state) in pr_titles {
        if title == "Configure Renovate" || title == onboarding_pr_title {
            continue;
        }
        stats.pr_stats.total += 1;
        match state.as_str() {
            "merged" => stats.pr_stats.merged += 1,
            "closed" => stats.pr_stats.closed += 1,
            "open" => stats.pr_stats.open += 1,
            _ => {}
        }
    }

    stats
}

pub fn collect_pr_stats(pr_states: &[&str]) -> PrStats {
    let mut stats = PrStats::default();
    for state in pr_states {
        stats.total += 1;
        match *state {
            "merged" => stats.merged += 1,
            "closed" => stats.closed += 1,
            "open" => stats.open += 1,
            _ => {}
        }
    }
    stats
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pr_stats_default() {
        let s = PrStats::default();
        assert_eq!(s.total, 0);
        assert_eq!(s.open, 0);
        assert_eq!(s.closed, 0);
        assert_eq!(s.merged, 0);
    }

    #[test]
    fn repository_statistics_default() {
        let s = RepositoryStatistics::default();
        assert_eq!(s.branch_count, 0);
        assert!(s.base_branches.is_empty());
        assert!(s.inactive_branches.is_empty());
        assert!(!s.cache_modified);
    }

    #[test]
    fn collect_statistics_empty() {
        let stats = collect_statistics(&[], "Configure Renovate");
        assert_eq!(stats.pr_stats.total, 0);
    }

    #[test]
    fn collect_statistics_with_prs() {
        let prs = vec![
            (1, "Update lodash".to_owned(), "merged".to_owned()),
            (2, "Update express".to_owned(), "open".to_owned()),
            (3, "Configure Renovate".to_owned(), "closed".to_owned()),
        ];
        let stats = collect_statistics(&prs, "Configure Renovate");
        assert_eq!(stats.pr_stats.total, 2);
        assert_eq!(stats.pr_stats.merged, 1);
        assert_eq!(stats.pr_stats.open, 1);
    }

    #[test]
    fn collect_pr_stats_counts() {
        let states = vec!["open", "merged", "closed", "open"];
        let stats = collect_pr_stats(&states);
        assert_eq!(stats.total, 4);
        assert_eq!(stats.open, 2);
        assert_eq!(stats.merged, 1);
        assert_eq!(stats.closed, 1);
    }

    #[test]
    fn repository_statistics_serialization_roundtrip() {
        let s = RepositoryStatistics {
            pr_stats: PrStats {
                total: 10,
                open: 3,
                closed: 2,
                merged: 5,
            },
            branch_count: 3,
            base_branches: vec!["main".into()],
            inactive_branches: vec!["old-branch".into()],
            cache_modified: true,
        };
        let json = serde_json::to_string(&s).unwrap();
        let back: RepositoryStatistics = serde_json::from_str(&json).unwrap();
        assert_eq!(back.pr_stats.total, 10);
        assert_eq!(back.branch_count, 3);
    }
}
