# `lib/util/http/index.spec.ts`

[← `util/http`](../../../_by-module/util/http.md) · [all modules](../../../README.md)

**18/52 in-scope tests ported** (34 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 37 | sets default user-agent | ported | [`crates/renovate-core/src/http.rs:1034`](../../../../../../crates/renovate-core/src/http.rs#L1034) |
| 47 | uses useragent when set as a plain string | pending | — |
| 56 | interpolates {{renovateversion}} in a custom useragent template | pending | — |
| 69 | renders unknown template variables as empty string | pending | — |
| 78 | supports handlebars helpers in useragent template | pending | — |
| 89 | supports conditional handlebars syntax in useragent template | pending | — |
| 101 | preserves existing headers | ported | [`crates/renovate-core/src/http.rs:1049`](../../../../../../crates/renovate-core/src/http.rs#L1049) |
| 111 | get | ported | [`crates/renovate-core/src/http.rs:902`](../../../../../../crates/renovate-core/src/http.rs#L902) |
| 122 | returns 429 error | ported | [`crates/renovate-core/src/http.rs:923`](../../../../../../crates/renovate-core/src/http.rs#L923) |
| 130 | returns 401 error | ported | [`crates/renovate-core/src/http.rs:938`](../../../../../../crates/renovate-core/src/http.rs#L938) |
| 166 | converts 404 error to externalhosterror | pending | — |
| 175 | disables hosts | pending | — |
| 182 | ignores 404 error and does not throw externalhosterror | pending | — |
| 191 | does not pass auth on redirects | pending | — |
| 209 | getjson | ported | [`crates/renovate-core/src/http.rs:852`](../../../../../../crates/renovate-core/src/http.rs#L852) |
| 233 | postjson | ported | [`crates/renovate-core/src/http.rs:887`](../../../../../../crates/renovate-core/src/http.rs#L887) |
| 248 | putjson | ported | [`crates/renovate-core/src/http.rs:967`](../../../../../../crates/renovate-core/src/http.rs#L967) |
| 263 | patchjson | ported | [`crates/renovate-core/src/http.rs:984`](../../../../../../crates/renovate-core/src/http.rs#L984) |
| 278 | deletejson | ported | [`crates/renovate-core/src/http.rs:1001`](../../../../../../crates/renovate-core/src/http.rs#L1001) |
| 293 | headjson | ported | [`crates/renovate-core/src/http.rs:1018`](../../../../../../crates/renovate-core/src/http.rs#L1018) |
| 308 | stream | pending | — |
| 333 | disables hosts for stream | pending | — |
| 341 | limits concurrency by host | pending | — |
| 431 | getbuffer | pending | — |
| 451 | works | pending | — |
| 484 | gets plain text with correct headers | ported | [`crates/renovate-core/src/http.rs:869`](../../../../../../crates/renovate-core/src/http.rs#L869) |
| 494 | works with custom options | pending | — |
| 509 | parses yaml response without schema | ported | [`crates/renovate-core/src/http.rs:1069`](../../../../../../crates/renovate-core/src/http.rs#L1069) |
| 516 | parses yaml with options | pending | — |
| 529 | throws on invalid yaml | ported | [`crates/renovate-core/src/http.rs:1085`](../../../../../../crates/renovate-core/src/http.rs#L1085) |
| 539 | parses yaml with schema validation | pending | — |
| 546 | parses yaml with options and schema | pending | — |
| 561 | throws on schema validation failure | pending | — |
| 569 | throws on invalid yaml | ported | [`crates/renovate-core/src/http.rs:1085`](../../../../../../crates/renovate-core/src/http.rs#L1085) |
| 579 | returns successful result with schema validation | pending | — |
| 590 | returns schema error result | pending | — |
| 604 | returns error result for invalid yaml | ported | [`crates/renovate-core/src/http.rs:1100`](../../../../../../crates/renovate-core/src/http.rs#L1100) |
| 615 | returns error result for network errors | ported | [`crates/renovate-core/src/http.rs:1115`](../../../../../../crates/renovate-core/src/http.rs#L1115) |
| 629 | works with options and schema | pending | — |
| 650 | uses schema for response body | pending | — |
| 670 | throws on schema mismatch | pending | — |
| 687 | uses schema for response body | pending | — |
| 701 | returns schema error result | pending | — |
| 715 | returns error result | pending | — |
| 728 | uses schema for response body | pending | — |
| 743 | throws on schema mismatch | pending | — |
| 761 | works without throttling | pending | — |
| 773 | limits request rate by host | pending | — |
| 793 | parses toml with schema validation | ported | [`crates/renovate-core/src/http.rs:1145`](../../../../../../crates/renovate-core/src/http.rs#L1145) |
| 800 | parses toml with options and schema | pending | — |
| 819 | throws on schema validation failure | pending | — |
| 834 | throws on invalid toml | ported | [`crates/renovate-core/src/http.rs:1130`](../../../../../../crates/renovate-core/src/http.rs#L1130) |

