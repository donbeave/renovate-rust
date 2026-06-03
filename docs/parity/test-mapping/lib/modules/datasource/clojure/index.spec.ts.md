# `lib/modules/datasource/clojure/index.spec.ts`

[← `datasource/clojure`](../../../../_by-module/datasource/clojure.md) · [all modules](../../../../README.md)

**9/9 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 93 | returns releases from custom repository | ported | [`crates/renovate-core/src/datasources/clojure.rs:176`](../../../../../../../crates/renovate-core/src/datasources/clojure.rs#L176) |
| 101 | collects releases from all registry urls | ported | [`crates/renovate-core/src/datasources/clojure.rs:204`](../../../../../../../crates/renovate-core/src/datasources/clojure.rs#L204) |
| 129 | falls back to next registry url | ported | [`crates/renovate-core/src/datasources/clojure.rs:242`](../../../../../../../crates/renovate-core/src/datasources/clojure.rs#L242) |
| 160 | ignores unsupported protocols | ported | [`crates/renovate-core/src/datasources/clojure.rs:274`](../../../../../../../crates/renovate-core/src/datasources/clojure.rs#L274) |
| 173 | skips registry with invalid metadata structure | ported | [`crates/renovate-core/src/datasources/clojure.rs:293`](../../../../../../../crates/renovate-core/src/datasources/clojure.rs#L293) |
| 192 | skips registry with invalid xml | ported | [`crates/renovate-core/src/datasources/clojure.rs:321`](../../../../../../../crates/renovate-core/src/datasources/clojure.rs#L321) |
| 208 | handles optional slash at the end of registry url | ported | [`crates/renovate-core/src/datasources/clojure.rs:348`](../../../../../../../crates/renovate-core/src/datasources/clojure.rs#L348) |
| 218 | returns null for invalid registryurls | ported | [`crates/renovate-core/src/datasources/clojure.rs:376`](../../../../../../../crates/renovate-core/src/datasources/clojure.rs#L376) |
| 227 | supports scm.url values prefixed with "scm:" | ported | [`crates/renovate-core/src/datasources/clojure.rs:389`](../../../../../../../crates/renovate-core/src/datasources/clojure.rs#L389) |

