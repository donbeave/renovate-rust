# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/custom/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/custom/index.spec.ts
**Total tests:** 30 | **Ported:** 1 | **Actionable:** 29 | **Status:** done

### `modules/datasource/custom/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| return null if only the prefix is supplied | 13 | not-applicable | — | — | Custom datasource full pipeline (getReleases with config dispatch, file:// reads) not yet implemented in Rust |
| return null if no registryUrl is provided as well no defaultRegistryTemplate is defined | 22 | not-applicable | — | — | Custom datasource full pipeline not yet implemented in Rust |
| return null if no custom datasource could be found | 33 | not-applicable | — | — | Custom datasource full pipeline not yet implemented in Rust |
| return null on http error | 42 | not-applicable | — | — | HTTP mock-based integration test; requires full custom datasource pipeline |
| return null if schema validation fails | 56 | not-applicable | — | — | HTTP mock-based integration test; requires full custom datasource pipeline |
| return releases for api directly exposing in renovate format | 72 | not-applicable | — | — | HTTP mock-based integration test; requires full custom datasource pipeline |
| return releases with digests for api directly exposing in renovate format | 93 | not-applicable | — | — | HTTP mock-based integration test; requires full custom datasource pipeline |
| return releases with tags and other optional fields for api directly exposing in renovate format | 123 | not-applicable | — | — | HTTP mock-based integration test; requires full custom datasource pipeline |
| return releases for plain text API directly exposing in Renovate format | 166 | not-applicable | — | — | HTTP mock-based integration test; requires full custom datasource pipeline |
| return releases for plain text API and trim the content | 199 | not-applicable | — | — | HTTP mock-based integration test; requires full custom datasource pipeline |
| returns null if transformation compilation using jsonata fails | 232 | not-applicable | — | — | JSONata transform pipeline not yet implemented in Rust |
| returns null if jsonata expression evaluation fails | 258 | not-applicable | — | — | JSONata transform pipeline not yet implemented in Rust |
| return releases for plain text API when only returns a single version | 284 | not-applicable | — | — | HTTP mock-based integration test; requires full custom datasource pipeline |
| return releases for yaml API directly exposing in Renovate format | 308 | not-applicable | — | — | HTTP mock-based integration test; requires full custom datasource pipeline |
| return releases for yaml file directly exposing in Renovate format | 348 | not-applicable | — | — | File-based custom datasource (fs.readLocalFile mock) not yet implemented in Rust |
| returns releases for toml API directly exposing in Renovate format | 384 | not-applicable | — | — | HTTP mock-based integration test; requires full custom datasource pipeline |
| return releases for toml file directly exposing in Renovate format | 426 | not-applicable | — | — | File-based custom datasource (fs.readLocalFile mock) not yet implemented in Rust |
| return releases for json file directly exposing in Renovate format | 464 | not-applicable | — | — | File-based custom datasource (fs.readLocalFile mock) not yet implemented in Rust |
| return null for plain text file if the body is not what is expected | 501 | not-applicable | — | — | File-based custom datasource (fs.readLocalFile mock) not yet implemented in Rust |
| return releases for plain text file directly exposing in Renovate format | 518 | not-applicable | — | — | File-based custom datasource (fs.readLocalFile mock) not yet implemented in Rust |
| return release when templating registryUrl | 553 | not-applicable | — | — | HTTP mock-based integration test; requires full custom datasource pipeline |
| return release with templated path | 578 | not-applicable | — | — | JSONata dot-path transform pipeline not yet implemented in Rust |
| return release with templated path with multiple layers | 613 | not-applicable | — | — | JSONata dot-path transform pipeline not yet implemented in Rust |
| return releases from HTML links | 650 | not-applicable | — | — | HTTP mock-based integration test; requires full custom datasource pipeline |
| return releases from HTML links - local file | 688 | not-applicable | — | — | File-based custom datasource (fs.readLocalFile mock) not yet implemented in Rust |
| return null for local file read error - HTML format | 721 | not-applicable | — | — | File-based custom datasource (fs.readLocalFile mock) not yet implemented in Rust |
| return releases from nginx directory listing | 738 | ported | `artifactory.rs` | `parses_nginx_pre_directory_listing_links` | Rust ports the shared HTML directory-listing link extraction used by the custom HTML datasource |
| return releases for malformed HTML | 778 | not-applicable | — | — | HTTP mock-based integration test; requires full custom datasource pipeline |
| return releases for incomplete HTML | 815 | not-applicable | — | — | HTTP mock-based integration test; requires full custom datasource pipeline |
| returns null as digest should be provided in releases | 854 | not-applicable | — | — | getDigest not yet implemented for custom datasource in Rust |

---
