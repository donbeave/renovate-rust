# `lib/modules/manager/custom/regex/utils.spec.ts`

[← `manager/custom`](../../../../../_by-module/manager/custom.md) · [all modules](../../../../../README.md)

**7/7 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 16 | does not crash for lazy regex | ported | [`crates/renovate-core/src/managers.rs:2046`](../../../../../../../../crates/renovate-core/src/managers.rs#L2046) |
| 27 | sets registryurls when registryurl group is a valid url | ported | [`crates/renovate-core/src/repo_config.rs:15570`](../../../../../../../../crates/renovate-core/src/repo_config.rs#L15570) |
| 39 | warns and skips registryurls when registryurl group is an invalid url | ported | [`crates/renovate-core/src/repo_config.rs:15591`](../../../../../../../../crates/renovate-core/src/repo_config.rs#L15591) |
| 55 | sets datasource when datasource group is provided | ported | [`crates/renovate-core/src/repo_config.rs:15610`](../../../../../../../../crates/renovate-core/src/repo_config.rs#L15610) |
| 67 | sets indentation when indentation group is whitespace | ported | [`crates/renovate-core/src/repo_config.rs:15627`](../../../../../../../../crates/renovate-core/src/repo_config.rs#L15627) |
| 79 | sets empty indentation when indentation group is non-whitespace | ported | [`crates/renovate-core/src/repo_config.rs:15645`](../../../../../../../../crates/renovate-core/src/repo_config.rs#L15645) |
| 91 | sets depname via default branch | ported | [`crates/renovate-core/src/repo_config.rs:15663`](../../../../../../../../crates/renovate-core/src/repo_config.rs#L15663) |

