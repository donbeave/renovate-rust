//! Errors and warnings collection.
//!
//! Mirrors `lib/workers/repository/errors-warnings.ts`.

use serde::{Deserialize, Serialize};

use crate::workers::types::RenovateConfig;
use crate::workers::types::ValidationMessage;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ErrorsWarnings {
    pub errors: Vec<ValidationMessage>,
    pub warnings: Vec<ValidationMessage>,
    pub dep_warnings: Vec<String>,
    pub dep_warning_files: Vec<String>,
}

pub fn collect_errors_warnings(config: &RenovateConfig) -> ErrorsWarnings {
    ErrorsWarnings {
        errors: Vec::new(),
        warnings: Vec::new(),
        dep_warnings: Vec::new(),
        dep_warning_files: Vec::new(),
    }
}

pub fn get_warnings_text(warnings: &[ValidationMessage]) -> String {
    if warnings.is_empty() {
        return String::new();
    }

    let mut text = format!("\n# Warnings ({})\n\n", warnings.len());
    text.push_str(
        "Please correct - or verify that you can safely ignore - \
         these warnings before you merge this PR.\n\n",
    );
    for w in warnings {
        if let (Some(topic), Some(msg)) = (&w.topic, &w.message) {
            text.push_str(&format!("-   `{}`: {}\n", topic, msg));
        }
    }
    text.push_str("\n---\n");
    text
}

pub fn get_errors_text(errors: &[ValidationMessage]) -> String {
    if errors.is_empty() {
        return String::new();
    }

    let mut text = format!("\n# Errors ({})\n\n", errors.len());
    text.push_str(
        "Renovate has found errors that you should fix (in this branch) \
         before finishing this PR.\n\n",
    );
    for e in errors {
        if let (Some(topic), Some(msg)) = (&e.topic, &e.message) {
            text.push_str(&format!("-   `{}`: {}\n", topic, msg));
        }
    }
    text.push_str("\n---\n");
    text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn errors_warnings_default() {
        let ew = ErrorsWarnings::default();
        assert!(ew.errors.is_empty());
        assert!(ew.warnings.is_empty());
        assert!(ew.dep_warnings.is_empty());
        assert!(ew.dep_warning_files.is_empty());
    }

    #[test]
    fn collect_errors_warnings_empty() {
        let config = RenovateConfig::default();
        let ew = collect_errors_warnings(&config);
        assert!(ew.errors.is_empty());
        assert!(ew.warnings.is_empty());
    }

    #[test]
    fn get_warnings_text_empty() {
        let text = get_warnings_text(&[]);
        assert!(text.is_empty());
    }

    #[test]
    fn get_warnings_text_with_messages() {
        let warnings = vec![ValidationMessage {
            topic: Some("deprecation".into()),
            message: Some("This is deprecated".into()),
        }];
        let text = get_warnings_text(&warnings);
        assert!(text.contains("# Warnings (1)"));
        assert!(text.contains("deprecation"));
    }

    #[test]
    fn get_errors_text_empty() {
        let text = get_errors_text(&[]);
        assert!(text.is_empty());
    }

    #[test]
    fn get_errors_text_with_messages() {
        let errors = vec![ValidationMessage {
            topic: Some("config".into()),
            message: Some("Invalid config".into()),
        }];
        let text = get_errors_text(&errors);
        assert!(text.contains("# Errors (1)"));
        assert!(text.contains("Invalid config"));
    }

    #[test]
    fn errors_warnings_serialization_roundtrip() {
        let ew = ErrorsWarnings {
            errors: vec![ValidationMessage {
                topic: Some("test".into()),
                message: Some("msg".into()),
            }],
            warnings: vec![],
            dep_warnings: vec!["warn1".into()],
            dep_warning_files: vec!["file1".into()],
        };
        let json = serde_json::to_string(&ew).unwrap();
        let back: ErrorsWarnings = serde_json::from_str(&json).unwrap();
        assert_eq!(back.errors.len(), 1);
        assert_eq!(back.dep_warnings.len(), 1);
    }
}
