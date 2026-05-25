# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/util/result.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/result.spec.ts
**Total tests:** 85 | **Ported:** 0 | **Actionable:** 85 | **Status:** pending

### `util/result › Result › constructors`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| ok result | 12 | pending | — | — | — |
| error result | 22 | pending | — | — | — |

### `util/result › Result › Wrapping`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| wraps callback returning value | 34 | pending | — | — | — |
| handles throw in callback | 39 | pending | — | — | — |
| wraps callback returning promise | 46 | pending | — | — | — |
| wraps callback returning failed promise | 51 | pending | — | — | — |
| wraps nullable callback | 57 | pending | — | — | — |
| wraps nullable callback null | 65 | pending | — | — | — |
| wraps nullable callback undefined | 70 | pending | — | — | — |
| distincts between null and undefined callback results | 75 | pending | — | — | — |
| handles nullable callback error | 84 | pending | — | — | — |
| wraps pure nullable value | 91 | pending | — | — | — |
| wraps nullable value null | 96 | pending | — | — | — |
| wraps nullable value undefined | 101 | pending | — | — | — |
| wraps zod parse result | 106 | pending | — | — | — |

### `util/result › Result › Unwrapping`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| unwraps successful value | 120 | pending | — | — | — |
| unwraps error value | 128 | pending | — | — | — |
| skips fallback for successful value | 136 | pending | — | — | — |
| uses fallback for error value | 141 | pending | — | — | — |
| unwrapOr throws uncaught transform error | 146 | pending | — | — | — |
| unwrap throws uncaught transform error | 157 | pending | — | — | — |
| returns ok-value for unwrapOrThrow | 168 | pending | — | — | — |
| throws error for unwrapOrThrow on error result | 173 | pending | — | — | — |
| unwrapOrNull returns value for ok-result | 178 | pending | — | — | — |
| unwrapOrNull returns null for error result | 183 | pending | — | — | — |
| unwrapOrNull throws uncaught transform error | 188 | pending | — | — | — |

### `util/result › Result › Transforming`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| transforms value to value | 201 | pending | — | — | — |
| transforms value to Result | 206 | pending | — | — | — |
| skips transform for error Result | 213 | pending | — | — | — |
| logs and returns error on transform failure | 220 | pending | — | — | — |
| automatically converts zod values | 232 | pending | — | — | — |

### `util/result › Result › Catch`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| bypasses ok result | 240 | pending | — | — | — |
| bypasses uncaught transform errors | 246 | pending | — | — | — |
| converts error to Result | 254 | pending | — | — | — |
| handles error thrown in catch function | 260 | pending | — | — | — |

### `util/result › Result › Parsing`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses Zod schema | 269 | pending | — | — | — |
| parses Zod schema by piping from Result | 302 | pending | — | — | — |

### `util/result › Result › Handlers`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| supports value handlers | 319 | pending | — | — | — |
| supports error handlers | 325 | pending | — | — | — |
| handles error thrown in value handler | 331 | pending | — | — | — |
| handles error thrown in error handler | 338 | pending | — | — | — |

### `util/result › AsyncResult › Wrapping`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| wraps promise | 349 | pending | — | — | — |
| wraps Result promise | 356 | pending | — | — | — |
| handles rejected promise | 363 | pending | — | — | — |
| wraps nullable promise | 370 | pending | — | — | — |
| wraps promise returning null | 378 | pending | — | — | — |
| wraps promise returning undefined | 383 | pending | — | — | — |
| distincts between null and undefined promise results | 388 | pending | — | — | — |
| handles rejected nullable promise | 398 | pending | — | — | — |

### `util/result › AsyncResult › Unwrapping`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| unwraps successful AsyncResult | 405 | pending | — | — | — |
| unwraps error AsyncResult | 413 | pending | — | — | — |
| skips fallback for successful AsyncResult | 421 | pending | — | — | — |
| uses fallback for error AsyncResult | 426 | pending | — | — | — |
| returns ok-value for unwrapOrThrow | 431 | pending | — | — | — |
| rejects for error for unwrapOrThrow | 436 | pending | — | — | — |
| unwrapOrNull returns value for ok-result | 441 | pending | — | — | — |
| unwrapOrNull returns null for error result | 446 | pending | — | — | — |

### `util/result › AsyncResult › Transforming`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| transforms AsyncResult to pure value | 453 | pending | — | — | — |
| transforms AsyncResult to Result | 460 | pending | — | — | — |
| transforms Result to AsyncResult | 467 | pending | — | — | — |
| transforms AsyncResult to AsyncResult | 474 | pending | — | — | — |
| skips transform for failed promises | 481 | pending | — | — | — |
| asyncronously transforms successfull promise to value | 488 | pending | — | — | — |
| asynchronously transforms successful AsyncResult to Result | 495 | pending | — | — | — |
| asynchronously transforms value to value | 502 | pending | — | — | — |
| asynchronously transforms value to Result | 509 | pending | — | — | — |
| skips async transform for error Result | 516 | pending | — | — | — |
| skips async transform for rejected promise | 524 | pending | — | — | — |
| re-wraps error thrown via unwrapping in async transform | 531 | pending | — | — | — |
| handles error thrown on Result async transform | 541 | pending | — | — | — |
| handles error thrown on promise transform | 553 | pending | — | — | — |
| handles error thrown on promise async transform | 567 | pending | — | — | — |
| accumulates error types into union type during chained transform | 579 | pending | — | — | — |
| asynchronously transforms Result to zod values | 598 | pending | — | — | — |
| transforms AsyncResult to zod values | 606 | pending | — | — | — |

### `util/result › AsyncResult › Catch`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| converts error to AsyncResult | 616 | pending | — | — | — |
| converts error to Promise | 622 | pending | — | — | — |
| handles error thrown in Promise result | 629 | pending | — | — | — |
| converts AsyncResult error to Result | 635 | pending | — | — | — |

### `util/result › Parsing`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses Zod schema by piping from AsyncResult | 645 | pending | — | — | — |
| handles uncaught error thrown in the steps before parsing | 660 | pending | — | — | — |

### `util/result › Handlers`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| supports value handlers | 672 | pending | — | — | — |
| supports error handlers | 678 | pending | — | — | — |
| handles error thrown in value handler | 684 | pending | — | — | — |
| handles error thrown in error handler | 691 | pending | — | — | — |

---

