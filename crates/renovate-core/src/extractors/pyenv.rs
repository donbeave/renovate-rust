//! pyenv `.python-version` dependency extractor.
//!
//! Renovate reference:
//! - `lib/modules/manager/pyenv/extract.ts`

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyenvDep {
    pub dep_name: &'static str,
    pub commit_message_topic: &'static str,
    pub current_value: String,
    pub datasource: &'static str,
}

pub fn extract(content: &str) -> PyenvDep {
    PyenvDep {
        dep_name: "python",
        commit_message_topic: "Python",
        current_value: content.trim().to_owned(),
        datasource: "docker",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "returns a result" — manager/pyenv/extract.spec.ts line 5
    #[test]
    fn returns_a_result() {
        let dep = extract("3.7.1\n");

        assert_eq!(
            dep,
            PyenvDep {
                dep_name: "python",
                commit_message_topic: "Python",
                current_value: "3.7.1".to_owned(),
                datasource: "docker",
            }
        );
    }

    // Ported: "supports ranges" — manager/pyenv/extract.spec.ts line 18
    #[test]
    fn supports_ranges() {
        let dep = extract("3.8\n");

        assert_eq!(
            dep,
            PyenvDep {
                dep_name: "python",
                commit_message_topic: "Python",
                current_value: "3.8".to_owned(),
                datasource: "docker",
            }
        );
    }

    // Ported: "skips non ranges" — manager/pyenv/extract.spec.ts line 31
    #[test]
    fn skips_non_ranges() {
        let dep = extract("latestn");

        assert_eq!(
            dep,
            PyenvDep {
                dep_name: "python",
                commit_message_topic: "Python",
                current_value: "latestn".to_owned(),
                datasource: "docker",
            }
        );
    }
}
