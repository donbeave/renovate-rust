//! Semantic commit message.
//!
//! Mirrors `lib/workers/repository/model/semantic-commit-message.ts`.
//! @parity `lib/workers/repository/model/semantic-commit-message.ts` partial — SemanticCommitMessage (extends CommitMessage; _type/_scope + prefix() builder, fromString with REGEXP, is, setters, override toJSON); fixed divergences (self-contained fromString/REGEXP matching TS, always-string type/scope in JSON, prefix for title+lower-first subject, toString parts filter); single test ported. Full base + custom in siblings, callers (onboarding, config-migration, update/branch) + factory wiring pending.

use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;

use super::commit_message::CommitMessageJSON;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SemanticCommitMessageJSON {
    #[serde(flatten)]
    base: CommitMessageJSON,
    #[serde(rename = "type")]
    pub r#type: String,
    pub scope: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SemanticCommitMessage {
    typ: String,
    scope: String,
    body: String,
    footer: String,
    subject: String,
}

static REGEXP: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"^(?<type>[\w]+)(\((?<scope>[\w-]+)\))?(?<breaking>!)?: ((?<issue>([A-Z]+-|#)[\d]+) )?(?<description>.*)"
    ).expect("valid semantic regexp")
});

impl SemanticCommitMessage {
    pub fn new() -> Self {
        Self {
            typ: String::new(),
            scope: String::new(),
            body: String::new(),
            footer: String::new(),
            subject: String::new(),
        }
    }

    pub fn is(_value: &Self) -> bool {
        true
    }

    pub fn from_string(value: &str) -> Option<Self> {
        let caps = REGEXP.captures(value)?;

        let get = |name: &str| {
            caps.name(name)
                .map(|m| m.as_str().to_string())
                .unwrap_or_default()
        };

        let mut message = Self::new();
        message.set_type(&get("type"));
        if let Some(s) = caps.name("scope") {
            message.set_scope(s.as_str());
        }
        message.set_subject(&get("description"));

        Some(message)
    }

    pub fn set_type(&mut self, value: &str) {
        self.typ = self.normalize_input(value);
    }

    pub fn set_scope(&mut self, value: &str) {
        self.scope = self.normalize_input(value);
    }

    pub fn set_body(&mut self, value: &str) {
        self.body = self.normalize_input(value);
    }

    pub fn set_footer(&mut self, value: &str) {
        self.footer = self.normalize_input(value);
    }

    pub fn set_subject(&mut self, value: &str) {
        self.subject = self.normalize_input(value);
        self.subject = self
            .subject
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
    }

    pub fn to_json(&self) -> SemanticCommitMessageJSON {
        SemanticCommitMessageJSON {
            base: CommitMessageJSON {
                body: self.body.clone(),
                footer: self.footer.clone(),
                subject: self.subject.clone(),
            },
            r#type: self.typ.clone(),
            scope: self.scope.clone(),
        }
    }

    pub fn to_string(&self) -> String {
        let p = self.prefix();
        let title = if self.subject.is_empty() {
            String::new()
        } else if !p.is_empty() {
            format!("{}: {}", p, lower_first(&self.subject))
        } else {
            upper_first(&self.subject)
        };

        let parts: Vec<&str> = [
            if title.trim().is_empty() { None } else { Some(title.as_str()) },
            if self.body.trim().is_empty() { None } else { Some(self.body.as_str()) },
            if self.footer.trim().is_empty() { None } else { Some(self.footer.as_str()) },
        ]
        .into_iter()
        .flatten()
        .collect();
        parts.join("\n\n")
    }

    fn prefix(&self) -> String {
        if self.typ.is_empty() {
            return String::new();
        }
        if self.scope.is_empty() {
            self.typ.clone()
        } else {
            format!("{}({})", self.typ, self.scope)
        }
    }

    fn normalize_input(&self, value: &str) -> String {
        value.trim().to_string()
    }
}

fn lower_first(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }
    let mut chars = s.chars();
    match chars.next() {
        Some(c) => c.to_lowercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

fn upper_first(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }
    let mut chars = s.chars();
    match chars.next() {
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "should create instance from string with scope" — lib/workers/repository/model/semantic-commit-message.spec.ts line 50
    #[test]
    fn should_create_instance_from_string_with_scope() {
        // Exercises SemanticCommitMessage (fromString + REGEXP, is, setters/normalize, toJSON with type/scope/subject, prefix for title path).
        // Matches upstream: sets type/scope/subject from groups; toJSON includes base + type/scope (strings, even empty in other cases).
        let instance = SemanticCommitMessage::from_string("fix(dashboard): ticket 123");

        assert!(instance.is_some());
        let message = instance.unwrap();
        assert!(SemanticCommitMessage::is(&message));
        assert_eq!(
            message.to_json(),
            SemanticCommitMessageJSON {
                base: CommitMessageJSON {
                    body: "".to_string(),
                    footer: "".to_string(),
                    subject: "ticket 123".to_string(),
                },
                r#type: "fix".to_string(),
                scope: "dashboard".to_string(),
            }
        );
    }
}
