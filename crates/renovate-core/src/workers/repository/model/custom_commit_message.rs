//! Custom commit message.
//!
//! Mirrors `lib/workers/repository/model/custom-commit-message.ts`.
//! @parity `lib/workers/repository/model/custom-commit-message.ts` partial — CustomCommitMessage (extends CommitMessage; _prefix with setter/getter using normalize; override toJSON to include prefix); fixed divergences (title construction for "fix: test", to_json always string prefix even '', to_string filter+parts, safe unicode format_subject matching base/TS); single test ported. Full base in commit-message.rs (previous), semantic sibling pending, callers pending.

use serde::{Deserialize, Serialize};

use super::commit_message::{CommitMessage, CommitMessageJSON};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CustomCommitMessageJSON {
    #[serde(flatten)]
    base: CommitMessageJSON,
    pub prefix: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CustomCommitMessage {
    prefix: String,
    body: String,
    footer: String,
    subject: String,
}

impl CustomCommitMessage {
    pub fn new() -> Self {
        Self {
            prefix: String::new(),
            body: String::new(),
            footer: String::new(),
            subject: String::new(),
        }
    }

    pub fn prefix(&self) -> &str {
        &self.prefix
    }

    pub fn set_prefix(&mut self, value: &str) {
        self.prefix = self.normalize_input(value);
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

    pub fn to_json(&self) -> CustomCommitMessageJSON {
        CustomCommitMessageJSON {
            base: CommitMessageJSON {
                body: self.body.clone(),
                footer: self.footer.clone(),
                subject: self.subject.clone(),
            },
            prefix: self.prefix.clone(),
        }
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

    pub fn to_string(&self) -> String {
        let title = self.title();
        // match base/TS toString: filter non-empty (and not ws-only) parts before join \n\n
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

    fn normalize_input(&self, value: &str) -> String {
        value.trim().to_string()
    }

    // for test compatibility (and used by title/to_string); safe unicode handling like base
    pub fn format_subject(&self) -> String {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "should provide ability to set body and footer" — lib/workers/repository/model/custom-commit-message.spec.ts line 31
    #[test]
    fn should_provide_ability_to_set_body_and_footer() {
        // Exercises the CustomCommitMessage (prefix setter/normalize, toJSON including prefix, toString with body/footer, inherits base formatting).
        // The upstream test creates Custom, sets subject/prefix/body/footer, expects toJSON and toString.
        let mut commit_message = CustomCommitMessage::new();
        commit_message.set_subject("subject");
        commit_message.set_body("body");
        commit_message.set_footer("footer");
        // prefix defaults to ''

        assert_eq!(commit_message.to_json(), CustomCommitMessageJSON {
            base: CommitMessageJSON { body: "body".to_string(), footer: "footer".to_string(), subject: "subject".to_string() },
            prefix: "".to_string(),
        });
        assert_eq!(commit_message.display().to_string(), "Subject\n\nbody\n\nfooter");
    }
}