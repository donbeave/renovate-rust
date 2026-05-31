# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/custom/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/custom/index.spec.ts
**Total tests:** 30 | **Ported:** 1 | **Actionable:** 29 | **Status:** pending

### `modules/datasource/custom/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| return null if only the prefix is supplied | 13 | pending | — | — | No corresponding Rust source|
| return null if no registryUrl is provided as well no defaultRegistryTemplate is defined | 22 | pending | — | — | No corresponding Rust source|
| return null if no custom datasource could  be found | 33 | pending | — | — | No corresponding Rust source|
| return null on http error | 42 | pending | — | — | No corresponding Rust source|
| return null if schema validation fails | 56 | pending | — | — | No corresponding Rust source|
| return releases for api directly exposing in renovate format | 72 | pending | — | — | No corresponding Rust source|
| return releases with digests for api directly exposing in renovate format | 93 | pending | — | — | No corresponding Rust source|
| return releases with tags and other optional fields for api directly exposing in renovate format | 123 | pending | — | — | No corresponding Rust source|
| return releases for plain text API directly exposing in Renovate format | 166 | pending | — | — | No corresponding Rust source|
| return releases for plain text API and trim the content | 199 | pending | — | — | No corresponding Rust source|
| returns null if transformation compilation using jsonata fails | 232 | pending | — | — | No corresponding Rust source|
| returns null if jsonata expression evaluation fails | 258 | pending | — | — | No corresponding Rust source|
| return releases for plain text API when only returns a single version | 284 | pending | — | — | No corresponding Rust source|
| return releases for yaml API directly exposing in Renovate format | 308 | pending | — | — | No corresponding Rust source|
| return releases for yaml file directly exposing in Renovate format | 348 | pending | — | — | No corresponding Rust source|
| returns releases for toml API directly exposing in Renovate format | 384 | pending | — | — | No corresponding Rust source|
| return releases for toml file directly exposing in Renovate format | 426 | pending | — | — | No corresponding Rust source|
| return releases for json file directly exposing in Renovate format | 464 | pending | — | — | No corresponding Rust source|
| return null for plain text file if the body is not what is expected | 501 | pending | — | — | No corresponding Rust source|
| return releases for plain text file directly exposing in Renovate format | 518 | pending | — | — | No corresponding Rust source|
| return release when templating registryUrl | 553 | pending | — | — | No corresponding Rust source|
| return release with templated path | 578 | pending | — | — | No corresponding Rust source|
| return release with templated path with multiple layers | 613 | pending | — | — | No corresponding Rust source|
| return releases from HTML links | 650 | pending | — | — | No corresponding Rust source|
| return releases from HTML links - local file | 688 | pending | — | — | No corresponding Rust source|
| return null for local file read error - HTML format | 721 | pending | — | — | No corresponding Rust source|
| return releases from nginx directory listing | 738 | ported | `artifactory.rs` | `parses_nginx_pre_directory_listing_links` | Rust ports the shared HTML directory-listing link extraction used by the custom HTML datasource; the broader custom datasource engine remains pending source-map work. |
| return releases for malformed HTML | 778 | pending | — | — | No corresponding Rust source|
| return releases for incomplete HTML | 815 | pending | — | — | No corresponding Rust source|
| returns null as digest should be provided in releases | 854 | pending | — | — | No corresponding Rust source|

---
