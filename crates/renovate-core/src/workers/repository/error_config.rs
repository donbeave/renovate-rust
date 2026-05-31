//! Error config handling.
//!
//! Mirrors `lib/workers/repository/error-config.ts`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ErrorConfigAction {
    #[default]
    RaiseIssue,
    UpdateOnboardingPr,
    Suppress,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ErrorConfigResult {
    pub action: ErrorConfigAction,
    pub title: String,
    pub body: String,
}

pub fn handle_config_error(
    error_message: &str,
    is_onboarding_pr_open: bool,
    suppress_notifications: bool,
) -> ErrorConfigResult {
    if suppress_notifications {
        return ErrorConfigResult {
            action: ErrorConfigAction::Suppress,
            title: String::new(),
            body: String::new(),
        };
    }

    if is_onboarding_pr_open {
        return ErrorConfigResult {
            action: ErrorConfigAction::UpdateOnboardingPr,
            title: "Action Required: Fix Renovate Configuration".to_owned(),
            body: format!(
                "There is an error with this repository's Renovate configuration \
                 that needs to be fixed. As a precaution, Renovate will stop PRs \
                 until it is resolved.\n\n{}",
                error_message
            ),
        };
    }

    ErrorConfigResult {
        action: ErrorConfigAction::RaiseIssue,
        title: "Action Required: Fix Renovate Configuration".to_owned(),
        body: format!(
            "There is an error with this repository's Renovate configuration \
             that needs to be fixed.\n\n{}",
            error_message
        ),
    }
}

pub fn raise_config_warning_issue(
    error_message: &str,
    validation_source: Option<&str>,
    validation_error: Option<&str>,
) -> String {
    let mut body = String::from(
        "There is an error with this repository's Renovate configuration \
         that needs to be fixed. As a precaution, Renovate will stop PRs \
         until it is resolved.\n\n",
    );

    if let Some(source) = validation_source {
        body.push_str(&format!("Location: `{source}`\n"));
    }
    if let Some(error) = validation_error {
        body.push_str(&format!("Error type: {error}\n"));
    }
    body.push_str(&format!("Message: {error_message}\n"));

    body
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_config_action_variants() {
        assert_ne!(ErrorConfigAction::RaiseIssue, ErrorConfigAction::Suppress);
        assert_ne!(ErrorConfigAction::UpdateOnboardingPr, ErrorConfigAction::RaiseIssue);
    }

    #[test]
    fn error_config_result_default() {
        let r = ErrorConfigResult::default();
        assert!(r.title.is_empty());
        assert!(r.body.is_empty());
    }

    #[test]
    fn handle_config_error_suppress() {
        let result = handle_config_error("test error", false, true);
        assert_eq!(result.action, ErrorConfigAction::Suppress);
    }

    #[test]
    fn handle_config_error_onboarding_pr() {
        let result = handle_config_error("test error", true, false);
        assert_eq!(result.action, ErrorConfigAction::UpdateOnboardingPr);
        assert!(result.body.contains("test error"));
    }

    #[test]
    fn handle_config_error_raise_issue() {
        let result = handle_config_error("test error", false, false);
        assert_eq!(result.action, ErrorConfigAction::RaiseIssue);
        assert!(!result.title.is_empty());
    }

    #[test]
    fn raise_config_warning_issue_basic() {
        let body = raise_config_warning_issue("bad config", None, None);
        assert!(body.contains("bad config"));
    }

    #[test]
    fn raise_config_warning_issue_with_source() {
        let body = raise_config_warning_issue(
            "bad config",
            Some("renovate.json"),
            Some("validation"),
        );
        assert!(body.contains("renovate.json"));
        assert!(body.contains("validation"));
    }

    #[test]
    fn error_config_result_serialization_roundtrip() {
        let r = ErrorConfigResult {
            action: ErrorConfigAction::RaiseIssue,
            title: "Fix Config".into(),
            body: "error body".into(),
        };
        let json = serde_json::to_string(&r).unwrap();
        let back: ErrorConfigResult = serde_json::from_str(&json).unwrap();
        assert_eq!(back.title, "Fix Config");
    }
}
