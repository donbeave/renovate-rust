//! Debian package (deb) datasource utilities.
//!
//! Mirrors `lib/modules/datasource/deb/url.ts` and
//! `lib/modules/datasource/deb/utils.ts`.

/// Construct binary component URLs from a Debian registry URL.
///
/// The `registry_url` is expected to contain:
/// - `suite` or `release` query param (the distribution codename)
/// - `components` query param (comma-separated component list)
/// - `binaryArch` query param (e.g., `amd64`)
///
/// Returns an error message when required params are missing.
///
/// Mirrors `constructComponentUrls` from `lib/modules/datasource/deb/url.ts`.
pub fn construct_component_urls(registry_url: &str) -> Result<Vec<String>, String> {
    let parsed = url::Url::parse(registry_url).map_err(|e| format!("Invalid registry URL: {e}"))?;

    // Extract required params
    let components_str = parsed
        .query_pairs()
        .find(|(k, _)| k == "components")
        .map(|(_, v)| v.into_owned())
        .ok_or_else(|| "Missing required query parameter: components".to_owned())?;

    let binary_arch = parsed
        .query_pairs()
        .find(|(k, _)| k == "binaryArch")
        .map(|(_, v)| v.into_owned())
        .ok_or_else(|| "Missing required query parameter: binaryArch".to_owned())?;

    // Get suite or release (optional params, at least one should be present for the suite)
    let suite = parsed
        .query_pairs()
        .find(|(k, _)| k == "suite")
        .or_else(|| parsed.query_pairs().find(|(k, _)| k == "release"))
        .map(|(_, v)| v.into_owned())
        .ok_or_else(|| "Missing required query parameter: suite or release".to_owned())?;

    // Build base URL without the specific query params
    let mut base = parsed;
    base.set_query(None);
    let base_str = base.as_str().trim_end_matches('/');

    // Build component URLs
    let components: Vec<&str> = components_str.split(',').collect();
    let urls = components
        .iter()
        .map(|component| format!("{base_str}/dists/{suite}/{component}/binary-{binary_arch}"))
        .collect();

    Ok(urls)
}

/// Parse SHA256 checksum for `package_path` from a Debian InRelease file.
///
/// Mirrors `parseChecksumsFromInRelease` from `lib/modules/datasource/deb/checksum.ts`.
pub fn parse_checksums_from_in_release(
    in_release_content: &str,
    package_path: &str,
) -> Option<String> {
    use regex::Regex;
    let pattern = format!(r"([a-f0-9]{{64}})\s+\d+\s+{}$", regex::escape(package_path));
    let re = Regex::new(&pattern).ok()?;
    for line in in_release_content.lines() {
        if let Some(caps) = re.captures(line) {
            return Some(caps[1].to_owned());
        }
    }
    None
}

/// Supported compression types for Debian package archive extraction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DebCompression {
    Gz,
}

impl DebCompression {
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "gz" => Some(Self::Gz),
            _ => None,
        }
    }
}

/// Validate that the compression type is supported, returning an error for
/// unsupported types.
///
/// Mirrors the compression check in `extract()` from
/// `lib/modules/datasource/deb/utils.ts`.
pub fn check_compression_supported(compression: &str) -> Result<DebCompression, String> {
    DebCompression::parse(compression)
        .ok_or_else(|| format!("Unsupported compression standard '{compression}'"))
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "constructs URLs correctly from registry URL with suite" — datasource/deb/url.spec.ts line 11
    #[test]
    fn construct_component_urls_with_suite() {
        let registry_url =
            "https://deb.debian.org/debian?suite=stable&components=main,contrib&binaryArch=amd64";
        let result = construct_component_urls(registry_url).unwrap();
        assert_eq!(
            result,
            vec![
                "https://deb.debian.org/debian/dists/stable/main/binary-amd64",
                "https://deb.debian.org/debian/dists/stable/contrib/binary-amd64",
            ]
        );
    }

    // Ported: "constructs URLs correctly from registry URL with deprecated release" — datasource/deb/url.spec.ts line 22
    #[test]
    fn construct_component_urls_with_release() {
        let registry_url = "https://deb.debian.org/debian?release=bullseye&components=main,contrib&binaryArch=amd64";
        let result = construct_component_urls(registry_url).unwrap();
        assert_eq!(
            result,
            vec![
                "https://deb.debian.org/debian/dists/bullseye/main/binary-amd64",
                "https://deb.debian.org/debian/dists/bullseye/contrib/binary-amd64",
            ]
        );
    }

    // Ported: "parses the checksum for the specified package" — datasource/deb/checksum.spec.ts line 27
    #[test]
    fn parse_checksums_finds_sha256() {
        let in_release = concat!(
            " 28dff8b86daf5731f94c272bca52d981    61020 contrib/binary-amd64/Packages.gz\n",
            " bf77b15e68c5bfd7267c76a34172021de8f10f861f41ebda7b39d1390dd4bf9a    61020 contrib/binary-amd64/Packages.gz\n",
        );
        let expected = "bf77b15e68c5bfd7267c76a34172021de8f10f861f41ebda7b39d1390dd4bf9a";
        assert_eq!(
            parse_checksums_from_in_release(in_release, "contrib/binary-amd64/Packages.gz"),
            Some(expected.to_owned())
        );
        // Non-existing path returns None
        assert_eq!(
            parse_checksums_from_in_release(in_release, "non-existing/binary-amd64/Packages.gz"),
            None
        );
    }

    // Ported: "should throw error for unsupported compression" — datasource/deb/utils.spec.ts line 29
    #[test]
    fn extract_rejects_unsupported_compression() {
        let result = check_compression_supported("xz");
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .contains("Unsupported compression standard")
        );
    }

    // Ported: "throws an error if required parameters are missing" — datasource/deb/url.spec.ts line 33
    #[test]
    fn construct_component_urls_missing_params() {
        let registry_url = "https://deb.debian.org/debian?components=main,contrib";
        let result = construct_component_urls(registry_url);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .contains("Missing required query parameter")
        );
    }
}
