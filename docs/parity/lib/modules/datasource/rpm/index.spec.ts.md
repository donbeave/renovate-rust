# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/rpm/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/rpm/index.spec.ts
**Total tests:** 22 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `modules/datasource/rpm/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns the correct primary.xml URL | 11 | not-applicable | — | — | RPM/YUM repository metadata datasource lookup, repomd.xml/primary.xml.gz parsing, and checksum handling are not implemented in Rust. |
| returns the correct primary.xml URL when repomd.xml omits xml declaration | 33 | not-applicable | — | — | RPM/YUM repository metadata datasource lookup, repomd.xml/primary.xml.gz parsing, and checksum handling are not implemented in Rust. |
| throws an error if repomd.xml is missing | 55 | not-applicable | — | — | RPM/YUM repository metadata datasource lookup, repomd.xml/primary.xml.gz parsing, and checksum handling are not implemented in Rust. |
| throws an error if http.getText fails | 65 | not-applicable | — | — | RPM/YUM repository metadata datasource lookup, repomd.xml/primary.xml.gz parsing, and checksum handling are not implemented in Rust. |
| throws an error if repomdXml is not in XML format | 76 | not-applicable | — | — | RPM/YUM repository metadata datasource lookup, repomd.xml/primary.xml.gz parsing, and checksum handling are not implemented in Rust. |
| throws an error if no primary data is found | 94 | not-applicable | — | — | RPM/YUM repository metadata datasource lookup, repomd.xml/primary.xml.gz parsing, and checksum handling are not implemented in Rust. |
| throws an error if no location element is found | 116 | not-applicable | — | — | RPM/YUM repository metadata datasource lookup, repomd.xml/primary.xml.gz parsing, and checksum handling are not implemented in Rust. |
| throws an error if location href is missing | 138 | not-applicable | — | — | RPM/YUM repository metadata datasource lookup, repomd.xml/primary.xml.gz parsing, and checksum handling are not implemented in Rust. |
| returns the correct releases | 167 | not-applicable | — | — | RPM/YUM repository metadata datasource lookup, repomd.xml/primary.xml.gz parsing, and checksum handling are not implemented in Rust. |
| throws an error if somesha256-primary.xml.gz is not found | 223 | not-applicable | — | — | RPM/YUM repository metadata datasource lookup, repomd.xml/primary.xml.gz parsing, and checksum handling are not implemented in Rust. |
| throws an error if response.body is empty | 236 | not-applicable | — | — | RPM/YUM repository metadata datasource lookup, repomd.xml/primary.xml.gz parsing, and checksum handling are not implemented in Rust. |
| returns null if no element package is found in primary.xml | 249 | not-applicable | — | — | RPM/YUM repository metadata datasource lookup, repomd.xml/primary.xml.gz parsing, and checksum handling are not implemented in Rust. |
| returns null if the specific packageName is not found in primary.xml | 275 | not-applicable | — | — | RPM/YUM repository metadata datasource lookup, repomd.xml/primary.xml.gz parsing, and checksum handling are not implemented in Rust. |
| returns an empty array if version is not found in a version element | 302 | not-applicable | — | — | RPM/YUM repository metadata datasource lookup, repomd.xml/primary.xml.gz parsing, and checksum handling are not implemented in Rust. |
| returns an array of releases without duplicate versionWithRel | 329 | not-applicable | — | — | RPM/YUM repository metadata datasource lookup, repomd.xml/primary.xml.gz parsing, and checksum handling are not implemented in Rust. |
| handles parser error event in getReleasesByPackageName | 368 | not-applicable | — | — | RPM/YUM repository metadata datasource lookup, repomd.xml/primary.xml.gz parsing, and checksum handling are not implemented in Rust. |
| returns null if registryUrl is not provided | 397 | not-applicable | — | — | RPM/YUM repository metadata datasource lookup, repomd.xml/primary.xml.gz parsing, and checksum handling are not implemented in Rust. |
| returns null if primaryXmlUrl is empty | 405 | not-applicable | — | — | RPM/YUM repository metadata datasource lookup, repomd.xml/primary.xml.gz parsing, and checksum handling are not implemented in Rust. |
| returns null if packageName is not provided | 414 | not-applicable | — | — | RPM/YUM repository metadata datasource lookup, repomd.xml/primary.xml.gz parsing, and checksum handling are not implemented in Rust. |
| returns the correct releases | 422 | not-applicable | — | — | RPM/YUM repository metadata datasource lookup, repomd.xml/primary.xml.gz parsing, and checksum handling are not implemented in Rust. |
| throws an error if getPrimaryGzipUrl fails | 466 | not-applicable | — | — | RPM/YUM repository metadata datasource lookup, repomd.xml/primary.xml.gz parsing, and checksum handling are not implemented in Rust. |
| throws an error if getReleasesByPackageName fails | 479 | not-applicable | — | — | RPM/YUM repository metadata datasource lookup, repomd.xml/primary.xml.gz parsing, and checksum handling are not implemented in Rust. |

---

