# `lib/modules/datasource/github-tags/index.spec.ts`

[← `datasource/github-tags`](../../../../_by-module/datasource/github-tags.md) · [all modules](../../../../README.md)

**11/11 in-scope tests ported** (0 pending, 1 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 25 | returns commit digest | ported | [`crates/renovate-core/src/datasources/github_tags.rs:469`](../../../../../../../crates/renovate-core/src/datasources/github_tags.rs#L469) |
| 36 | returns null for missing commit | ported | [`crates/renovate-core/src/datasources/github_tags.rs:486`](../../../../../../../crates/renovate-core/src/datasources/github_tags.rs#L486) |
| 45 | returns untagged commit digest | ported | [`crates/renovate-core/src/datasources/github_tags.rs:501`](../../../../../../../crates/renovate-core/src/datasources/github_tags.rs#L501) |
| 54 | returns tagged commit digest | ported | [`crates/renovate-core/src/datasources/github_tags.rs:518`](../../../../../../../crates/renovate-core/src/datasources/github_tags.rs#L518) |
| 73 | returns null for missing hash | opt-out | GraphQL-specific behavior: the Rust implementation uses the GitHub REST API, where /tags always includes commit.sha |
| 91 | returns null for missing tagged commit digest | ported | [`crates/renovate-core/src/datasources/github_tags.rs:536`](../../../../../../../crates/renovate-core/src/datasources/github_tags.rs#L536) |
| 110 | returns null for error | ported | [`crates/renovate-core/src/datasources/github_tags.rs:551`](../../../../../../../crates/renovate-core/src/datasources/github_tags.rs#L551) |
| 120 | returns tags | ported | [`crates/renovate-core/src/datasources/github_tags.rs:566`](../../../../../../../crates/renovate-core/src/datasources/github_tags.rs#L566) |
| 184 | if it is newer than tag timestamp | ported | [`crates/renovate-core/src/datasources/github_tags.rs:631`](../../../../../../../crates/renovate-core/src/datasources/github_tags.rs#L631) |
| 213 | keeps tag timestamp when release timestamp is older | ported | [`crates/renovate-core/src/datasources/github_tags.rs:668`](../../../../../../../crates/renovate-core/src/datasources/github_tags.rs#L668) |
| 242 | keeps tag timestamp when release timestamp is equal | ported | [`crates/renovate-core/src/datasources/github_tags.rs:705`](../../../../../../../crates/renovate-core/src/datasources/github_tags.rs#L705) |
| 271 | keeps tag timestamp when no corresponding release exists | ported | [`crates/renovate-core/src/datasources/github_tags.rs:742`](../../../../../../../crates/renovate-core/src/datasources/github_tags.rs#L742) |

