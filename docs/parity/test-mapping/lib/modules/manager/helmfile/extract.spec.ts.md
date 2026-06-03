# `lib/modules/manager/helmfile/extract.spec.ts`

[← `manager/helmfile`](../../../../_by-module/manager/helmfile.md) · [all modules](../../../../README.md)

**20/20 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 18 | skip null yaml document | ported | [`crates/renovate-core/src/extractors/helmfile.rs:559`](../../../../../../../crates/renovate-core/src/extractors/helmfile.rs#L559) |
| 31 | returns null if no releases | ported | [`crates/renovate-core/src/extractors/helmfile.rs:565`](../../../../../../../crates/renovate-core/src/extractors/helmfile.rs#L565) |
| 46 | do not crash on invalid helmfile.yaml | ported | [`crates/renovate-core/src/extractors/helmfile.rs:576`](../../../../../../../crates/renovate-core/src/extractors/helmfile.rs#L576) |
| 63 | skip if repository details are not specified | ported | [`crates/renovate-core/src/extractors/helmfile.rs:589`](../../../../../../../crates/renovate-core/src/extractors/helmfile.rs#L589) |
| 84 | skip templetized release with invalid characters | ported | [`crates/renovate-core/src/extractors/helmfile.rs:606`](../../../../../../../crates/renovate-core/src/extractors/helmfile.rs#L606) |
| 118 | skip local charts | ported | [`crates/renovate-core/src/extractors/helmfile.rs:632`](../../../../../../../crates/renovate-core/src/extractors/helmfile.rs#L632) |
| 139 | skip chart with unknown repository | ported | [`crates/renovate-core/src/extractors/helmfile.rs:647`](../../../../../../../crates/renovate-core/src/extractors/helmfile.rs#L647) |
| 160 | skip chart with special character in the name | ported | [`crates/renovate-core/src/extractors/helmfile.rs:664`](../../../../../../../crates/renovate-core/src/extractors/helmfile.rs#L664) |
| 184 | skip chart that does not have specified version | ported | [`crates/renovate-core/src/extractors/helmfile.rs:687`](../../../../../../../crates/renovate-core/src/extractors/helmfile.rs#L687) |
| 204 | parses multidoc yaml | ported | [`crates/renovate-core/src/extractors/helmfile.rs:703`](../../../../../../../crates/renovate-core/src/extractors/helmfile.rs#L703) |
| 242 | parses a chart with a go templating | ported | [`crates/renovate-core/src/extractors/helmfile.rs:767`](../../../../../../../crates/renovate-core/src/extractors/helmfile.rs#L767) |
| 280 | parses a chart with empty strings for template values | ported | [`crates/renovate-core/src/extractors/helmfile.rs:790`](../../../../../../../crates/renovate-core/src/extractors/helmfile.rs#L790) |
| 316 | parses a chart with an oci repository and non-oci one | ported | [`crates/renovate-core/src/extractors/helmfile.rs:820`](../../../../../../../crates/renovate-core/src/extractors/helmfile.rs#L820) |
| 366 | allows oci chart names containing forward slashes | ported | [`crates/renovate-core/src/extractors/helmfile.rs:865`](../../../../../../../crates/renovate-core/src/extractors/helmfile.rs#L865) |
| 392 | parses a chart with an oci repository with --- | ported | [`crates/renovate-core/src/extractors/helmfile.rs:888`](../../../../../../../crates/renovate-core/src/extractors/helmfile.rs#L888) |
| 423 | parses and replaces templating strings | ported | [`crates/renovate-core/src/extractors/helmfile.rs:910`](../../../../../../../crates/renovate-core/src/extractors/helmfile.rs#L910) |
| 477 | detects kustomize and respects relative paths | ported | [`crates/renovate-core/src/extractors/helmfile.rs:996`](../../../../../../../crates/renovate-core/src/extractors/helmfile.rs#L996) |
| 513 | makes sure url joiner works correctly | ported | [`crates/renovate-core/src/extractors/helmfile.rs:1029`](../../../../../../../crates/renovate-core/src/extractors/helmfile.rs#L1029) |
| 539 | skips helm-git repos | ported | [`crates/renovate-core/src/extractors/helmfile.rs:1048`](../../../../../../../crates/renovate-core/src/extractors/helmfile.rs#L1048) |
| 576 | parses templates key alongside releases | ported | [`crates/renovate-core/src/extractors/helmfile.rs:1212`](../../../../../../../crates/renovate-core/src/extractors/helmfile.rs#L1212) |

