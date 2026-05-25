//! sbt-package datasource utilities.
//!
//! Renovate reference: `lib/modules/datasource/sbt-package/util.ts`

use std::cmp::Ordering;

use crate::versioning::maven::compare;

/// Returns the latest version from a slice using Maven version ordering.
pub fn get_latest_version<'a>(versions: &[&'a str]) -> Option<&'a str> {
    versions.iter().copied().reduce(|best, v| {
        if compare(v, best) == Ordering::Greater {
            v
        } else {
            best
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "gets latest version" — datasource/sbt-package/util.spec.ts line 4
    #[test]
    fn gets_latest_version() {
        assert_eq!(
            get_latest_version(&["1.0.0", "3.0.0", "2.0.0"]),
            Some("3.0.0")
        );
    }
}
