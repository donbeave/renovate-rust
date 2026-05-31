//! Tests for GitHub Release Attachments datasource.
//!
//! Tests release attachment parsing and fixtures.

use crate::datasources::github_release_attachments::{
    GithubReleaseWithAssets, ReleaseAsset,
};

fn make_release(tag: &str, assets: &[(&str, u64)]) -> GithubReleaseWithAssets {
    GithubReleaseWithAssets {
        tag_name: tag.to_owned(),
        name: Some(tag.to_owned()),
        prerelease: false,
        published_at: Some("2024-01-15T10:00:00Z".to_owned()),
        assets: assets
            .iter()
            .map(|(name, size)| ReleaseAsset {
                name: name.to_string(),
                url: format!("https://api.github.com/assets/{}", name.len()),
                size: *size,
                content_type: Some("application/octet-stream".to_owned()),
                download_count: 42,
            })
            .collect(),
    }
}

#[test]
fn release_with_assets_has_correct_tag() {
    let release = make_release("v1.0.0", &[("binary.tar.gz", 1024), ("checksums.txt", 256)]);
    assert_eq!(release.tag_name, "v1.0.0");
    assert_eq!(release.assets.len(), 2);
}

#[test]
fn release_assets_have_correct_names() {
    let release = make_release("v2.0.0", &[("linux-amd64.tar.gz", 5000)]);
    assert_eq!(release.assets[0].name, "linux-amd64.tar.gz");
    assert_eq!(release.assets[0].size, 5000);
}

#[test]
fn release_with_no_assets() {
    let release = make_release("v0.1.0", &[]);
    assert!(release.assets.is_empty());
}

#[test]
fn release_prerelease_flag() {
    let mut release = make_release("v3.0.0-beta.1", &[]);
    release.prerelease = true;
    assert!(release.prerelease);
}

#[test]
fn release_published_at_set() {
    let release = make_release("v1.0.0", &[]);
    assert_eq!(
        release.published_at.as_deref(),
        Some("2024-01-15T10:00:00Z")
    );
}

#[test]
fn release_asset_content_type() {
    let release = make_release("v1.0.0", &[("binary.tar.gz", 1024)]);
    assert_eq!(
        release.assets[0].content_type.as_deref(),
        Some("application/octet-stream")
    );
}

#[test]
fn release_asset_download_count() {
    let release = make_release("v1.0.0", &[("binary.tar.gz", 1024)]);
    assert_eq!(release.assets[0].download_count, 42);
}

#[test]
fn multiple_releases_ordering() {
    let releases = vec![
        make_release("v2.0.0", &[("bin.tar.gz", 100)]),
        make_release("v1.0.0", &[("bin.tar.gz", 50)]),
    ];
    assert_eq!(releases[0].tag_name, "v2.0.0");
    assert_eq!(releases[1].tag_name, "v1.0.0");
}

#[test]
fn release_with_multiple_asset_types() {
    let release = make_release(
        "v1.0.0",
        &[
            ("binary-linux-amd64.tar.gz", 5000),
            ("binary-darwin-amd64.tar.gz", 4500),
            ("binary-windows-amd64.zip", 4800),
            ("checksums.txt", 256),
            ("sha256sums.txt", 128),
        ],
    );
    assert_eq!(release.assets.len(), 5);
}
