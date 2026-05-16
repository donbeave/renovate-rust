# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/util/result.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/result.spec.ts
**Total tests:** 85 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `util/result › Result › constructors`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| ok result | 12 | not-applicable | — | — | Renovate's TypeScript `Result` wrapper is not implemented as a Rust API; Rust uses the standard `Result` type directly. |
| error result | 22 | not-applicable | — | — | Renovate's TypeScript `Result` wrapper is not implemented as a Rust API; Rust uses the standard `Result` type directly. |

### `util/result › Result › Wrapping`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| wraps callback returning value | 34 | not-applicable | — | — | Renovate's TypeScript callback-wrapping helper is not implemented as a Rust API; Rust uses standard `Result` construction. |
| handles throw in callback | 39 | not-applicable | — | — | Renovate's JavaScript thrown-value wrapping behavior has no Rust API equivalent. |
| wraps callback returning promise | 46 | not-applicable | — | — | Renovate's TypeScript promise-to-`AsyncResult` wrapper has no Rust API equivalent. |
| wraps callback returning failed promise | 51 | not-applicable | — | — | Renovate's TypeScript promise-to-`AsyncResult` wrapper has no Rust API equivalent. |
| wraps nullable callback | 57 | not-applicable | — | — | Renovate's TypeScript nullish-to-error helper has no Rust API equivalent; Rust uses `Option`/`Result` explicitly. |
| wraps nullable callback null | 65 | not-applicable | — | — | Renovate's JavaScript `null` handling has no Rust API equivalent; Rust uses `Option`/`Result` explicitly. |
| wraps nullable callback undefined | 70 | not-applicable | — | — | Renovate's JavaScript `undefined` handling has no Rust API equivalent. |
| distincts between null and undefined callback results | 75 | not-applicable | — | — | Renovate's JavaScript null-vs-undefined distinction has no Rust API equivalent. |
| handles nullable callback error | 84 | not-applicable | — | — | Renovate's JavaScript thrown-value wrapping behavior has no Rust API equivalent. |
| wraps pure nullable value | 91 | not-applicable | — | — | Renovate's TypeScript nullish-to-error helper has no Rust API equivalent; Rust uses `Option`/`Result` explicitly. |
| wraps nullable value null | 96 | not-applicable | — | — | Renovate's JavaScript `null` handling has no Rust API equivalent. |
| wraps nullable value undefined | 101 | not-applicable | — | — | Renovate's JavaScript `undefined` handling has no Rust API equivalent. |
| wraps zod parse result | 106 | not-applicable | — | — | Renovate's Zod `safeParse` integration has no Rust API equivalent. |

### `util/result › Result › Unwrapping`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| unwraps successful value | 120 | not-applicable | — | — | Renovate's TypeScript `Result` wrapper unwrapping shape is not implemented as a Rust API. |
| unwraps error value | 128 | not-applicable | — | — | Renovate's TypeScript `Result` wrapper unwrapping shape is not implemented as a Rust API. |
| skips fallback for successful value | 136 | not-applicable | — | — | Renovate's TypeScript `unwrapOr` helper is not implemented as a Rust API; Rust uses standard `Result` combinators. |
| uses fallback for error value | 141 | not-applicable | — | — | Renovate's TypeScript `unwrapOr` helper is not implemented as a Rust API; Rust uses standard `Result` combinators. |
| unwrapOr throws uncaught transform error | 146 | not-applicable | — | — | Renovate's JavaScript uncaught-transform sentinel behavior has no Rust API equivalent. |
| unwrap throws uncaught transform error | 157 | not-applicable | — | — | Renovate's JavaScript uncaught-transform sentinel behavior has no Rust API equivalent. |
| returns ok-value for unwrapOrThrow | 168 | not-applicable | — | — | Renovate's TypeScript `unwrapOrThrow` helper is not implemented as a Rust API. |
| throws error for unwrapOrThrow on error result | 173 | not-applicable | — | — | Renovate's JavaScript thrown-value behavior has no Rust API equivalent. |
| unwrapOrNull returns value for ok-result | 178 | not-applicable | — | — | Renovate's TypeScript `unwrapOrNull` helper has no Rust API equivalent. |
| unwrapOrNull returns null for error result | 183 | not-applicable | — | — | Renovate's TypeScript `unwrapOrNull` helper has no Rust API equivalent. |
| unwrapOrNull throws uncaught transform error | 188 | not-applicable | — | — | Renovate's JavaScript uncaught-transform sentinel behavior has no Rust API equivalent. |

