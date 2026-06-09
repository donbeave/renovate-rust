# `lib/modules/versioning/index.spec.ts`

[← `versioning/_common`](../../../_by-module/versioning/_common.md) · [all modules](../../../README.md)

**3/3 in-scope tests ported** (0 pending, 1 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 8 | should return versioning list | ported | [`crates/renovate-core/src/versioning.rs:233`](../../../../../../crates/renovate-core/src/versioning.rs#L233) |
| 12 | should fallback to semver-coerced | ported | [`crates/renovate-core/src/versioning.rs:243`](../../../../../../crates/renovate-core/src/versioning.rs#L243) |
| 18 | should accept config | ported | [`crates/renovate-core/src/versioning.rs:252`](../../../../../../crates/renovate-core/src/versioning.rs#L252) |
| 22 | matches the api contract | opt-out | large zod-based runtime contract validation (z.string().refine + dynamic import(`./${name}/index.ts`) + .pipe(z.object({isValid: z.function(), getNewValue: z.function(), ..., matches, sortVersions, ...})) over allVersioning.getVersionings() + config.allowedValues; asserts every registered versioning implements the full surface at runtime. Rust uses compile-time trait (Versioning) + static registry (no dynamic import, no zod); the individual method behaviors (is_valid, get_new_value, matches, sort etc.) are covered by the per-versioning specs (many 100% ported). Classic TS runtime reflection + schema guard with no additional business logic to port. |

