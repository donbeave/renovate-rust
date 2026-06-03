# `lib/modules/manager/nix/extract.spec.ts`

[← `manager/nix`](../../../../_by-module/manager/nix.md) · [all modules](../../../../README.md)

**38/38 in-scope tests ported** (0 pending, 0 opt-out) · status: ported

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 10 | returns null when no nixpkgs input exists | ported | [`crates/renovate-core/src/extractors/nix.rs:458`](../../../../../../../crates/renovate-core/src/extractors/nix.rs#L458) |
| 25 | does not include nixpkgs input with no explicit ref | ported | [`crates/renovate-core/src/extractors/nix.rs:467`](../../../../../../../crates/renovate-core/src/extractors/nix.rs#L467) |
| 42 | includes nixpkgs input with only ref | ported | [`crates/renovate-core/src/extractors/nix.rs:478`](../../../../../../../crates/renovate-core/src/extractors/nix.rs#L478) |
| 59 | returns null when no inputs | ported | [`crates/renovate-core/src/extractors/nix.rs:489`](../../../../../../../crates/renovate-core/src/extractors/nix.rs#L489) |
| 71 | returns null when inputs are missing locked | ported | [`crates/renovate-core/src/extractors/nix.rs:624`](../../../../../../../crates/renovate-core/src/extractors/nix.rs#L624) |
| 95 | returns null when inputs are missing original | ported | [`crates/renovate-core/src/extractors/nix.rs:652`](../../../../../../../crates/renovate-core/src/extractors/nix.rs#L652) |
| 121 | returns null when original inputs are from local path | ported | [`crates/renovate-core/src/extractors/nix.rs:516`](../../../../../../../crates/renovate-core/src/extractors/nix.rs#L516) |
| 153 | returns null when locked inputs are indirect | ported | [`crates/renovate-core/src/extractors/nix.rs:552`](../../../../../../../crates/renovate-core/src/extractors/nix.rs#L552) |
| 185 | returns null when locked inputs are from local path | ported | [`crates/renovate-core/src/extractors/nix.rs:588`](../../../../../../../crates/renovate-core/src/extractors/nix.rs#L588) |
| 217 | returns nixpkgs input | ported | [`crates/renovate-core/src/extractors/nix.rs:502`](../../../../../../../crates/renovate-core/src/extractors/nix.rs#L502) |
| 260 | includes nixpkgs with no explicit ref | ported | [`crates/renovate-core/src/extractors/nix.rs:682`](../../../../../../../crates/renovate-core/src/extractors/nix.rs#L682) |
| 300 | includes patchelf from head | ported | [`crates/renovate-core/src/extractors/nix.rs:726`](../../../../../../../crates/renovate-core/src/extractors/nix.rs#L726) |
| 358 | includes ijq from sourcehut without a flake | ported | [`crates/renovate-core/src/extractors/nix.rs:773`](../../../../../../../crates/renovate-core/src/extractors/nix.rs#L773) |
| 399 | includes home-manager from gitlab | ported | [`crates/renovate-core/src/extractors/nix.rs:818`](../../../../../../../crates/renovate-core/src/extractors/nix.rs#L818) |
| 440 | test other version | ported | [`crates/renovate-core/src/extractors/nix.rs:863`](../../../../../../../crates/renovate-core/src/extractors/nix.rs#L863) |
| 452 | includes nixpkgs with ref and shallow arguments | ported | [`crates/renovate-core/src/extractors/nix.rs:876`](../../../../../../../crates/renovate-core/src/extractors/nix.rs#L876) |
| 494 | includes nixpkgs but using indirect type that cannot be updated | ported | [`crates/renovate-core/src/extractors/nix.rs:923`](../../../../../../../crates/renovate-core/src/extractors/nix.rs#L923) |
| 524 | includes nixpkgs but using indirect type and path locked type that cannot be updated | ported | [`crates/renovate-core/src/extractors/nix.rs:957`](../../../../../../../crates/renovate-core/src/extractors/nix.rs#L957) |
| 553 | includes flake from github enterprise | ported | [`crates/renovate-core/src/extractors/nix.rs:990`](../../../../../../../crates/renovate-core/src/extractors/nix.rs#L990) |
| 649 | includes flake with tarball type | ported | [`crates/renovate-core/src/extractors/nix.rs:1052`](../../../../../../../crates/renovate-core/src/extractors/nix.rs#L1052) |
| 750 | uri decode gitlab subgroup | ported | [`crates/renovate-core/src/extractors/nix.rs:1303`](../../../../../../../crates/renovate-core/src/extractors/nix.rs#L1303) |
| 790 | includes flake with only tarball type | ported | [`crates/renovate-core/src/extractors/nix.rs:1140`](../../../../../../../crates/renovate-core/src/extractors/nix.rs#L1140) |
| 818 | includes flake with nixpkgs-lib as tarball type | ported | [`crates/renovate-core/src/extractors/nix.rs:1172`](../../../../../../../crates/renovate-core/src/extractors/nix.rs#L1172) |
| 897 | includes flake with nixpkgs channel as tarball type | ported | [`crates/renovate-core/src/extractors/nix.rs:1097`](../../../../../../../crates/renovate-core/src/extractors/nix.rs#L1097) |
| 937 | finds currentdigest correctly when input sha is pinned | ported | [`crates/renovate-core/src/extractors/nix.rs:1347`](../../../../../../../crates/renovate-core/src/extractors/nix.rs#L1347) |
| 983 | does not duplicate nixpkgs dependency | ported | [`crates/renovate-core/src/extractors/nix.rs:1392`](../../../../../../../crates/renovate-core/src/extractors/nix.rs#L1392) |
| 1028 | returns null when flake.lock file cannot be read | ported | [`crates/renovate-core/src/extractors/nix.rs:1492`](../../../../../../../crates/renovate-core/src/extractors/nix.rs#L1492) |
| 1033 | returns null when flake.nix file cannot be read | ported | [`crates/renovate-core/src/extractors/nix.rs:1498`](../../../../../../../crates/renovate-core/src/extractors/nix.rs#L1498) |
| 1046 | returns null when flake.lock has invalid json | ported | [`crates/renovate-core/src/extractors/nix.rs:1697`](../../../../../../../crates/renovate-core/src/extractors/nix.rs#L1697) |
| 1051 | returns deps when no root inputs but deps exist | ported | [`crates/renovate-core/src/extractors/nix.rs:1703`](../../../../../../../crates/renovate-core/src/extractors/nix.rs#L1703) |
| 1065 | handles currentdigest replacement when config provided | ported | [`crates/renovate-core/src/extractors/nix.rs:1439`](../../../../../../../crates/renovate-core/src/extractors/nix.rs#L1439) |
| 1112 | includes nixpkgs with ref when original has rev | ported | [`crates/renovate-core/src/extractors/nix.rs:1511`](../../../../../../../crates/renovate-core/src/extractors/nix.rs#L1511) |
| 1154 | includes github flake with ref when original has rev | ported | [`crates/renovate-core/src/extractors/nix.rs:1558`](../../../../../../../crates/renovate-core/src/extractors/nix.rs#L1558) |
| 1196 | includes gitlab flake with custom host | ported | [`crates/renovate-core/src/extractors/nix.rs:1605`](../../../../../../../crates/renovate-core/src/extractors/nix.rs#L1605) |
| 1238 | includes sourcehut flake with custom host | ported | [`crates/renovate-core/src/extractors/nix.rs:1651`](../../../../../../../crates/renovate-core/src/extractors/nix.rs#L1651) |
| 1280 | includes tarball flake with ref when original has rev | ported | [`crates/renovate-core/src/extractors/nix.rs:1258`](../../../../../../../crates/renovate-core/src/extractors/nix.rs#L1258) |
| 1321 | handles unknown flake lock type | ported | [`crates/renovate-core/src/extractors/nix.rs:1716`](../../../../../../../crates/renovate-core/src/extractors/nix.rs#L1716) |
| 1348 | ignores unsupported file type and still extracts other inputs | ported | [`crates/renovate-core/src/extractors/nix.rs:1742`](../../../../../../../crates/renovate-core/src/extractors/nix.rs#L1742) |

