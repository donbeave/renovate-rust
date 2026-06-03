# `lib/modules/datasource/jenkins-plugins/index.spec.ts`

[← `datasource/jenkins-plugins`](../../../../_by-module/datasource/jenkins-plugins.md) · [all modules](../../../../README.md)

**5/5 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 57 | returns null for a package miss | ported | [`crates/renovate-core/src/datasources/jenkins_plugins.rs:204`](../../../../../../../crates/renovate-core/src/datasources/jenkins_plugins.rs#L204) |
| 69 | returns package releases for a hit for info and releases | ported | [`crates/renovate-core/src/datasources/jenkins_plugins.rs:221`](../../../../../../../crates/renovate-core/src/datasources/jenkins_plugins.rs#L221) |
| 104 | returns package releases for a hit for info and miss for releases | ported | [`crates/renovate-core/src/datasources/jenkins_plugins.rs:268`](../../../../../../../crates/renovate-core/src/datasources/jenkins_plugins.rs#L268) |
| 122 | returns null empty response | ported | [`crates/renovate-core/src/datasources/jenkins_plugins.rs:296`](../../../../../../../crates/renovate-core/src/datasources/jenkins_plugins.rs#L296) |
| 131 | returns package releases from a custom registry | ported | [`crates/renovate-core/src/datasources/jenkins_plugins.rs:313`](../../../../../../../crates/renovate-core/src/datasources/jenkins_plugins.rs#L313) |