### `util/result › Result › Transforming`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| transforms value to value | 201 | not-applicable | — | — | Renovate's TypeScript `Result.transform` helper is not implemented as a Rust API; Rust uses standard `Result` combinators. |
| transforms value to Result | 206 | not-applicable | — | — | Renovate's TypeScript `Result.transform` helper is not implemented as a Rust API; Rust uses standard `Result` combinators. |
| skips transform for error Result | 213 | not-applicable | — | — | Renovate's TypeScript `Result.transform` helper is not implemented as a Rust API; Rust uses standard `Result` combinators. |
| logs and returns error on transform failure | 220 | not-applicable | — | — | Renovate's JavaScript logger side effect for thrown transform errors has no Rust API equivalent. |
| automatically converts zod values | 232 | not-applicable | — | — | Renovate's Zod `safeParse` integration has no Rust API equivalent. |

### `util/result › Result › Catch`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| bypasses ok result | 240 | not-applicable | — | — | Renovate's TypeScript `Result.catch` helper is not implemented as a Rust API; Rust uses standard `Result` combinators. |
| bypasses uncaught transform errors | 246 | not-applicable | — | — | Renovate's JavaScript uncaught-transform sentinel behavior has no Rust API equivalent. |
| converts error to Result | 254 | not-applicable | — | — | Renovate's TypeScript `Result.catch` helper is not implemented as a Rust API; Rust uses standard `Result` combinators. |
| handles error thrown in catch function | 260 | not-applicable | — | — | Renovate's JavaScript thrown-value wrapping behavior has no Rust API equivalent. |

### `util/result › Result › Parsing`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses Zod schema | 269 | not-applicable | — | — | Renovate's Zod parsing helper on `Result` has no Rust API equivalent. |
| parses Zod schema by piping from Result | 302 | not-applicable | — | — | Renovate's Zod parsing helper on `Result` has no Rust API equivalent. |

### `util/result › Result › Handlers`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| supports value handlers | 319 | not-applicable | — | — | Renovate's TypeScript `onValue` handler helper is not implemented as a Rust API. |
| supports error handlers | 325 | not-applicable | — | — | Renovate's TypeScript `onError` handler helper is not implemented as a Rust API. |
| handles error thrown in value handler | 331 | not-applicable | — | — | Renovate's JavaScript thrown-value handler behavior has no Rust API equivalent. |
| handles error thrown in error handler | 338 | not-applicable | — | — | Renovate's JavaScript thrown-value handler behavior has no Rust API equivalent. |

### `util/result › AsyncResult › Wrapping`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| wraps promise | 349 | not-applicable | — | — | Renovate's TypeScript `AsyncResult` promise wrapper is not implemented as a Rust API; Rust uses async `Result` directly. |
| wraps Result promise | 356 | not-applicable | — | — | Renovate's TypeScript `AsyncResult` promise wrapper is not implemented as a Rust API; Rust uses async `Result` directly. |
| handles rejected promise | 363 | not-applicable | — | — | Renovate's JavaScript promise rejection wrapping has no Rust API equivalent. |
| wraps nullable promise | 370 | not-applicable | — | — | Renovate's TypeScript nullish promise wrapper has no Rust API equivalent. |
| wraps promise returning null | 378 | not-applicable | — | — | Renovate's JavaScript `null` promise handling has no Rust API equivalent. |
| wraps promise returning undefined | 383 | not-applicable | — | — | Renovate's JavaScript `undefined` promise handling has no Rust API equivalent. |
| distincts between null and undefined promise results | 388 | not-applicable | — | — | Renovate's JavaScript null-vs-undefined distinction has no Rust API equivalent. |
| handles rejected nullable promise | 398 | not-applicable | — | — | Renovate's JavaScript promise rejection wrapping has no Rust API equivalent. |

### `util/result › AsyncResult › Unwrapping`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| unwraps successful AsyncResult | 405 | not-applicable | — | — | Renovate's TypeScript `AsyncResult` unwrapping helper is not implemented as a Rust API. |
| unwraps error AsyncResult | 413 | not-applicable | — | — | Renovate's TypeScript `AsyncResult` unwrapping helper is not implemented as a Rust API. |
| skips fallback for successful AsyncResult | 421 | not-applicable | — | — | Renovate's TypeScript async `unwrapOr` helper is not implemented as a Rust API. |
| uses fallback for error AsyncResult | 426 | not-applicable | — | — | Renovate's TypeScript async `unwrapOr` helper is not implemented as a Rust API. |
| returns ok-value for unwrapOrThrow | 431 | not-applicable | — | — | Renovate's TypeScript async `unwrapOrThrow` helper is not implemented as a Rust API. |
| rejects for error for unwrapOrThrow | 436 | not-applicable | — | — | Renovate's JavaScript promise rejection behavior has no Rust API equivalent. |
| unwrapOrNull returns value for ok-result | 441 | not-applicable | — | — | Renovate's TypeScript async `unwrapOrNull` helper has no Rust API equivalent. |
| unwrapOrNull returns null for error result | 446 | not-applicable | — | — | Renovate's TypeScript async `unwrapOrNull` helper has no Rust API equivalent. |

