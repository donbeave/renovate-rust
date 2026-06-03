# `lib/modules/manager/helmfile/extract.spec.ts`

[← `manager/helmfile`](../../../../_by-module/manager/helmfile.md) · [all modules](../../../../README.md)

**20/20 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 18 | skip null yaml document | ported | `crates/renovate-core/src/extractors/helmfile.rs:559` |
| 31 | returns null if no releases | ported | `crates/renovate-core/src/extractors/helmfile.rs:565` |
| 46 | do not crash on invalid helmfile.yaml | ported | `crates/renovate-core/src/extractors/helmfile.rs:576` |
| 63 | skip if repository details are not specified | ported | `crates/renovate-core/src/extractors/helmfile.rs:589` |
| 84 | skip templetized release with invalid characters | ported | `crates/renovate-core/src/extractors/helmfile.rs:606` |
| 118 | skip local charts | ported | `crates/renovate-core/src/extractors/helmfile.rs:632` |
| 139 | skip chart with unknown repository | ported | `crates/renovate-core/src/extractors/helmfile.rs:647` |
| 160 | skip chart with special character in the name | ported | `crates/renovate-core/src/extractors/helmfile.rs:664` |
| 184 | skip chart that does not have specified version | ported | `crates/renovate-core/src/extractors/helmfile.rs:687` |
| 204 | parses multidoc yaml | ported | `crates/renovate-core/src/extractors/helmfile.rs:703` |
| 242 | parses a chart with a go templating | ported | `crates/renovate-core/src/extractors/helmfile.rs:767` |
| 280 | parses a chart with empty strings for template values | ported | `crates/renovate-core/src/extractors/helmfile.rs:790` |
| 316 | parses a chart with an oci repository and non-oci one | ported | `crates/renovate-core/src/extractors/helmfile.rs:820` |
| 366 | allows oci chart names containing forward slashes | ported | `crates/renovate-core/src/extractors/helmfile.rs:865` |
| 392 | parses a chart with an oci repository with --- | ported | `crates/renovate-core/src/extractors/helmfile.rs:888` |
| 423 | parses and replaces templating strings | ported | `crates/renovate-core/src/extractors/helmfile.rs:910` |
| 477 | detects kustomize and respects relative paths | ported | `crates/renovate-core/src/extractors/helmfile.rs:996` |
| 513 | makes sure url joiner works correctly | ported | `crates/renovate-core/src/extractors/helmfile.rs:1029` |
| 539 | skips helm-git repos | ported | `crates/renovate-core/src/extractors/helmfile.rs:1048` |
| 576 | parses templates key alongside releases | ported | `crates/renovate-core/src/extractors/helmfile.rs:1212` |

