# `lib/modules/datasource/go/releases-goproxy.spec.ts`

[← `datasource/go`](../../../../_by-module/datasource/go.md) · [all modules](../../../../README.md)

**1/28 in-scope tests ported** (27 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 27 | encodecase | ported | [`crates/renovate-core/src/datasources/gomod.rs:278`](../../../../../../../crates/renovate-core/src/datasources/gomod.rs#L278) |
| 37 | listversions | pending | — |
| 49 | versioninfo | pending | — |
| 78 | handles direct | pending | — |
| 102 | skips gonoproxy and goprivate packages | pending | — |
| 127 | fetches release data from goproxy | pending | — |
| 171 | handles timestamp fetch errors | pending | — |
| 204 | _(it.each / template — verify manually)_ | ? | — |
| 253 | handles comma fallback | pending | — |
| 303 | short-circuits for errors other than 404 or 410 | pending | — |
| 332 | supports "direct" keyword | pending | — |
| 370 | supports "off" keyword | pending | — |
| 392 | handles soureurl fetch errors | pending | — |
| 423 | _(it.each / template — verify manually)_ | ? | — |
| 479 | handles major releases with 403 status (artifactory) | pending | — |
| 527 | handles gopkg.in major releases | pending | — |
| 570 | handles gopkg.in major releases from v0 | pending | — |
| 607 | handles baseurl with slash at the end | pending | — |
| 644 | continues if package returns no releases | pending | — |
| 661 | uses latest if package has no releases | pending | — |
| 689 | and returns unfiltered `constraints` in the release | pending | — |
| 779 | handles major version updates | pending | — |
| 899 | handles http errors by omitting constraints on failed http requests | pending | — |
| 956 | does not set constraints if no `go` directive | pending | — |
| 998 | _(it.each / template — verify manually)_ | ? | — |
| 1053 | converts minor-only version numbers to include patch of .0 | pending | — |
| 1100 | skips `toolchain` directive | pending | — |
| 1148 | does not look up `go` directive requirements if constraintsfiltering=none | pending | — |

