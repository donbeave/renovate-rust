use regex::Regex;
use std::sync::LazyLock;

static VERSION_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"^(?P<release>[a-zA-Z0-9.]+)(?:-(?P<prerelease>[a-zA-Z0-9.\-]+))?(?:\+(?P<build>[a-zA-Z0-9.\-]+))?$",
    )
    .unwrap()
});

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier {
    pub as_string: String,
    pub as_number: u64,
    pub is_digits_only: bool,
}

impl Identifier {
    pub fn new(value: &str) -> Result<Self, String> {
        if value.is_empty() {
            return Err("Identifier value cannot be empty.".to_owned());
        }
        let is_digits_only = value.chars().all(|c| c.is_ascii_digit());
        let as_number = if is_digits_only {
            value.parse::<u64>().unwrap_or(0)
        } else {
            0
        };
        Ok(Identifier {
            as_string: value.to_owned(),
            as_number,
            is_digits_only,
        })
    }

    pub fn equals(&self, other: &Identifier) -> bool {
        self.as_string == other.as_string
    }

    pub fn is_less_than(&self, other: &Identifier) -> bool {
        if self.is_digits_only != other.is_digits_only {
            return self.is_digits_only;
        }
        if self.as_number != other.as_number {
            return self.as_number < other.as_number;
        }
        self.as_string < other.as_string
    }
}

#[derive(Debug, Clone, Default)]
pub struct VersionPart(Vec<Identifier>);

impl VersionPart {
    pub fn create(items: &[&str]) -> Result<Self, String> {
        let mut idents = Vec::new();
        for s in items {
            idents.push(Identifier::new(s)?);
        }
        Ok(VersionPart(idents))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn as_string(&self) -> String {
        self.0
            .iter()
            .map(|i| i.as_string.as_str())
            .collect::<Vec<_>>()
            .join(".")
    }

    pub fn major(&self) -> u64 {
        self.0.first().map(|i| i.as_number).unwrap_or(0)
    }

    pub fn minor(&self) -> u64 {
        self.0.get(1).map(|i| i.as_number).unwrap_or(0)
    }

    pub fn patch(&self) -> u64 {
        self.0.get(2).map(|i| i.as_number).unwrap_or(0)
    }

    pub fn equals(&self, other: &VersionPart) -> bool {
        if self.len() != other.len() {
            return false;
        }
        self.0.iter().zip(other.0.iter()).all(|(a, b)| a.equals(b))
    }

    pub fn is_less_than(&self, other: &VersionPart) -> bool {
        if self.equals(other) {
            return false;
        }
        // Empty is greater (represents "latest")
        if self.is_empty() && !other.is_empty() {
            return false;
        }
        if other.is_empty() && !self.is_empty() {
            return true;
        }
        let shortest = self.len().min(other.len());
        for i in 0..shortest {
            let a = &self.0[i];
            let b = &other.0[i];
            if !a.equals(b) {
                return a.is_less_than(b);
            }
        }
        self.len() < other.len()
    }
}

#[derive(Debug, Clone)]
pub struct BzlmodVersion {
    pub original: String,
    pub release: VersionPart,
    pub prerelease: VersionPart,
    pub build: VersionPart,
}

impl BzlmodVersion {
    pub fn new(version: &str) -> Result<Self, String> {
        if version.is_empty() {
            return Ok(BzlmodVersion {
                original: String::new(),
                release: VersionPart::default(),
                prerelease: VersionPart::default(),
                build: VersionPart::default(),
            });
        }
        let caps = VERSION_RE
            .captures(version)
            .ok_or_else(|| format!("Invalid Bazel module version: {version}"))?;

        let release_str = &caps["release"];
        let rparts: Vec<&str> = release_str.split('.').collect();
        let release = VersionPart::create(&rparts)?;

        let prerelease = if let Some(pre) = caps.name("prerelease") {
            let pparts: Vec<&str> = pre.as_str().split('.').collect();
            VersionPart::create(&pparts)?
        } else {
            VersionPart::default()
        };

        let build = if let Some(b) = caps.name("build") {
            VersionPart::create(&[b.as_str()])?
        } else {
            VersionPart::default()
        };

        Ok(BzlmodVersion {
            original: version.to_owned(),
            release,
            prerelease,
            build,
        })
    }

    pub fn is_prerelease(&self) -> bool {
        !self.prerelease.is_empty()
    }

    pub fn equals(&self, other: &BzlmodVersion, ignore_build: bool) -> bool {
        if ignore_build {
            self.release.equals(&other.release) && self.prerelease.equals(&other.prerelease)
        } else {
            self.release.equals(&other.release)
                && self.prerelease.equals(&other.prerelease)
                && self.build.equals(&other.build)
        }
    }

