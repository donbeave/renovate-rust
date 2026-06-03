# `lib/modules/datasource/bitbucket-server-tags/index.spec.ts`

[← `datasource/bitbucket-server-tags`](../../../../_by-module/datasource/bitbucket-server-tags.md) · [all modules](../../../../README.md)

**8/11 ported** (3 pending) · status: partial

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 12 | returns tags | ported | [`crates/renovate-core/src/datasources/bitbucket_server_tags.rs:180`](../../../../../../../crates/renovate-core/src/datasources/bitbucket_server_tags.rs#L180) |
| 66 | returns null on empty result | ported | [`crates/renovate-core/src/datasources/bitbucket_server_tags.rs:243`](../../../../../../../crates/renovate-core/src/datasources/bitbucket_server_tags.rs#L243) |
| 80 | returns null on missing registryurl | ported | [`crates/renovate-core/src/datasources/bitbucket_server_tags.rs:261`](../../../../../../../crates/renovate-core/src/datasources/bitbucket_server_tags.rs#L261) |
| 88 | handles not found | ported | [`crates/renovate-core/src/datasources/bitbucket_server_tags.rs:271`](../../../../../../../crates/renovate-core/src/datasources/bitbucket_server_tags.rs#L271) |
| 104 | returns commit hash of provided tag | ported | [`crates/renovate-core/src/datasources/bitbucket_server_tags.rs:291`](../../../../../../../crates/renovate-core/src/datasources/bitbucket_server_tags.rs#L291) |
| 124 | missing hash | ported | [`crates/renovate-core/src/datasources/bitbucket_server_tags.rs:315`](../../../../../../../crates/renovate-core/src/datasources/bitbucket_server_tags.rs#L315) |
| 146 | returns most recent commit hash | ported | [`crates/renovate-core/src/datasources/bitbucket_server_tags.rs:336`](../../../../../../../crates/renovate-core/src/datasources/bitbucket_server_tags.rs#L336) |
| 173 | no commits | ported | [`crates/renovate-core/src/datasources/bitbucket_server_tags.rs:365`](../../../../../../../crates/renovate-core/src/datasources/bitbucket_server_tags.rs#L365) |
| 195 | returns null on empty result | ported | [`crates/renovate-core/src/datasources/bitbucket_server_tags.rs:243`](../../../../../../../crates/renovate-core/src/datasources/bitbucket_server_tags.rs#L243) |
| 211 | returns null on missing registryurl | ported | [`crates/renovate-core/src/datasources/bitbucket_server_tags.rs:261`](../../../../../../../crates/renovate-core/src/datasources/bitbucket_server_tags.rs#L261) |
| 219 | handles not found | ported | [`crates/renovate-core/src/datasources/bitbucket_server_tags.rs:271`](../../../../../../../crates/renovate-core/src/datasources/bitbucket_server_tags.rs#L271) |

