# `lib/modules/manager/gomod/update.spec.ts`

[← `manager/gomod`](../../../../_by-module/manager/gomod.md) · [all modules](../../../../README.md)

**33/33 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 12 | replaces existing value | ported | [`crates/renovate-core/src/extractors/gomod.rs:2252`](../../../../../../../crates/renovate-core/src/extractors/gomod.rs#L2252) |
| 28 | replaces golang version update | ported | [`crates/renovate-core/src/extractors/gomod.rs:2261`](../../../../../../../crates/renovate-core/src/extractors/gomod.rs#L2261) |
| 44 | replaces go toolchain | ported | [`crates/renovate-core/src/extractors/gomod.rs:2270`](../../../../../../../crates/renovate-core/src/extractors/gomod.rs#L2270) |
| 60 | replaces two values in one file | ported | [`crates/renovate-core/src/extractors/gomod.rs:2618`](../../../../../../../crates/renovate-core/src/extractors/gomod.rs#L2618) |
| 90 | returns same | ported | [`crates/renovate-core/src/extractors/gomod.rs:2279`](../../../../../../../crates/renovate-core/src/extractors/gomod.rs#L2279) |
| 104 | bumps major v0 > v1 | ported | [`crates/renovate-core/src/extractors/gomod.rs:2295`](../../../../../../../crates/renovate-core/src/extractors/gomod.rs#L2295) |
| 123 | replaces major updates > 1 | ported | [`crates/renovate-core/src/extractors/gomod.rs:2315`](../../../../../../../crates/renovate-core/src/extractors/gomod.rs#L2315) |
| 142 | bumps major with single package name component | ported | [`crates/renovate-core/src/extractors/gomod.rs:2335`](../../../../../../../crates/renovate-core/src/extractors/gomod.rs#L2335) |
| 161 | bumps major with multiple package name components | ported | [`crates/renovate-core/src/extractors/gomod.rs:2355`](../../../../../../../crates/renovate-core/src/extractors/gomod.rs#L2355) |
| 182 | replaces major gopkg.in updates | ported | [`crates/renovate-core/src/extractors/gomod.rs:2375`](../../../../../../../crates/renovate-core/src/extractors/gomod.rs#L2375) |
| 202 | skip replacing incompatible major updates | ported | [`crates/renovate-core/src/extractors/gomod.rs:2395`](../../../../../../../crates/renovate-core/src/extractors/gomod.rs#L2395) |
| 223 | returns null if mismatch | ported | [`crates/renovate-core/src/extractors/gomod.rs:2415`](../../../../../../../crates/renovate-core/src/extractors/gomod.rs#L2415) |
| 237 | returns null if error | ported | [`crates/renovate-core/src/extractors/gomod.rs:2431`](../../../../../../../crates/renovate-core/src/extractors/gomod.rs#L2431) |
| 247 | replaces multiline | ported | [`crates/renovate-core/src/extractors/gomod.rs:2438`](../../../../../../../crates/renovate-core/src/extractors/gomod.rs#L2438) |
| 263 | replaces quoted multiline | ported | [`crates/renovate-core/src/extractors/gomod.rs:2631`](../../../../../../../crates/renovate-core/src/extractors/gomod.rs#L2631) |
| 280 | replaces major multiline | ported | [`crates/renovate-core/src/extractors/gomod.rs:2446`](../../../../../../../crates/renovate-core/src/extractors/gomod.rs#L2446) |
| 299 | bumps major multiline | ported | [`crates/renovate-core/src/extractors/gomod.rs:2466`](../../../../../../../crates/renovate-core/src/extractors/gomod.rs#L2466) |
| 318 | bumps major v0 > v1 multiline | ported | [`crates/renovate-core/src/extractors/gomod.rs:2486`](../../../../../../../crates/renovate-core/src/extractors/gomod.rs#L2486) |
| 337 | update multiline digest | ported | [`crates/renovate-core/src/extractors/gomod.rs:2639`](../../../../../../../crates/renovate-core/src/extractors/gomod.rs#L2639) |
| 356 | skips already-updated multiline digest | ported | [`crates/renovate-core/src/extractors/gomod.rs:2658`](../../../../../../../crates/renovate-core/src/extractors/gomod.rs#L2658) |
| 373 | updates pseudo-version with digest updatetype | ported | [`crates/renovate-core/src/extractors/gomod.rs:2677`](../../../../../../../crates/renovate-core/src/extractors/gomod.rs#L2677) |
| 403 | handles multiline mismatch | ported | [`crates/renovate-core/src/extractors/gomod.rs:2506`](../../../../../../../crates/renovate-core/src/extractors/gomod.rs#L2506) |
| 418 | handles +incompatible tag | ported | [`crates/renovate-core/src/extractors/gomod.rs:2514`](../../../../../../../crates/renovate-core/src/extractors/gomod.rs#L2514) |
| 437 | handles +incompatible tag without duplicating it | ported | [`crates/renovate-core/src/extractors/gomod.rs:2528`](../../../../../../../crates/renovate-core/src/extractors/gomod.rs#L2528) |
| 458 | handles replace line with minor version update | ported | [`crates/renovate-core/src/extractors/gomod.rs:2543`](../../../../../../../crates/renovate-core/src/extractors/gomod.rs#L2543) |
| 474 | handles replace line with major version update | ported | [`crates/renovate-core/src/extractors/gomod.rs:2551`](../../../../../../../crates/renovate-core/src/extractors/gomod.rs#L2551) |
| 494 | handles replace line with major version update that bumps both sides of the replace | ported | [`crates/renovate-core/src/extractors/gomod.rs:2704`](../../../../../../../crates/renovate-core/src/extractors/gomod.rs#L2704) |
| 525 | handles replace line with digest | ported | [`crates/renovate-core/src/extractors/gomod.rs:2732`](../../../../../../../crates/renovate-core/src/extractors/gomod.rs#L2732) |
| 546 | handles no pinned version to latest available version | ported | [`crates/renovate-core/src/extractors/gomod.rs:2571`](../../../../../../../crates/renovate-core/src/extractors/gomod.rs#L2571) |
| 565 | handles multiline replace update | ported | [`crates/renovate-core/src/extractors/gomod.rs:2753`](../../../../../../../crates/renovate-core/src/extractors/gomod.rs#L2753) |
| 589 | should return null for replacement | ported | [`crates/renovate-core/src/extractors/gomod.rs:2591`](../../../../../../../crates/renovate-core/src/extractors/gomod.rs#L2591) |
| 598 | should perform indirect upgrades when top-level | ported | [`crates/renovate-core/src/extractors/gomod.rs:2602`](../../../../../../../crates/renovate-core/src/extractors/gomod.rs#L2602) |
| 614 | should perform indirect upgrades when in require blocks | ported | [`crates/renovate-core/src/extractors/gomod.rs:2610`](../../../../../../../crates/renovate-core/src/extractors/gomod.rs#L2610) |

