//! CDN URL dependency extractor.
//!
//! Extracts cdnjs.cloudflare.com URLs from arbitrary text files.
//!
//! Renovate reference: `lib/modules/manager/cdnurl/extract.ts`

use std::sync::LazyLock;

use regex::Regex;

static CLOUDFLARE_URL_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"//cdnjs\.cloudflare\.com/ajax/libs/(?P<depName>[^/]+?)/(?P<currentValue>[^/]+?)/(?P<asset>[-/_.a-zA-Z0-9]+)",
    )
    .unwrap()
});

/// A dependency extracted from a CDN URL.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CdnUrlDep {
    pub datasource: &'static str,
    pub dep_name: String,
    pub package_name: String,
    pub current_value: String,
}

/// Extract cdnjs dependencies from arbitrary file content.
pub fn extract_package_file(content: &str) -> Vec<CdnUrlDep> {
    let mut deps = Vec::new();
    let mut offset = 0;
    while let Some(cap) = CLOUDFLARE_URL_RE.captures(&content[offset..]) {
        let m = cap.get(0).unwrap();
        let dep_name = cap["depName"].to_owned();
        let current_value = cap["currentValue"].to_owned();
        let asset = &cap["asset"];
        let package_name = format!("{dep_name}/{asset}");
        deps.push(CdnUrlDep {
            datasource: "cdnjs",
            dep_name,
            package_name,
            current_value,
        });
        offset += m.start() + m.len();
    }
    deps
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "extractPackageFile" — cdnurl/extract.spec.ts line 5
    #[test]
    fn extract_package_file_sample() {
        let content = include_str!("../../tests/fixtures/cdnurl/sample.txt");
        let deps = extract_package_file(content);
        assert_eq!(deps.len(), 10);
        assert_eq!(deps[0].dep_name, "prop-types");
        assert_eq!(deps[0].current_value, "15.6.1");
        assert_eq!(deps[0].package_name, "prop-types/prop-types.min.js");
        assert_eq!(deps[1].dep_name, "react");
        assert_eq!(deps[1].current_value, "16.3.2");
        assert_eq!(deps[2].dep_name, "react-dom");
        assert_eq!(deps[2].current_value, "16.3.2");
        assert_eq!(deps[3].dep_name, "react-transition-group");
        assert_eq!(deps[3].current_value, "2.2.1");
        assert_eq!(deps[4].dep_name, "popper.js");
        assert_eq!(deps[4].current_value, "1.14.3");
        assert_eq!(deps[5].dep_name, "react-popper");
        assert_eq!(deps[5].current_value, "0.10.4");
        assert_eq!(deps[6].dep_name, "reactstrap");
        assert_eq!(deps[6].current_value, "7.1.0");
        assert_eq!(deps[7].dep_name, "react-router");
        assert_eq!(deps[7].current_value, "4.3.1");
        assert_eq!(deps[8].dep_name, "react-markdown");
        assert_eq!(deps[8].current_value, "4.0.6");
        assert_eq!(deps[9].dep_name, "axios");
        assert_eq!(deps[9].current_value, "0.18.0");
        for dep in &deps {
            assert_eq!(dep.datasource, "cdnjs");
        }
    }
}
