# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/maven/util.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/maven/util.spec.ts
**Total tests:** 16 | **Ported:** 0 | **Actionable:** 16 | **Status:** not-applicable

### `modules/datasource/maven/util`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns error for unsupported protocols | 52 | not-applicable | — | — | TS-library-specific; uses partial<Http> TypeScript OOP partial mock; tests Maven HTTP utilities with TypeScript class instantiation|
| returns error for xml parse error | 63 | not-applicable | — | — | TS-library-specific; uses partial<Http> TypeScript OOP partial mock; tests Maven HTTP utilities with TypeScript class instantiation|
| returns the downloaded text body | 81 | not-applicable | — | — | TS-library-specific; uses partial<Http> TypeScript OOP partial mock; tests Maven HTTP utilities with TypeScript class instantiation|
| returns error for non-S3 URLs | 98 | not-applicable | — | — | TS-library-specific; uses partial<Http> TypeScript OOP partial mock; tests Maven HTTP utilities with TypeScript class instantiation|
| uses correct cache provider for %s | 118 | not-applicable | — | — | TS-library-specific; uses partial<Http> TypeScript OOP partial mock; tests Maven HTTP utilities with TypeScript class instantiation|
| returns empty for HOST_DISABLED error | 108 | not-applicable | — | — | TS-library-specific; uses partial<Http> TypeScript OOP partial mock; tests Maven HTTP utilities with TypeScript class instantiation|
| returns empty for host error | 119 | not-applicable | — | — | TS-library-specific; uses partial<Http> TypeScript OOP partial mock; tests Maven HTTP utilities with TypeScript class instantiation|
| returns empty for temporary error | 130 | not-applicable | — | — | TS-library-specific; uses partial<Http> TypeScript OOP partial mock; tests Maven HTTP utilities with TypeScript class instantiation|
| throws ExternalHostError for 429 status with redis cache | 153 | not-applicable | — | — | TS-library-specific; uses partial<Http> TypeScript OOP partial mock; tests Maven HTTP utilities with TypeScript class instantiation|
| throws ExternalHostError for 429 status without redis cache | 174 | not-applicable | — | — | TS-library-specific; uses partial<Http> TypeScript OOP partial mock; tests Maven HTTP utilities with TypeScript class instantiation|
| throws ExternalHostError for non-429 temporary error on maven central | 195 | not-applicable | — | — | TS-library-specific; uses partial<Http> TypeScript OOP partial mock; tests Maven HTTP utilities with TypeScript class instantiation|
| returns empty for connection error | 210 | not-applicable | — | — | TS-library-specific; uses partial<Http> TypeScript OOP partial mock; tests Maven HTTP utilities with TypeScript class instantiation|
| returns empty for unsupported error | 221 | not-applicable | — | — | TS-library-specific; uses partial<Http> TypeScript OOP partial mock; tests Maven HTTP utilities with TypeScript class instantiation|
| caches 404 for maven-metadata.xml URLs | 301 | not-applicable | — | — | TS-library-specific; uses partial<Http> TypeScript OOP partial mock; tests Maven HTTP utilities with TypeScript class instantiation|
| does not cache 404 for non-metadata URLs | 327 | not-applicable | — | — | TS-library-specific; uses partial<Http> TypeScript OOP partial mock; tests Maven HTTP utilities with TypeScript class instantiation|
| returns cached not-found without making HTTP request | 343 | not-applicable | — | — | TS-library-specific; uses partial<Http> TypeScript OOP partial mock; tests Maven HTTP utilities with TypeScript class instantiation|

---
