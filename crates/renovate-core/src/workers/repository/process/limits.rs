//! Rate limiting logic.
//!
//! Mirrors `lib/workers/repository/process/limits.ts`.

use serde::{Deserialize, Serialize};

use crate::workers::types::RenovateConfig;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PrHourlyLimit {
    pub pr_hourly_limit: i64,
    pub prs_created_this_hour: usize,
    pub limit_reached: bool,
}

impl PrHourlyLimit {
    pub fn new(limit: i64) -> Self {
        Self {
            pr_hourly_limit: limit,
            prs_created_this_hour: 0,
            limit_reached: false,
        }
    }

    pub fn check(&mut self) -> bool {
        if self.pr_hourly_limit <= 0 {
            return false;
        }
        self.limit_reached = self.prs_created_this_hour as i64 >= self.pr_hourly_limit;
        self.limit_reached
    }

    pub fn increment(&mut self) {
        self.prs_created_this_hour += 1;
        self.check();
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CommitHourlyLimit {
    pub commit_hourly_limit: i64,
    pub commits_this_hour: usize,
    pub limit_reached: bool,
}

impl CommitHourlyLimit {
    pub fn new(limit: i64) -> Self {
        Self {
            commit_hourly_limit: limit,
            commits_this_hour: 0,
            limit_reached: false,
        }
    }

    pub fn check(&mut self) -> bool {
        if self.commit_hourly_limit <= 0 {
            return false;
        }
        self.limit_reached = self.commits_this_hour as i64 >= self.commit_hourly_limit;
        self.limit_reached
    }

    pub fn increment(&mut self) {
        self.commits_this_hour += 1;
        self.check();
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BranchConcurrentLimit {
    pub branch_concurrent_limit: i64,
    pub existing_branches: usize,
    pub limit_reached: bool,
}

impl BranchConcurrentLimit {
    pub fn new(limit: i64) -> Self {
        Self {
            branch_concurrent_limit: limit,
            existing_branches: 0,
            limit_reached: false,
        }
    }

    pub fn check(&mut self) -> bool {
        if self.branch_concurrent_limit <= 0 {
            return false;
        }
        self.limit_reached = self.existing_branches as i64 >= self.branch_concurrent_limit;
        self.limit_reached
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PrConcurrentLimit {
    pub pr_concurrent_limit: i64,
    pub open_prs: usize,
    pub limit_reached: bool,
}

impl PrConcurrentLimit {
    pub fn new(limit: i64) -> Self {
        Self {
            pr_concurrent_limit: limit,
            open_prs: 0,
            limit_reached: false,
        }
    }

    pub fn check(&mut self) -> bool {
        if self.pr_concurrent_limit <= 0 {
            return false;
        }
        self.limit_reached = self.open_prs as i64 >= self.pr_concurrent_limit;
        self.limit_reached
    }
}

pub fn check_pr_hourly_limit(config: &RenovateConfig, prs_created: usize) -> bool {
    let limit = config.pr_hourly_limit.unwrap_or(0);
    if limit <= 0 {
        return false;
    }
    prs_created as i64 >= limit
}

pub fn check_pr_concurrent_limit(config: &RenovateConfig, open_prs: usize) -> bool {
    let limit = config.pr_concurrent_limit.unwrap_or(0);
    if limit <= 0 {
        return false;
    }
    open_prs as i64 >= limit
}

pub fn check_branch_concurrent_limit(config: &RenovateConfig, existing: usize) -> bool {
    let limit = config.branch_concurrent_limit.unwrap_or(0);
    if limit <= 0 {
        return false;
    }
    existing as i64 >= limit
}

pub fn check_commit_hourly_limit(config: &RenovateConfig, commits: usize) -> bool {
    let limit = config.commit_hourly_limit.unwrap_or(0);
    if limit <= 0 {
        return false;
    }
    commits as i64 >= limit
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pr_hourly_limit_new() {
        let l = PrHourlyLimit::new(2);
        assert_eq!(l.pr_hourly_limit, 2);
        assert_eq!(l.prs_created_this_hour, 0);
        assert!(!l.limit_reached);
    }

    #[test]
    fn pr_hourly_limit_check_not_reached() {
        let mut l = PrHourlyLimit::new(2);
        l.prs_created_this_hour = 1;
        assert!(!l.check());
    }

    #[test]
    fn pr_hourly_limit_check_reached() {
        let mut l = PrHourlyLimit::new(2);
        l.prs_created_this_hour = 2;
        assert!(l.check());
    }

    #[test]
    fn pr_hourly_limit_increment() {
        let mut l = PrHourlyLimit::new(2);
        l.increment();
        assert_eq!(l.prs_created_this_hour, 1);
        assert!(!l.limit_reached);
        l.increment();
        assert_eq!(l.prs_created_this_hour, 2);
        assert!(l.limit_reached);
    }

    #[test]
    fn pr_hourly_limit_zero_means_unlimited() {
        let mut l = PrHourlyLimit::new(0);
        l.prs_created_this_hour = 100;
        assert!(!l.check());
    }

    #[test]
    fn commit_hourly_limit_new() {
        let l = CommitHourlyLimit::new(5);
        assert_eq!(l.commit_hourly_limit, 5);
    }

    #[test]
    fn commit_hourly_limit_check() {
        let mut l = CommitHourlyLimit::new(3);
        l.commits_this_hour = 3;
        assert!(l.check());
    }

    #[test]
    fn commit_hourly_limit_increment() {
        let mut l = CommitHourlyLimit::new(1);
        l.increment();
        assert!(l.limit_reached);
    }

    #[test]
    fn branch_concurrent_limit_new() {
        let l = BranchConcurrentLimit::new(10);
        assert_eq!(l.branch_concurrent_limit, 10);
    }

    #[test]
    fn branch_concurrent_limit_check() {
        let mut l = BranchConcurrentLimit::new(5);
        l.existing_branches = 5;
        assert!(l.check());
    }

    #[test]
    fn branch_concurrent_limit_not_reached() {
        let mut l = BranchConcurrentLimit::new(5);
        l.existing_branches = 3;
        assert!(!l.check());
    }

    #[test]
    fn pr_concurrent_limit_new() {
        let l = PrConcurrentLimit::new(10);
        assert_eq!(l.pr_concurrent_limit, 10);
    }

    #[test]
    fn pr_concurrent_limit_check() {
        let mut l = PrConcurrentLimit::new(5);
        l.open_prs = 5;
        assert!(l.check());
    }

    #[test]
    fn check_pr_hourly_limit_unlimited() {
        let config = RenovateConfig::default();
        assert!(!check_pr_hourly_limit(&config, 100));
    }

    #[test]
    fn check_pr_hourly_limit_not_reached() {
        let config = RenovateConfig {
            pr_hourly_limit: Some(10),
            ..Default::default()
        };
        assert!(!check_pr_hourly_limit(&config, 5));
    }

    #[test]
    fn check_pr_hourly_limit_reached() {
        let config = RenovateConfig {
            pr_hourly_limit: Some(2),
            ..Default::default()
        };
        assert!(check_pr_hourly_limit(&config, 2));
    }

    #[test]
    fn check_pr_concurrent_limit_unlimited() {
        let config = RenovateConfig::default();
        assert!(!check_pr_concurrent_limit(&config, 100));
    }

    #[test]
    fn check_pr_concurrent_limit_reached() {
        let config = RenovateConfig {
            pr_concurrent_limit: Some(3),
            ..Default::default()
        };
        assert!(check_pr_concurrent_limit(&config, 3));
    }

    #[test]
    fn check_branch_concurrent_limit_unlimited() {
        let config = RenovateConfig::default();
        assert!(!check_branch_concurrent_limit(&config, 100));
    }

    #[test]
    fn check_branch_concurrent_limit_reached() {
        let config = RenovateConfig {
            branch_concurrent_limit: Some(5),
            ..Default::default()
        };
        assert!(check_branch_concurrent_limit(&config, 5));
    }

    #[test]
    fn check_commit_hourly_limit_unlimited() {
        let config = RenovateConfig::default();
        assert!(!check_commit_hourly_limit(&config, 100));
    }

    #[test]
    fn check_commit_hourly_limit_reached() {
        let config = RenovateConfig {
            commit_hourly_limit: Some(10),
            ..Default::default()
        };
        assert!(check_commit_hourly_limit(&config, 10));
    }

    #[test]
    fn pr_hourly_limit_default() {
        let l = PrHourlyLimit::default();
        assert_eq!(l.pr_hourly_limit, 0);
    }

    #[test]
    fn pr_hourly_limit_serialization_roundtrip() {
        let l = PrHourlyLimit {
            pr_hourly_limit: 5,
            prs_created_this_hour: 3,
            limit_reached: true,
        };
        let json = serde_json::to_string(&l).unwrap();
        let back: PrHourlyLimit = serde_json::from_str(&json).unwrap();
        assert_eq!(back.pr_hourly_limit, 5);
        assert_eq!(back.prs_created_this_hour, 3);
        assert!(back.limit_reached);
    }

    #[test]
    fn commit_hourly_limit_serialization_roundtrip() {
        let l = CommitHourlyLimit {
            commit_hourly_limit: 10,
            commits_this_hour: 5,
            limit_reached: false,
        };
        let json = serde_json::to_string(&l).unwrap();
        let back: CommitHourlyLimit = serde_json::from_str(&json).unwrap();
        assert_eq!(back.commit_hourly_limit, 10);
    }
}