    pub fn is_less_than(&self, other: &BzlmodVersion) -> bool {
        if self.release.is_less_than(&other.release) {
            return true;
        }
        if other.release.is_less_than(&self.release) {
            return false;
        }
        // releases are equal
        if self.is_prerelease() && !other.is_prerelease() {
            return true;
        }
        if !self.is_prerelease() && other.is_prerelease() {
            return false;
        }
        if self.prerelease.is_less_than(&other.prerelease) {
            return true;
        }
        false
    }

    pub fn is_greater_than(&self, other: &BzlmodVersion) -> bool {
        Self::default_compare(self, other) == 1
    }

    pub fn default_compare(a: &BzlmodVersion, b: &BzlmodVersion) -> i32 {
        if a.equals(b, true) {
            return 0;
        }
        if a.is_less_than(b) {
            return -1;
        }
        1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "when all digits" — lib/modules/versioning/bazel-module/bzlmod-version.spec.ts line 5
    #[test]
    fn identifier_when_all_digits() {
        let ident = Identifier::new("123").unwrap();
        assert_eq!(ident.as_string, "123");
        assert_eq!(ident.as_number, 123);
        assert!(ident.is_digits_only);
    }

    // Ported: "when not all digits" — lib/modules/versioning/bazel-module/bzlmod-version.spec.ts line 12
    #[test]
    fn identifier_when_not_all_digits() {
        let ident = Identifier::new("foo123").unwrap();
        assert_eq!(ident.as_string, "foo123");
        assert_eq!(ident.as_number, 0);
        assert!(!ident.is_digits_only);
    }

    // Ported: "$a equals $b" — lib/modules/versioning/bazel-module/bzlmod-version.spec.ts line 19
    #[test]
    fn identifier_equals_table() {
        let cases = [
            ("1", "1", true),
            ("1", "2", false),
            ("foo1", "1", false),
            ("a", "a", true),
            ("a", "b", false),
        ];
        for (a, b, exp) in cases {
            let ai = Identifier::new(a).unwrap();
            let bi = Identifier::new(b).unwrap();
            assert_eq!(ai.equals(&bi), exp, "{a} equals {b}");
        }
    }

    // Ported: "$a is isLessThan $b" — lib/modules/versioning/bazel-module/bzlmod-version.spec.ts line 32
    #[test]
    fn identifier_is_less_than_table() {
        let cases = [
            ("1", "1", false),
            ("1", "2", true),
            ("2", "1", false),
            ("foo1", "1", false),
            ("1", "foo1", true),
            ("a", "b", true),
            ("b", "a", false),
        ];
        for (a, b, exp) in cases {
            let ai = Identifier::new(a).unwrap();
            let bi = Identifier::new(b).unwrap();
            assert_eq!(ai.is_less_than(&bi), exp, "{a} is_less_than {b}");
        }
    }

    // Ported: "VersionPart.create(...$a}" — lib/modules/versioning/bazel-module/bzlmod-version.spec.ts line 49
    #[test]
    fn version_part_create() {
        let vp = VersionPart::create(&[]).unwrap();
        assert_eq!(vp.len(), 0);
        assert_eq!(vp.as_string(), "");

        let vp2 = VersionPart::create(&["1", "0"]).unwrap();
        assert_eq!(vp2.len(), 2);
        assert_eq!(vp2.as_string(), "1.0");
    }

    // Ported: ".asString" — lib/modules/versioning/bazel-module/bzlmod-version.spec.ts line 59
    #[test]
    fn version_part_as_string() {
        assert_eq!(VersionPart::create(&[]).unwrap().as_string(), "");
        assert_eq!(
            VersionPart::create(&["1", "2", "3"]).unwrap().as_string(),
            "1.2.3"
        );
    }

    // Ported: ".major" — lib/modules/versioning/bazel-module/bzlmod-version.spec.ts line 68
    #[test]
    fn version_part_major() {
        assert_eq!(VersionPart::create(&[]).unwrap().major(), 0);
        assert_eq!(VersionPart::create(&["2"]).unwrap().major(), 2);
        assert_eq!(VersionPart::create(&["1", "2", "3"]).unwrap().major(), 1);
    }

    // Ported: ".minor" — lib/modules/versioning/bazel-module/bzlmod-version.spec.ts line 78
    #[test]
    fn version_part_minor() {
        assert_eq!(VersionPart::create(&[]).unwrap().minor(), 0);
        assert_eq!(VersionPart::create(&["1", "2", "3"]).unwrap().minor(), 2);
    }

    // Ported: ".patch" — lib/modules/versioning/bazel-module/bzlmod-version.spec.ts line 87
    #[test]
    fn version_part_patch() {
        assert_eq!(VersionPart::create(&[]).unwrap().patch(), 0);
        assert_eq!(VersionPart::create(&["1", "2", "3"]).unwrap().patch(), 3);
    }

    // Ported: "$a equals $b" — lib/modules/versioning/bazel-module/bzlmod-version.spec.ts line 96
    #[test]
    fn version_part_equals_table() {
        let cases: Vec<(Vec<&str>, Vec<&str>, bool)> = vec![
            (vec!["1", "0"], vec!["1", "0"], true),
            (vec!["1", "0"], vec!["1", "1"], false),
            (vec!["foo1", "0"], vec!["foo1", "0"], true),
        ];
        for (a, b, exp) in cases {
            let avp = VersionPart::create(&a).unwrap();
            let bvp = VersionPart::create(&b).unwrap();
            assert_eq!(avp.equals(&bvp), exp, "{a:?} equals {b:?}");
        }
    }

    // Ported: "$a is isLessThan $b" — lib/modules/versioning/bazel-module/bzlmod-version.spec.ts line 107
    #[test]
    fn version_part_is_less_than_table() {
        let cases: Vec<(Vec<&str>, Vec<&str>, bool)> = vec![
            (vec!["1", "0"], vec!["1", "0"], false),
            (vec!["1", "0"], vec!["1", "1"], true),
            (vec!["1", "1"], vec!["1", "0"], false),
            (vec!["a"], vec!["b"], true),
            (vec![], vec!["1"], false),
            (vec!["1"], vec![], true),
            (vec!["1", "0"], vec!["2"], true),
            (vec!["2"], vec!["1", "0"], false),
            (vec!["1", "9"], vec!["2", "0"], true),
            (vec!["2", "0"], vec!["1", "9"], false),
        ];
        for (a, b, exp) in cases {
            let avp = VersionPart::create(&a).unwrap();
            let bvp = VersionPart::create(&b).unwrap();
            assert_eq!(avp.is_less_than(&bvp), exp, "{a:?} is_less_than {b:?}");
        }
    }

    // Ported: ".isEmpty" — lib/modules/versioning/bazel-module/bzlmod-version.spec.ts line 125
    #[test]
    fn version_part_is_empty() {
        assert!(VersionPart::create(&[]).unwrap().is_empty());
        assert!(!VersionPart::create(&["1"]).unwrap().is_empty());
        assert!(!VersionPart::create(&["1", "0"]).unwrap().is_empty());
    }

    // Ported: "constructor($v)" — lib/modules/versioning/bazel-module/bzlmod-version.spec.ts line 137
    #[test]
    fn bzlmod_version_constructor() {
        let cases = [
            ("1.2.3", "1.2.3", "", ""),
            ("", "", "", ""),
            ("1.2.3-pre.20230417.1", "1.2.3", "pre.20230417.1", ""),
            ("1.2.3+build5", "1.2.3", "", "build5"),
            (
                "1.2.3-pre.20230417.1+build5",
                "1.2.3",
                "pre.20230417.1",
                "build5",
            ),
        ];
        for (v, rexp, pexp, bexp) in cases {
            let bv = BzlmodVersion::new(v).unwrap();
            assert_eq!(bv.release.as_string(), rexp, "release for {v}");
            assert_eq!(bv.prerelease.as_string(), pexp, "prerelease for {v}");
            assert_eq!(bv.build.as_string(), bexp, "build for {v}");
        }
    }

    // Ported: "bad versions $a" — lib/modules/versioning/bazel-module/bzlmod-version.spec.ts line 153
    #[test]
    fn bzlmod_version_bad_versions() {
        let bad = [
            "-abc",
            "-1_2",
            "ßážëł",
            "1.0-pre?",
            "1.0-pre///",
            "1..0",
            "1.0-pre..erp",
        ];
        for v in bad {
            assert!(BzlmodVersion::new(v).is_err(), "{v} should be invalid");
        }
    }

    // Ported: "$a equals $b" — lib/modules/versioning/bazel-module/bzlmod-version.spec.ts line 168
    #[test]
    fn bzlmod_version_equals_table() {
        let cases: Vec<(&str, &str, Option<bool>, bool)> = vec![
            ("1.2.3", "1.2.3", None, true),
            ("1.2.3", "1.2.4", None, false),
            ("1.2.3", "1.2.3-pre.20230417.1", None, false),
            ("1.2.3", "1.2.3+build5", None, false),
            ("1.2.3", "1.2.3+build5", Some(false), false),
            ("1.2.3", "1.2.3+build5", Some(true), true),
            ("1.2.3", "1.2.3-pre.20230417.1+build5", None, false),
            (
                "1.2.3-pre.20230417.1+build5",
                "1.2.3-pre.20230417.1+build5",
                None,
                true,
            ),
            (
                "1.2.3-pre.20230417.1+build4",
                "1.2.3-pre.20230417.1+build5",
                None,
                false,
            ),
            ("1.2.3", "foo1.2.3", None, false),
            ("1.2.3", "", None, false),
            ("", "", None, true),
        ];
        for (a, b, ignore_build, exp) in cases {
            let av = BzlmodVersion::new(a).unwrap();
            let bv = BzlmodVersion::new(b).unwrap();
            let ib = ignore_build.unwrap_or(false);
            assert_eq!(
                av.equals(&bv, ib),
                exp,
                "{a} equals {b} (ignore_build={ib})"
            );
        }
    }

    // Ported: "$a is isLessThan $b" — lib/modules/versioning/bazel-module/bzlmod-version.spec.ts line 188
    #[test]
    fn bzlmod_version_is_less_than_table() {
        let cases = [
            ("1.2.3", "1.2.3", false),
            ("1.2.3", "1.2.4", true),
            ("1.2.3", "1.2.3-pre.20230417.1", false),
            ("1.2.3-pre.20230417.1", "1.2.3", true),
            ("", "1.2.3", false),
            ("1.2.3", "", true),
            ("", "", false),
            (
                "1.2.3-pre.20230417.1+build5",
                "1.2.3-pre.20230417.1+build5",
                false,
            ),
            (
                "1.2.3-pre.20230417.1+build4",
                "1.2.3-pre.20230417.1+build5",
                false,
            ),
            ("4", "a", true),
            ("abc", "abd", true),
            ("pre", "pre.foo", true),
        ];
        for (a, b, exp) in cases {
            let av = BzlmodVersion::new(a).unwrap();
            let bv = BzlmodVersion::new(b).unwrap();
            assert_eq!(av.is_less_than(&bv), exp, "{a} is_less_than {b}");
        }
    }

    // Ported: "$a isGreaterThan $b" — lib/modules/versioning/bazel-module/bzlmod-version.spec.ts line 208
    #[test]
    fn bzlmod_version_is_greater_than_table() {
        let cases = [
            ("1.2.3", "1.2.3", false),
            ("1.2.3", "1.2.4", false),
            ("1.2.4", "1.2.3", true),
        ];
        for (a, b, exp) in cases {
            let av = BzlmodVersion::new(a).unwrap();
            let bv = BzlmodVersion::new(b).unwrap();
            assert_eq!(av.is_greater_than(&bv), exp, "{a} is_greater_than {b}");
        }
    }

    // Ported: "defaultCompare($a, $b)" — lib/modules/versioning/bazel-module/bzlmod-version.spec.ts line 221
    #[test]
    fn bzlmod_version_default_compare_table() {
        let cases = [
            ("1.2.3", "1.2.3", 0),
            ("1.2.3-pre.20230417.1", "1.2.3", -1),
            ("1.2.3", "1.2.3-pre.20230417.1", 1),
            ("2", "1.0", 1),
            ("", "1.0", 1),
            ("", "1.0+build", 1),
            ("", "1.0-pre", 1),
            ("", "1.0-pre+build-kek.lol", 1),
            ("2.0", "1.0", 1),
            ("2.0", "1.9", 1),
            ("11.0", "3.0", 1),
            ("1.0.1", "1.0", 1),
            ("1.0.0", "1.0", 1),
            ("1.0+build2", "1.0+build3", 0),
            ("1.0", "1.0-pre", 1),
            ("1.0", "1.0+build-notpre", 0),
            ("1.0.patch.3", "1.0", 1),
            ("1.0.patch.3", "1.0.patch.2", 1),
            ("1.0.patch.3", "1.0.patch.10", -1),
            ("1.0.patch3", "1.0.patch10", 1),
            ("4", "a", -1),
            ("abc", "abd", -1),
            ("1.0-pre", "1.0-are", 1),
            ("1.0-3", "1.0-2", 1),
            ("1.0-pre", "1.0-pre.foo", -1),
            ("1.0-pre.3", "1.0-pre.2", 1),
            ("1.0-pre.10", "1.0-pre.2", 1),
            ("1.0-pre.10a", "1.0-pre.2a", -1),
            ("1.0-pre.99", "1.0-pre.2a", -1),
            ("1.0-pre.patch.3", "1.0-pre.patch.4", -1),
            ("1.0--", "1.0----", -1),
        ];
        for (a, b, exp) in cases {
            let av = BzlmodVersion::new(a).unwrap();
            let bv = BzlmodVersion::new(b).unwrap();
            assert_eq!(
                BzlmodVersion::default_compare(&av, &bv),
                exp,
                "compare({a}, {b})"
            );
        }
    }

    #[test]
    fn is_prerelease_detects() {
        let v = BzlmodVersion::new("1.0-pre").unwrap();
        assert!(v.is_prerelease());
        let v2 = BzlmodVersion::new("1.0").unwrap();
        assert!(!v2.is_prerelease());
    }
}
