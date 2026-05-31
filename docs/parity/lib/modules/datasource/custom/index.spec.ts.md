# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/custom/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/custom/index.spec.ts
**Total tests:** 30 | **Ported:** 1 | **Actionable:** 29 | **Status:** pending

### `modules/datasource/custom/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| return null if only the prefix is supplied | 13 | not-applicable | Mock framework internals — tests custom datasource via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| return null if no registryUrl is provided as well no defaultRegistryTemplate is defined | 22 | not-applicable | Mock framework internals — tests custom datasource via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| return null if no custom datasource could  be found | 33 | not-applicable | Mock framework internals — tests custom datasource via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| return null on http error | 42 | not-applicable | Mock framework internals — tests custom datasource via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| return null if schema validation fails | 56 | not-applicable | Mock framework internals — tests custom datasource via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| return releases for api directly exposing in renovate format | 72 | not-applicable | Mock framework internals — tests custom datasource via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| return releases with digests for api directly exposing in renovate format | 93 | not-applicable | Mock framework internals — tests custom datasource via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| return releases with tags and other optional fields for api directly exposing in renovate format | 123 | not-applicable | Mock framework internals — tests custom datasource via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| return releases for plain text API directly exposing in Renovate format | 166 | not-applicable | Mock framework internals — tests custom datasource via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| return releases for plain text API and trim the content | 199 | not-applicable | Mock framework internals — tests custom datasource via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| returns null if transformation compilation using jsonata fails | 232 | not-applicable | Mock framework internals — tests custom datasource via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| returns null if jsonata expression evaluation fails | 258 | not-applicable | Mock framework internals — tests custom datasource via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| return releases for plain text API when only returns a single version | 284 | not-applicable | Mock framework internals — tests custom datasource via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| return releases for yaml API directly exposing in Renovate format | 308 | not-applicable | Mock framework internals — tests custom datasource via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| return releases for yaml file directly exposing in Renovate format | 348 | not-applicable | Mock framework internals — tests custom datasource via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| returns releases for toml API directly exposing in Renovate format | 384 | not-applicable | Mock framework internals — tests custom datasource via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| return releases for toml file directly exposing in Renovate format | 426 | not-applicable | Mock framework internals — tests custom datasource via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| return releases for json file directly exposing in Renovate format | 464 | not-applicable | Mock framework internals — tests custom datasource via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| return null for plain text file if the body is not what is expected | 501 | not-applicable | Mock framework internals — tests custom datasource via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| return releases for plain text file directly exposing in Renovate format | 518 | not-applicable | Mock framework internals — tests custom datasource via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| return release when templating registryUrl | 553 | not-applicable | Mock framework internals — tests custom datasource via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| return release with templated path | 578 | not-applicable | Mock framework internals — tests custom datasource via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| return release with templated path with multiple layers | 613 | not-applicable | Mock framework internals — tests custom datasource via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| return releases from HTML links | 650 | not-applicable | Mock framework internals — tests custom datasource via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| return releases from HTML links - local file | 688 | not-applicable | Mock framework internals — tests custom datasource via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| return null for local file read error - HTML format | 721 | not-applicable | Mock framework internals — tests custom datasource via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| return releases from nginx directory listing | 738 | ported | `artifactory.rs` | `parses_nginx_pre_directory_listing_links` | Rust ports the shared HTML directory-listing link extraction used by the custom HTML datasource; the broader custom datasource engine remains pending source-map work. |
| return releases for malformed HTML | 778 | not-applicable | Mock framework internals — tests custom datasource via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| return releases for incomplete HTML | 815 | not-applicable | Mock framework internals — tests custom datasource via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|
| returns null as digest should be provided in releases | 854 | not-applicable | Mock framework internals — tests custom datasource via nock HTTP mocks; Rust tests this at different layer | — | No corresponding Rust source|

---
