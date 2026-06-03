# `lib/modules/manager/npm/post-update/pnpm.spec.ts`

[← `manager/npm`](../../../../../_by-module/manager/npm.md) · [all modules](../../../../../README.md)

**31/31 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 48 | does nothing when no upgrades | ported | [`crates/renovate-core/src/extractors/npm_post_update/pnpm.rs:211`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/pnpm.rs#L211) |
| 55 | generates lock files | ported | [`crates/renovate-core/src/extractors/npm_post_update/pnpm.rs:217`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/pnpm.rs#L217) |
| 69 | catches errors | ported | [`crates/renovate-core/src/extractors/npm_post_update/pnpm.rs:223`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/pnpm.rs#L223) |
| 86 | finds pnpm globally | ported | [`crates/renovate-core/src/extractors/npm_post_update/pnpm.rs:229`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/pnpm.rs#L229) |
| 100 | performs lock file updates | ported | [`crates/renovate-core/src/extractors/npm_post_update/pnpm.rs:236`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/pnpm.rs#L236) |
| 120 | performs lock file updates for workspace with packages | ported | [`crates/renovate-core/src/extractors/npm_post_update/pnpm.rs:175`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/pnpm.rs#L175) |
| 146 | performs lock file updates for workspace with packages using pnpm 10.x | ported | [`crates/renovate-core/src/extractors/npm_post_update/pnpm.rs:245`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/pnpm.rs#L245) |
| 181 | performs lock file updates for non workspace using pnpm 10.x | ported | [`crates/renovate-core/src/extractors/npm_post_update/pnpm.rs:181`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/pnpm.rs#L181) |
| 210 | performs lock file updates for workspace with empty package list | ported | [`crates/renovate-core/src/extractors/npm_post_update/pnpm.rs:252`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/pnpm.rs#L252) |
| 234 | performs lock file updates for workspace with config but no package list | ported | [`crates/renovate-core/src/extractors/npm_post_update/pnpm.rs:258`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/pnpm.rs#L258) |
| 261 | performs lock file updates and install when lock file updates mixed with regular updates | ported | [`crates/renovate-core/src/extractors/npm_post_update/pnpm.rs:264`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/pnpm.rs#L264) |
| 290 | performs lock file maintenance | ported | [`crates/renovate-core/src/extractors/npm_post_update/pnpm.rs:273`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/pnpm.rs#L273) |
| 302 | performs dedupe | ported | [`crates/renovate-core/src/extractors/npm_post_update/pnpm.rs:150`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/pnpm.rs#L150) |
| 324 | uses the new version if packagemanager is updated | ported | [`crates/renovate-core/src/extractors/npm_post_update/pnpm.rs:120`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/pnpm.rs#L120) |
| 341 | uses constraint version if parent json has constraints | ported | [`crates/renovate-core/src/extractors/npm_post_update/pnpm.rs:134`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/pnpm.rs#L134) |
| 385 | uses packagemanager version and puts it into constraint | ported | [`crates/renovate-core/src/extractors/npm_post_update/pnpm.rs:140`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/pnpm.rs#L140) |
| 429 | uses volta version and puts it into constraint | ported | [`crates/renovate-core/src/extractors/npm_post_update/pnpm.rs:201`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/pnpm.rs#L201) |
| 486 | uses skips pnpm v7 if lockfileversion indicates <7 | ported | [`crates/renovate-core/src/extractors/npm_post_update/pnpm.rs:298`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/pnpm.rs#L298) |
| 502 | works for docker mode | ported | [`crates/renovate-core/src/extractors/npm_post_update/pnpm.rs:282`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/pnpm.rs#L282) |
| 539 | works for install mode | ported | [`crates/renovate-core/src/extractors/npm_post_update/pnpm.rs:187`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/pnpm.rs#L187) |
| 564 | allows pnpmfile even if ignoring scripts | ported | [`crates/renovate-core/src/extractors/npm_post_update/pnpm.rs:289`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/pnpm.rs#L289) |
| 591 | if nodemaxmemory set on global config | ported | [`crates/renovate-core/src/extractors/npm_post_update/pnpm.rs:322`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/pnpm.rs#L322) |
| 622 | if nodemaxmemory set on repo config | ported | [`crates/renovate-core/src/extractors/npm_post_update/pnpm.rs:331`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/pnpm.rs#L331) |
| 651 | returns null if no lock file | ported | [`crates/renovate-core/src/extractors/npm_post_update/pnpm.rs:304`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/pnpm.rs#L304) |
| 657 | returns null when error reading lock file | ported | [`crates/renovate-core/src/extractors/npm_post_update/pnpm.rs:310`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/pnpm.rs#L310) |
| 663 | returns null if no lockfileversion | ported | [`crates/renovate-core/src/extractors/npm_post_update/pnpm.rs:316`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/pnpm.rs#L316) |
| 669 | returns null if lockfileversion is not a number or numeric string | ported | [`crates/renovate-core/src/extractors/npm_post_update/pnpm.rs:114`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/pnpm.rs#L114) |
| 675 | returns default if lockfileversion is 1 | ported | [`crates/renovate-core/src/extractors/npm_post_update/pnpm.rs:337`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/pnpm.rs#L337) |
| 681 | maps supported versions | ported | [`crates/renovate-core/src/extractors/npm_post_update/pnpm.rs:343`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/pnpm.rs#L343) |
| 687 | maps supported versions for v6 | ported | [`crates/renovate-core/src/extractors/npm_post_update/pnpm.rs:108`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/pnpm.rs#L108) |
| 693 | maps supported versions for v9 | ported | [`crates/renovate-core/src/extractors/npm_post_update/pnpm.rs:102`](../../../../../../../../crates/renovate-core/src/extractors/npm_post_update/pnpm.rs#L102) |

