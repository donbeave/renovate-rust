# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/rpm/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/rpm/index.spec.ts
**Total tests:** 28 | **Ported:** 0 | **Actionable:** 28 | **Status:** pending

### `modules/datasource/rpm/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns the correct primary.xml URL | 11 | pending | — | — | No corresponding Rust source|
| returns the correct primary.xml URL when repomd.xml omits xml declaration | 33 | pending | — | — | No corresponding Rust source|
| throws an error if repomd.xml is missing | 55 | pending | — | — | No corresponding Rust source|
| throws an error if http.getText fails | 65 | pending | — | — | No corresponding Rust source|
| throws an error if repomdXml is not in XML format | 76 | pending | — | — | No corresponding Rust source|
| throws an error if no primary data is found | 94 | pending | — | — | No corresponding Rust source|
| throws an error if no location element is found | 116 | pending | — | — | No corresponding Rust source|
| throws an error if location href is missing | 138 | pending | — | — | No corresponding Rust source|
| returns the correct releases | 167 | pending | — | — | No corresponding Rust source|
| throws an error if somesha256-primary.xml.gz is not found | 223 | pending | — | — | No corresponding Rust source|
| throws an error if response.body is empty | 236 | pending | — | — | No corresponding Rust source|
| returns null if no element package is found in primary.xml | 249 | pending | — | — | No corresponding Rust source|
| returns null if the specific packageName is not found in primary.xml | 275 | pending | — | — | No corresponding Rust source|
| returns an empty array if version is not found in a version element | 302 | pending | — | — | No corresponding Rust source|
| returns an array of releases without duplicate versionWithRel | 329 | pending | — | — | No corresponding Rust source|
| handles parser error event in getReleasesByPackageName | 368 | pending | — | — | No corresponding Rust source|
| returns null if registryUrl is not provided | 397 | pending | — | — | No corresponding Rust source|
| returns null if primaryXmlUrl is empty | 405 | pending | — | — | No corresponding Rust source|
| returns null if packageName is not provided | 414 | pending | — | — | No corresponding Rust source|
| returns the correct releases | 422 | pending | — | — | No corresponding Rust source|
| throws an error if getPrimaryGzipUrl fails | 466 | pending | — | — | No corresponding Rust source|
| throws an error if getReleasesByPackageName fails | 479 | pending | — | — | No corresponding Rust source|
| rethrows non-Error fetch failures | 276 | pending | — | — | No corresponding Rust source|
| reuses the extracted primary.xml file across package lookups | 292 | pending | — | — | No corresponding Rust source|
| re-downloads primary.xml if the freshness check fails | 336 | pending | — | — | No corresponding Rust source|
| throws if extracting primary.xml fails without an existing cache file | 380 | pending | — | — | No corresponding Rust source|
| keeps the previous extracted primary.xml if a refresh extract fails | 414 | pending | — | — | No corresponding Rust source|
| replaces the extracted primary.xml after a successful refresh | 471 | pending | — | — | No corresponding Rust source|

---
