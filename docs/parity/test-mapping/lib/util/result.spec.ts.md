# `lib/util/result.spec.ts`

[← `util/_root`](../../_by-module/util/_root.md) · [all modules](../../README.md)

**2/85 in-scope tests ported** (83 pending, 0 opt-out) · status: partial

| Line | Test | Status | Rust destination / opt-out reason |
|--:|---|---|---|
| 12 | ok result | pending | — |
| 22 | error result | pending | — |
| 34 | wraps callback returning value | ported | [`crates/renovate-core/src/util/result.rs:551`](../../../../../crates/renovate-core/src/util/result.rs#L551) |
| 39 | handles throw in callback | pending | — |
| 46 | wraps callback returning promise | pending | — |
| 51 | wraps callback returning failed promise | pending | — |
| 57 | wraps nullable callback | pending | — |
| 65 | wraps nullable callback null | pending | — |
| 70 | wraps nullable callback undefined | pending | — |
| 75 | distincts between null and undefined callback results | pending | — |
| 84 | handles nullable callback error | pending | — |
| 91 | wraps pure nullable value | pending | — |
| 96 | wraps nullable value null | pending | — |
| 101 | wraps nullable value undefined | pending | — |
| 106 | wraps zod parse result | pending | — |
| 118 | unwraps successful value | pending | — |
| 126 | unwraps error value | pending | — |
| 134 | skips fallback for successful value | pending | — |
| 139 | uses fallback for error value | pending | — |
| 144 | unwrapor throws uncaught transform error | pending | — |
| 155 | unwrap throws uncaught transform error | pending | — |
| 166 | returns ok-value for unwraporthrow | pending | — |
| 171 | throws error for unwraporthrow on error result | pending | — |
| 176 | unwrapornull returns value for ok-result | pending | — |
| 181 | unwrapornull returns null for error result | pending | — |
| 186 | unwrapornull throws uncaught transform error | pending | — |
| 199 | transforms value to value | pending | — |
| 204 | transforms value to result | pending | — |
| 211 | skips transform for error result | pending | — |
| 218 | logs and returns error on transform failure | pending | — |
| 230 | automatically converts zod values | pending | — |
| 238 | bypasses ok result | pending | — |
| 244 | bypasses uncaught transform errors | pending | — |
| 252 | converts error to result | pending | — |
| 258 | handles error thrown in catch function | pending | — |
| 267 | parses zod schema | pending | — |
| 306 | parses zod schema by piping from result | pending | — |
| 329 | supports value handlers | pending | — |
| 335 | supports error handlers | pending | — |
| 341 | handles error thrown in value handler | pending | — |
| 348 | handles error thrown in error handler | pending | — |
| 359 | wraps promise | pending | — |
| 366 | wraps result promise | pending | — |
| 373 | handles rejected promise | pending | — |
| 380 | wraps nullable promise | pending | — |
| 388 | wraps promise returning null | pending | — |
| 393 | wraps promise returning undefined | pending | — |
| 398 | distincts between null and undefined promise results | pending | — |
| 408 | handles rejected nullable promise | pending | — |
| 415 | unwraps successful asyncresult | pending | — |
| 423 | unwraps error asyncresult | pending | — |
| 431 | skips fallback for successful asyncresult | pending | — |
| 436 | uses fallback for error asyncresult | pending | — |
| 441 | returns ok-value for unwraporthrow | pending | — |
| 446 | rejects for error for unwraporthrow | pending | — |
| 451 | unwrapornull returns value for ok-result | pending | — |
| 456 | unwrapornull returns null for error result | pending | — |
| 463 | transforms asyncresult to pure value | pending | — |
| 470 | transforms asyncresult to result | pending | — |
| 477 | transforms result to asyncresult | ported | [`crates/renovate-core/src/util/result.rs:611`](../../../../../crates/renovate-core/src/util/result.rs#L611) |
| 484 | transforms asyncresult to asyncresult | pending | — |
| 491 | skips transform for failed promises | pending | — |
| 498 | asyncronously transforms successfull promise to value | pending | — |
| 505 | asynchronously transforms successful asyncresult to result | pending | — |
| 512 | asynchronously transforms value to value | pending | — |
| 519 | asynchronously transforms value to result | pending | — |
| 526 | skips async transform for error result | pending | — |
| 534 | skips async transform for rejected promise | pending | — |
| 541 | re-wraps error thrown via unwrapping in async transform | pending | — |
| 551 | handles error thrown on result async transform | pending | — |
| 563 | handles error thrown on promise transform | pending | — |
| 577 | handles error thrown on promise async transform | pending | — |
| 589 | accumulates error types into union type during chained transform | pending | — |
| 608 | asynchronously transforms result to zod values | pending | — |
| 616 | transforms asyncresult to zod values | pending | — |
| 626 | converts error to asyncresult | pending | — |
| 632 | converts error to promise | pending | — |
| 639 | handles error thrown in promise result | pending | — |
| 645 | converts asyncresult error to result | pending | — |
| 655 | parses zod schema by piping from asyncresult | pending | — |
| 676 | handles uncaught error thrown in the steps before parsing | pending | — |
| 688 | supports value handlers | pending | — |
| 694 | supports error handlers | pending | — |
| 700 | handles error thrown in value handler | pending | — |
| 707 | handles error thrown in error handler | pending | — |

