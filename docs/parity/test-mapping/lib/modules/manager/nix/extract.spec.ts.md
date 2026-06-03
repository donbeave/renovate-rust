# `lib/modules/manager/nix/extract.spec.ts`

[← `manager/nix`](../../../../_by-module/manager/nix.md) · [all modules](../../../../README.md)

**38/38 ported** (0 pending) · status: ported

| Line | Test | Status | Rust destination |
|--:|---|---|---|
| 10 | returns null when no nixpkgs input exists | ported | `crates/renovate-core/src/extractors/nix.rs:458` |
| 25 | does not include nixpkgs input with no explicit ref | ported | `crates/renovate-core/src/extractors/nix.rs:467` |
| 42 | includes nixpkgs input with only ref | ported | `crates/renovate-core/src/extractors/nix.rs:478` |
| 59 | returns null when no inputs | ported | `crates/renovate-core/src/extractors/nix.rs:489` |
| 71 | returns null when inputs are missing locked | ported | `crates/renovate-core/src/extractors/nix.rs:624` |
| 95 | returns null when inputs are missing original | ported | `crates/renovate-core/src/extractors/nix.rs:652` |
| 121 | returns null when original inputs are from local path | ported | `crates/renovate-core/src/extractors/nix.rs:516` |
| 153 | returns null when locked inputs are indirect | ported | `crates/renovate-core/src/extractors/nix.rs:552` |
| 185 | returns null when locked inputs are from local path | ported | `crates/renovate-core/src/extractors/nix.rs:588` |
| 217 | returns nixpkgs input | ported | `crates/renovate-core/src/extractors/nix.rs:502` |
| 260 | includes nixpkgs with no explicit ref | ported | `crates/renovate-core/src/extractors/nix.rs:682` |
| 300 | includes patchelf from head | ported | `crates/renovate-core/src/extractors/nix.rs:726` |
| 358 | includes ijq from sourcehut without a flake | ported | `crates/renovate-core/src/extractors/nix.rs:773` |
| 399 | includes home-manager from gitlab | ported | `crates/renovate-core/src/extractors/nix.rs:818` |
| 440 | test other version | ported | `crates/renovate-core/src/extractors/nix.rs:863` |
| 452 | includes nixpkgs with ref and shallow arguments | ported | `crates/renovate-core/src/extractors/nix.rs:876` |
| 494 | includes nixpkgs but using indirect type that cannot be updated | ported | `crates/renovate-core/src/extractors/nix.rs:923` |
| 524 | includes nixpkgs but using indirect type and path locked type that cannot be updated | ported | `crates/renovate-core/src/extractors/nix.rs:957` |
| 553 | includes flake from github enterprise | ported | `crates/renovate-core/src/extractors/nix.rs:990` |
| 649 | includes flake with tarball type | ported | `crates/renovate-core/src/extractors/nix.rs:1052` |
| 750 | uri decode gitlab subgroup | ported | `crates/renovate-core/src/extractors/nix.rs:1303` |
| 790 | includes flake with only tarball type | ported | `crates/renovate-core/src/extractors/nix.rs:1140` |
| 818 | includes flake with nixpkgs-lib as tarball type | ported | `crates/renovate-core/src/extractors/nix.rs:1172` |
| 897 | includes flake with nixpkgs channel as tarball type | ported | `crates/renovate-core/src/extractors/nix.rs:1097` |
| 937 | finds currentdigest correctly when input sha is pinned | ported | `crates/renovate-core/src/extractors/nix.rs:1347` |
| 983 | does not duplicate nixpkgs dependency | ported | `crates/renovate-core/src/extractors/nix.rs:1392` |
| 1028 | returns null when flake.lock file cannot be read | ported | `crates/renovate-core/src/extractors/nix.rs:1492` |
| 1033 | returns null when flake.nix file cannot be read | ported | `crates/renovate-core/src/extractors/nix.rs:1498` |
| 1046 | returns null when flake.lock has invalid json | ported | `crates/renovate-core/src/extractors/nix.rs:1697` |
| 1051 | returns deps when no root inputs but deps exist | ported | `crates/renovate-core/src/extractors/nix.rs:1703` |
| 1065 | handles currentdigest replacement when config provided | ported | `crates/renovate-core/src/extractors/nix.rs:1439` |
| 1112 | includes nixpkgs with ref when original has rev | ported | `crates/renovate-core/src/extractors/nix.rs:1511` |
| 1154 | includes github flake with ref when original has rev | ported | `crates/renovate-core/src/extractors/nix.rs:1558` |
| 1196 | includes gitlab flake with custom host | ported | `crates/renovate-core/src/extractors/nix.rs:1605` |
| 1238 | includes sourcehut flake with custom host | ported | `crates/renovate-core/src/extractors/nix.rs:1651` |
| 1280 | includes tarball flake with ref when original has rev | ported | `crates/renovate-core/src/extractors/nix.rs:1258` |
| 1321 | handles unknown flake lock type | ported | `crates/renovate-core/src/extractors/nix.rs:1716` |
| 1348 | ignores unsupported file type and still extracts other inputs | ported | `crates/renovate-core/src/extractors/nix.rs:1742` |

