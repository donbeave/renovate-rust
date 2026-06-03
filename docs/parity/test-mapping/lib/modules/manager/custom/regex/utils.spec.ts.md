# `lib/modules/manager/custom/regex/utils.spec.ts`

[← `manager/custom`](../../../../../_by-module/manager/custom.md) · [all modules](../../../../../README.md)

**7/7 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 16 | does not crash for lazy regex | ported | [`crates/renovate-core/src/managers.rs:2051`](../../../../../../../../crates/renovate-core/src/managers.rs#L2051) |
| 27 | sets registryurls when registryurl group is a valid url | ported | [`crates/renovate-core/src/repo_config.rs:15569`](../../../../../../../../crates/renovate-core/src/repo_config.rs#L15569) |
| 39 | warns and skips registryurls when registryurl group is an invalid url | ported | [`crates/renovate-core/src/repo_config.rs:15590`](../../../../../../../../crates/renovate-core/src/repo_config.rs#L15590) |
| 55 | sets datasource when datasource group is provided | ported | [`crates/renovate-core/src/repo_config.rs:15609`](../../../../../../../../crates/renovate-core/src/repo_config.rs#L15609) |
| 67 | sets indentation when indentation group is whitespace | ported | [`crates/renovate-core/src/repo_config.rs:15626`](../../../../../../../../crates/renovate-core/src/repo_config.rs#L15626) |
| 79 | sets empty indentation when indentation group is non-whitespace | ported | [`crates/renovate-core/src/repo_config.rs:15644`](../../../../../../../../crates/renovate-core/src/repo_config.rs#L15644) |
| 91 | sets depname via default branch | ported | [`crates/renovate-core/src/repo_config.rs:15662`](../../../../../../../../crates/renovate-core/src/repo_config.rs#L15662) |

