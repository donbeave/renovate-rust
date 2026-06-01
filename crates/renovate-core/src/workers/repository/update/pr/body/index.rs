//! PR body generation.
//!
//! Mirrors `lib/workers/repository/update/pr/body/index.ts`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PrBodyConfig {
    pub header: Option<String>,
    pub footer: Option<String>,
    pub pr_body_columns: Option<Vec<String>>,
    pub updates: Vec<UpdateEntry>,
    pub has_release_notes: bool,
    pub dep_name: Option<String>,
    pub current_value: Option<String>,
    pub new_value: Option<String>,
    pub update_type: Option<String>,
    pub source_url: Option<String>,
    pub changelog_url: Option<String>,
    pub is_group: Option<bool>,
    pub group_name: Option<String>,
    pub controls: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateEntry {
    pub dep_name: String,
    pub dep_type: Option<String>,
    pub update_type: Option<String>,
    pub current_value: Option<String>,
    pub new_value: Option<String>,
    pub new_name: Option<String>,
}

pub fn generate_pr_body(config: &PrBodyConfig) -> String {
    let mut body = String::new();

    if let Some(ref header) = config.header
        && !header.is_empty()
    {
        body.push_str(header);
        body.push_str("\n\n");
    }

    if !config.updates.is_empty() {
        body.push_str(&generate_updates_table(
            &config.updates,
            config.pr_body_columns.as_deref(),
        ));
    }

    if config.has_release_notes {
        body.push_str("\n\n### Release Notes\n\n");
        if let Some(ref dep_name) = config.dep_name {
            body.push_str(dep_name);
            body.push('\n');
        }
    }

    if config.controls {
        body.push_str("\n\n---\n\n - [ ] <!-- rebase-check -->If you want to rebase/retry this PR, check this box\n\n");
    }

    if let Some(ref footer) = config.footer
        && !footer.is_empty()
    {
        body.push_str("\n---\n\n");
        body.push_str(footer);
    }

    body
}

fn generate_updates_table(updates: &[UpdateEntry], columns: Option<&[String]>) -> String {
    let default_columns = vec![
        "Package".to_owned(),
        "Type".to_owned(),
        "Update".to_owned(),
        "Change".to_owned(),
    ];

    let cols = columns.unwrap_or(&default_columns);

    if updates.is_empty() || cols.is_empty() {
        return String::new();
    }

    let mut table = String::from("\n\nThis PR contains the following updates:\n\n");

    table.push('|');
    for col in cols {
        table.push(' ');
        table.push_str(col);
        table.push(' ');
        table.push('|');
    }
    table.push('\n');

    table.push('|');
    for _ in cols {
        table.push_str("---|");
    }
    table.push('\n');

    for update in updates {
        table.push('|');
        for col in cols {
            let val = match col.as_str() {
                "Package" => {
                    let mut s = update.dep_name.clone();
                    if let Some(ref nn) = update.new_name
                        && nn != &update.dep_name
                    {
                        s.push_str(" → ");
                        s.push_str(nn);
                    }
                    s
                }
                "Type" => update.dep_type.clone().unwrap_or_default(),
                "Update" => update.update_type.clone().unwrap_or_default(),
                "Change" => {
                    let from = update.current_value.as_deref().unwrap_or("");
                    let to = update.new_value.as_deref().unwrap_or("");
                    if from.is_empty() && to.is_empty() {
                        String::new()
                    } else {
                        format!("`{from}` → `{to}`")
                    }
                }
                _ => String::new(),
            };
            table.push(' ');
            table.push_str(&val.replace('|', "\\|"));
            table.push(' ');
            table.push('|');
        }
        table.push('\n');
    }

    table.push('\n');
    table
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pr_body_config_default() {
        let c = PrBodyConfig::default();
        assert!(c.header.is_none());
        assert!(c.footer.is_none());
        assert!(c.updates.is_empty());
        assert!(!c.has_release_notes);
        assert!(!c.controls);
    }

    #[test]
    fn update_entry_default() {
        let e = UpdateEntry::default();
        assert!(e.dep_name.is_empty());
        assert!(e.dep_type.is_none());
    }

    #[test]
    fn generate_pr_body_empty() {
        let config = PrBodyConfig::default();
        assert!(generate_pr_body(&config).is_empty());
    }

    #[test]
    fn generate_pr_body_with_header() {
        let config = PrBodyConfig {
            header: Some("Custom Header".into()),
            ..Default::default()
        };
        let body = generate_pr_body(&config);
        assert!(body.starts_with("Custom Header"));
    }

    #[test]
    fn generate_pr_body_with_footer() {
        let config = PrBodyConfig {
            footer: Some("Custom Footer".into()),
            ..Default::default()
        };
        let body = generate_pr_body(&config);
        assert!(body.contains("Custom Footer"));
        assert!(body.contains("---"));
    }

    #[test]
    fn generate_pr_body_with_controls() {
        let config = PrBodyConfig {
            controls: true,
            ..Default::default()
        };
        let body = generate_pr_body(&config);
        assert!(body.contains("rebase-check"));
    }

    #[test]
    fn generate_pr_body_with_updates() {
        let config = PrBodyConfig {
            updates: vec![UpdateEntry {
                dep_name: "lodash".into(),
                current_value: Some("4.17.0".into()),
                new_value: Some("4.18.2".into()),
                update_type: Some("minor".into()),
                ..Default::default()
            }],
            ..Default::default()
        };
        let body = generate_pr_body(&config);
        assert!(body.contains("lodash"));
        assert!(body.contains("4.17.0"));
        assert!(body.contains("4.18.2"));
    }

    #[test]
    fn generate_pr_body_with_release_notes() {
        let config = PrBodyConfig {
            has_release_notes: true,
            dep_name: Some("lodash".into()),
            ..Default::default()
        };
        let body = generate_pr_body(&config);
        assert!(body.contains("Release Notes"));
        assert!(body.contains("lodash"));
    }

    #[test]
    fn generate_pr_body_full() {
        let config = PrBodyConfig {
            header: Some("Header".into()),
            footer: Some("Footer".into()),
            updates: vec![UpdateEntry {
                dep_name: "react".into(),
                current_value: Some("17.0.0".into()),
                new_value: Some("18.0.0".into()),
                update_type: Some("major".into()),
                ..Default::default()
            }],
            controls: true,
            ..Default::default()
        };
        let body = generate_pr_body(&config);
        assert!(body.contains("Header"));
        assert!(body.contains("react"));
        assert!(body.contains("Footer"));
        assert!(body.contains("rebase-check"));
    }

    #[test]
    fn generate_updates_table_empty() {
        let table = generate_updates_table(&[], None);
        assert!(table.is_empty());
    }

    #[test]
    fn generate_updates_table_with_rename() {
        let updates = vec![UpdateEntry {
            dep_name: "old-pkg".into(),
            new_name: Some("new-pkg".into()),
            ..Default::default()
        }];
        let table = generate_updates_table(&updates, None);
        assert!(table.contains("old-pkg → new-pkg"));
    }
}
