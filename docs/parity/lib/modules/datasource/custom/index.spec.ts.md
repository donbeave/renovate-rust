# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/custom/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/custom/index.spec.ts
**Total tests:** 30 | **Ported:** 1 | **Actionable:** 30 | **Status:** partial

### `modules/datasource/custom/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| return null if only the prefix is supplied | 13 | pending | — | — | — |
| return null if no registryUrl is provided as well no defaultRegistryTemplate is defined | 22 | pending | — | — | — |
| return null if no custom datasource could  be found | 33 | pending | — | — | — |
| return null on http error | 42 | pending | — | — | — |
| return null if schema validation fails | 56 | pending | — | — | — |
| return releases for api directly exposing in renovate format | 72 | pending | — | — | — |
| return releases with digests for api directly exposing in renovate format | 93 | pending | — | — | — |
| return releases with tags and other optional fields for api directly exposing in renovate format | 123 | pending | — | — | — |
| return releases for plain text API directly exposing in Renovate format | 166 | pending | — | — | — |
| return releases for plain text API and trim the content | 199 | pending | — | — | — |
| returns null if transformation compilation using jsonata fails | 232 | pending | — | — | — |
| returns null if jsonata expression evaluation fails | 258 | pending | — | — | — |
| return releases for plain text API when only returns a single version | 284 | pending | — | — | — |
| return releases for yaml API directly exposing in Renovate format | 308 | pending | — | — | — |
| return releases for yaml file directly exposing in Renovate format | 348 | pending | — | — | — |
| returns releases for toml API directly exposing in Renovate format | 384 | pending | — | — | — |
| return releases for toml file directly exposing in Renovate format | 426 | pending | — | — | — |
| return releases for json file directly exposing in Renovate format | 464 | pending | — | — | — |
| return null for plain text file if the body is not what is expected | 501 | pending | — | — | — |
| return releases for plain text file directly exposing in Renovate format | 518 | pending | — | — | — |
| return release when templating registryUrl | 553 | pending | — | — | — |
| return release with templated path | 578 | pending | — | — | — |
| return release with templated path with multiple layers | 613 | pending | — | — | — |
| return releases from HTML links | 650 | pending | — | — | — |
| return releases from HTML links - local file | 688 | pending | — | — | — |
| return null for local file read error - HTML format | 721 | pending | — | — | — |
| return releases from nginx directory listing | 738 | ported | `artifactory.rs` | `parses_nginx_pre_directory_listing_links` | Rust ports the shared HTML directory-listing link extraction used by the custom HTML datasource; the broader custom datasource engine remains pending source-map work. |
| return releases for malformed HTML | 778 | pending | — | — | — |
| return releases for incomplete HTML | 815 | pending | — | — | — |
| returns null as digest should be provided in releases | 854 | pending | — | — | — |

---

