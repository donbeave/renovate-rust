# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/util/result.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/result.spec.ts
**Total tests:** 85 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable-applicable-applicable

### `util/result › Result › constructors`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| ok result | 12 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||
| error result | 22 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||

### `util/result › Result › Wrapping`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| wraps callback returning value | 34 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||
| handles throw in callback | 39 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||
| wraps callback returning promise | 46 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||
| wraps callback returning failed promise | 51 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||
| wraps nullable callback | 57 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||
| wraps nullable callback null | 65 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||
| wraps nullable callback undefined | 70 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||
| distincts between null and undefined callback results | 75 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||
| handles nullable callback error | 84 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||
| wraps pure nullable value | 91 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||
| wraps nullable value null | 96 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||
| wraps nullable value undefined | 101 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||
| wraps zod parse result | 106 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||

### `util/result › Result › Unwrapping`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| unwraps successful value | 120 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||
| unwraps error value | 128 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||
| skips fallback for successful value | 136 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||
| uses fallback for error value | 141 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||
| unwrapOr throws uncaught transform error | 146 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||
| unwrap throws uncaught transform error | 157 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||
| returns ok-value for unwrapOrThrow | 168 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||
| throws error for unwrapOrThrow on error result | 173 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||
| unwrapOrNull returns value for ok-result | 178 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||
| unwrapOrNull returns null for error result | 183 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||
| unwrapOrNull throws uncaught transform error | 188 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||

### `util/result › Result › Transforming`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| transforms value to value | 201 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||
| transforms value to Result | 206 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||
| skips transform for error Result | 213 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||
| logs and returns error on transform failure | 220 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||
| automatically converts zod values | 232 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||

### `util/result › Result › Catch`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| bypasses ok result | 240 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||
| bypasses uncaught transform errors | 246 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||
| converts error to Result | 254 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||
| handles error thrown in catch function | 260 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||

### `util/result › Result › Parsing`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses Zod schema | 269 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||
| parses Zod schema by piping from Result | 302 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||

### `util/result › Result › Handlers`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| supports value handlers | 319 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||
| supports error handlers | 325 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||
| handles error thrown in value handler | 331 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||
| handles error thrown in error handler | 338 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||

### `util/result › AsyncResult › Wrapping`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| wraps promise | 349 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||
| wraps Result promise | 356 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||
| handles rejected promise | 363 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||
| wraps nullable promise | 370 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||
| wraps promise returning null | 378 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||
| wraps promise returning undefined | 383 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||
| distincts between null and undefined promise results | 388 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||
| handles rejected nullable promise | 398 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||

### `util/result › AsyncResult › Unwrapping`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| unwraps successful AsyncResult | 405 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||
| unwraps error AsyncResult | 413 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||
| skips fallback for successful AsyncResult | 421 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||
| uses fallback for error AsyncResult | 426 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||
| returns ok-value for unwrapOrThrow | 431 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||
| rejects for error for unwrapOrThrow | 436 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||
| unwrapOrNull returns value for ok-result | 441 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||
| unwrapOrNull returns null for error result | 446 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||

### `util/result › AsyncResult › Transforming`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| transforms AsyncResult to pure value | 453 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||
| transforms AsyncResult to Result | 460 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||
| transforms Result to AsyncResult | 467 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||
| transforms AsyncResult to AsyncResult | 474 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||
| skips transform for failed promises | 481 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||
| asyncronously transforms successfull promise to value | 488 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||
| asynchronously transforms successful AsyncResult to Result | 495 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||
| asynchronously transforms value to value | 502 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||
| asynchronously transforms value to Result | 509 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||
| skips async transform for error Result | 516 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||
| skips async transform for rejected promise | 524 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||
| re-wraps error thrown via unwrapping in async transform | 531 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||
| handles error thrown on Result async transform | 541 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||
| handles error thrown on promise transform | 553 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||
| handles error thrown on promise async transform | 567 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||
| accumulates error types into union type during chained transform | 579 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||
| asynchronously transforms Result to zod values | 598 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||
| transforms AsyncResult to zod values | 606 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||

### `util/result › AsyncResult › Catch`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| converts error to AsyncResult | 616 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||
| converts error to Promise | 622 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||
| handles error thrown in Promise result | 629 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||
| converts AsyncResult error to Result | 635 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||

### `util/result › Parsing`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| parses Zod schema by piping from AsyncResult | 645 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||
| handles uncaught error thrown in the steps before parsing | 660 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||

### `util/result › Handlers`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| supports value handlers | 672 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||
| supports error handlers | 678 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||
| handles error thrown in value handler | 684 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||
| handles error thrown in error handler | 691 | not-applicable | — | — | TypeScript type-system test; tests TypeScript reimplementation of Rust's native Result<T,E> ||

---

