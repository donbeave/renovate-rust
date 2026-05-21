# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/http/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/http/index.spec.ts
**Total tests:** 45 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `util/http/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| get | 29 | not-applicable | — | — | tests Http class (got-based) with httpMock; Rust uses reqwest |
| returns 429 error | 40 | not-applicable | — | — | tests Http class (got-based) with httpMock; Rust uses reqwest |
| returns 401 error | 48 | not-applicable | — | — | tests Http class (got-based) with httpMock; Rust uses reqwest |
| converts 404 error to ExternalHostError | 84 | not-applicable | — | — | tests Http class (got-based) with httpMock; Rust uses reqwest |
| disables hosts | 93 | not-applicable | — | — | tests Http class (got-based) with httpMock; Rust uses reqwest |
| ignores 404 error and does not throw ExternalHostError | 100 | not-applicable | — | — | tests Http class (got-based) with httpMock; Rust uses reqwest |
| does not pass auth on redirects | 109 | not-applicable | — | — | tests Http class (got-based) with httpMock; Rust uses reqwest |
| getJson | 127 | not-applicable | — | — | tests Http class (got-based) with httpMock; Rust uses reqwest |
| postJson | 151 | not-applicable | — | — | tests Http class (got-based) with httpMock; Rust uses reqwest |
| putJson | 166 | not-applicable | — | — | tests Http class (got-based) with httpMock; Rust uses reqwest |
| patchJson | 181 | not-applicable | — | — | tests Http class (got-based) with httpMock; Rust uses reqwest |
| deleteJson | 196 | not-applicable | — | — | tests Http class (got-based) with httpMock; Rust uses reqwest |
| headJson | 211 | not-applicable | — | — | tests Http class (got-based) with httpMock; Rust uses reqwest |
| stream | 226 | not-applicable | — | — | tests Http class (got-based) with httpMock; Rust uses reqwest |
| disables hosts for stream | 251 | not-applicable | — | — | tests Http class (got-based) with httpMock; Rust uses reqwest |
| limits concurrency by host | 259 | not-applicable | — | — | tests Http class (got-based) with httpMock; Rust uses reqwest |
| getBuffer | 349 | not-applicable | — | — | tests Http class (got-based) with httpMock; Rust uses reqwest |

### `util/http/index › retry`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works | 369 | not-applicable | — | — | tests Http class (got-based) with httpMock; Rust uses reqwest |

### `util/http/index › Schema support › getPlain`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| gets plain text with correct headers | 402 | not-applicable | — | — | tests Http class (got-based) with httpMock; Rust uses reqwest |
| works with custom options | 412 | not-applicable | — | — | tests Http class (got-based) with httpMock; Rust uses reqwest |

### `util/http/index › Schema support › getYamlUnchecked`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses yaml response without schema | 427 | not-applicable | — | — | tests Http class (got-based) with httpMock; Rust uses reqwest |
| parses yaml with options | 434 | not-applicable | — | — | tests Http class (got-based) with httpMock; Rust uses reqwest |
| throws on invalid yaml | 447 | not-applicable | — | — | tests Http class (got-based) with httpMock; Rust uses reqwest |

### `util/http/index › Schema support › getYaml`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses yaml with schema validation | 457 | not-applicable | — | — | tests Http class (got-based) with httpMock; Rust uses reqwest |
| parses yaml with options and schema | 464 | not-applicable | — | — | tests Http class (got-based) with httpMock; Rust uses reqwest |
| throws on schema validation failure | 479 | not-applicable | — | — | tests Http class (got-based) with httpMock; Rust uses reqwest |
| throws on invalid yaml | 487 | not-applicable | — | — | tests Http class (got-based) with httpMock; Rust uses reqwest |

### `util/http/index › Schema support › getYamlSafe`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns successful result with schema validation | 497 | not-applicable | — | — | tests Http class (got-based) with httpMock; Rust uses reqwest |
| returns schema error result | 508 | not-applicable | — | — | tests Http class (got-based) with httpMock; Rust uses reqwest |
| returns error result for invalid yaml | 522 | not-applicable | — | — | tests Http class (got-based) with httpMock; Rust uses reqwest |
| returns error result for network errors | 533 | not-applicable | — | — | tests Http class (got-based) with httpMock; Rust uses reqwest |
| works with options and schema | 547 | not-applicable | — | — | tests Http class (got-based) with httpMock; Rust uses reqwest |

### `util/http/index › Schema support › getJson`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| uses schema for response body | 568 | not-applicable | — | — | tests Http class (got-based) with httpMock; Rust uses reqwest |
| throws on schema mismatch | 588 | not-applicable | — | — | tests Http class (got-based) with httpMock; Rust uses reqwest |

### `util/http/index › Schema support › getJsonSafe`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| uses schema for response body | 605 | not-applicable | — | — | tests Http class (got-based) with httpMock; Rust uses reqwest |
| returns schema error result | 619 | not-applicable | — | — | tests Http class (got-based) with httpMock; Rust uses reqwest |
| returns error result | 633 | not-applicable | — | — | tests Http class (got-based) with httpMock; Rust uses reqwest |

### `util/http/index › Schema support › postJson`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| uses schema for response body | 646 | not-applicable | — | — | tests Http class (got-based) with httpMock; Rust uses reqwest |
| throws on schema mismatch | 661 | not-applicable | — | — | tests Http class (got-based) with httpMock; Rust uses reqwest |

### `util/http/index › Throttling`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| works without throttling | 679 | not-applicable | — | — | tests Http class (got-based) with httpMock; Rust uses reqwest |
| limits request rate by host | 691 | not-applicable | — | — | tests Http class (got-based) with httpMock; Rust uses reqwest |

### `util/http/index › getToml`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses toml with schema validation | 711 | not-applicable | — | — | tests Http class (got-based) with httpMock; Rust uses reqwest |
| parses toml with options and schema | 718 | not-applicable | — | — | tests Http class (got-based) with httpMock; Rust uses reqwest |
| throws on schema validation failure | 737 | not-applicable | — | — | tests Http class (got-based) with httpMock; Rust uses reqwest |
| throws on invalid toml | 752 | not-applicable | — | — | tests Http class (got-based) with httpMock; Rust uses reqwest |

---

