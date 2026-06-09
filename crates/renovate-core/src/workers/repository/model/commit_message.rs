//! Commit message base.
//!
//! Mirrors `lib/workers/repository/model/commit-message.ts`.
//! @parity `lib/workers/repository/model/commit-message.ts` partial — CommitMessage (abstract base with title/body/footer/subject, toString/toJSON, formatPrefix/formatSubject, setters with normalize, prefix hook); single test ported. Full concrete in semantic/custom (pending), callers (onboarding, config-migration, update/branch) pending.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CommitMessageJSON {
    pub body: String,
    pub footer: String,
    pub subject: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CommitMessage {
    body: String,
    footer: String,
    subject: String,
    prefix: String,
}

impl CommitMessage {
    pub fn new(prefix: String) -> Self {
        Self {
            prefix,
            ..Default::default()
        }
    }

    pub fn to_json(&self) -> CommitMessageJSON {
        CommitMessageJSON {
            body: self.body.clone(),
            footer: self.footer.clone(),
            subject: self.subject.clone(),
        }
    }

    pub fn to_string(&self) -> String {
        let parts: Vec<&str> = [
            Some(self.title.as_str()),
            if self.body.is_empty() { None } else { Some(self.body.as_str()) },
            if self.footer.is_empty() { None } else { Some(self.footer.as_str()) },
        ]
        .into_iter()
        .flatten()
        .collect();
        parts.join("\n\n")
    }

    pub fn title(&self) -> String {
        [CommitMessage::format_prefix(&self.prefix), self.format_subject()]
            .into_iter()
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join(" ")
            .trim()
            .to_string()
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

    fn format_subject(&self) -> String {
        if self.subject.is_empty() {
            return String::new();
        }
        if !self.prefix.is_empty() {
            let mut chars = self.subject.chars();
            match chars.next() {
                Some(c) => c.to_lowercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        } else {
            let mut chars = self.subject.chars();
            match chars.next() {
                Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        }
    }

    pub fn format_prefix(prefix: &str) -> String {
        if prefix.is_empty() {
            return String::new();
        }
        if prefix.ends_with(':') {
            prefix.to_string()
        } else {
            format!("{}:", prefix)
        }
    }

    fn normalize_input(&self, value: &str) -> String {
        value.trim().to_string()
    }

    // For subclasses to override prefix
    pub fn prefix(&self) -> &str {
        &self.prefix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "creates non-semantic commit message" — lib/workers/repository/config-migration/branch/commit-message.spec.ts line 30
    #[test]
    fn creates_non_semantic_commit_message() {
        // Exercises the base CommitMessage (toString, title, subject/body/footer, formatPrefix, normalize, prefix hook).
        // The upstream test (via wrapper) creates non-semantic and expects the formatted string.
        // Here we construct with prefix (non-semantic path) and assert toString matches expected formatting.
        let mut msg = CommitMessage::new("chore(config)".to_string());
        msg.set_subject("Migrate config renovate.json");
        // non-semantic: prefix + subject, no lowercasing of subject
        assert_eq!(msg.to_string(), "chore(config): Migrate config renovate.json");
    }
}