### `util/result › AsyncResult › Transforming`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| transforms AsyncResult to pure value | 453 | not-applicable | — | — | Renovate's TypeScript `AsyncResult.transform` helper is not implemented as a Rust API; Rust uses async `Result` directly. |
| transforms AsyncResult to Result | 460 | not-applicable | — | — | Renovate's TypeScript `AsyncResult.transform` helper is not implemented as a Rust API; Rust uses async `Result` directly. |
| transforms Result to AsyncResult | 467 | not-applicable | — | — | Renovate's TypeScript sync-to-async `Result` transform helper has no Rust API equivalent. |
| transforms AsyncResult to AsyncResult | 474 | not-applicable | — | — | Renovate's TypeScript `AsyncResult.transform` helper is not implemented as a Rust API; Rust uses async `Result` directly. |
| skips transform for failed promises | 481 | not-applicable | — | — | Renovate's TypeScript `AsyncResult.transform` helper is not implemented as a Rust API. |
| asyncronously transforms successfull promise to value | 488 | not-applicable | — | — | Renovate's TypeScript `AsyncResult.transform` helper is not implemented as a Rust API. |
| asynchronously transforms successful AsyncResult to Result | 495 | not-applicable | — | — | Renovate's TypeScript `AsyncResult.transform` helper is not implemented as a Rust API. |
| asynchronously transforms value to value | 502 | not-applicable | — | — | Renovate's TypeScript async `Result.transform` helper is not implemented as a Rust API. |
| asynchronously transforms value to Result | 509 | not-applicable | — | — | Renovate's TypeScript async `Result.transform` helper is not implemented as a Rust API. |
| skips async transform for error Result | 516 | not-applicable | — | — | Renovate's TypeScript async `Result.transform` helper is not implemented as a Rust API. |
| skips async transform for rejected promise | 524 | not-applicable | — | — | Renovate's TypeScript `AsyncResult.transform` helper is not implemented as a Rust API. |
| re-wraps error thrown via unwrapping in async transform | 531 | not-applicable | — | — | Renovate's JavaScript uncaught-transform sentinel behavior has no Rust API equivalent. |
| handles error thrown on Result async transform | 541 | not-applicable | — | — | Renovate's JavaScript logger side effect for rejected async transforms has no Rust API equivalent. |
| handles error thrown on promise transform | 553 | not-applicable | — | — | Renovate's JavaScript logger side effect for thrown async transform callbacks has no Rust API equivalent. |
| handles error thrown on promise async transform | 567 | not-applicable | — | — | Renovate's JavaScript logger side effect for rejected async transform callbacks has no Rust API equivalent. |
| accumulates error types into union type during chained transform | 579 | not-applicable | — | — | Renovate's TypeScript compile-time union accumulation behavior has no Rust API equivalent. |
| asynchronously transforms Result to zod values | 598 | not-applicable | — | — | Renovate's Zod `safeParse` integration has no Rust API equivalent. |
| transforms AsyncResult to zod values | 606 | not-applicable | — | — | Renovate's Zod `safeParse` integration has no Rust API equivalent. |

### `util/result › AsyncResult › Catch`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| converts error to AsyncResult | 616 | not-applicable | — | — | Renovate's TypeScript `AsyncResult.catch` helper is not implemented as a Rust API. |
| converts error to Promise | 622 | not-applicable | — | — | Renovate's TypeScript `AsyncResult.catch` helper is not implemented as a Rust API. |
| handles error thrown in Promise result | 629 | not-applicable | — | — | Renovate's JavaScript promise rejection wrapping has no Rust API equivalent. |
| converts AsyncResult error to Result | 635 | not-applicable | — | — | Renovate's TypeScript `AsyncResult.catch` helper is not implemented as a Rust API. |

### `util/result › Parsing`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses Zod schema by piping from AsyncResult | 645 | not-applicable | — | — | Renovate's Zod parsing helper on `AsyncResult` has no Rust API equivalent. |
| handles uncaught error thrown in the steps before parsing | 660 | not-applicable | — | — | Renovate's JavaScript uncaught-transform sentinel behavior has no Rust API equivalent. |

### `util/result › Handlers`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| supports value handlers | 672 | not-applicable | — | — | Renovate's TypeScript async `onValue` handler helper is not implemented as a Rust API. |
| supports error handlers | 678 | not-applicable | — | — | Renovate's TypeScript async `onError` handler helper is not implemented as a Rust API. |
| handles error thrown in value handler | 684 | not-applicable | — | — | Renovate's JavaScript thrown-value handler behavior has no Rust API equivalent. |
| handles error thrown in error handler | 691 | not-applicable | — | — | Renovate's JavaScript thrown-value handler behavior has no Rust API equivalent. |

---

