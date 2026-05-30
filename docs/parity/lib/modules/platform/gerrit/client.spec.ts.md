# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/platform/gerrit/client.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/gerrit/client.spec.ts
**Total tests:** 43 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `getGerritVersion()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns version | 24 | not-applicable | — | — | No corresponding Rust source|

### `getRepos()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns repos | 39 | not-applicable | — | — | No corresponding Rust source|

### `getProjectInfo()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| inactive | 56 | not-applicable | — | — | No corresponding Rust source|
| active | 74 | not-applicable | — | — | No corresponding Rust source|

### `getBranchInfo()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| info | 96 | not-applicable | — | — | No corresponding Rust source|

### `findChanges()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| footer:Renovate-Branch=dependency-xyz | 113 | not-applicable | — | — | No corresponding Rust source|
| sets query.n as 1 if a single change is requested | 191 | not-applicable | — | — | No corresponding Rust source|
| sets query.n as 50 if pageLimit is not provided | 206 | not-applicable | — | — | No corresponding Rust source|
| sets query.n with pageLimit if provided | 219 | not-applicable | — | — | No corresponding Rust source|
| sets query.S with startOffset if provided | 233 | not-applicable | — | — | No corresponding Rust source|
| sets query.S as 0 if startOffset is not provided | 247 | not-applicable | — | — | No corresponding Rust source|
| handles pagination automatically | 260 | not-applicable | — | — | No corresponding Rust source|
| handles pagination with startOffset | 305 | not-applicable | — | — | No corresponding Rust source|
| allows disabling automatic pagination | 339 | not-applicable | — | — | No corresponding Rust source|
| sets query.o when requestDetails is provided | 361 | not-applicable | — | — | No corresponding Rust source|

### `getChange()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| get | 381 | not-applicable | — | — | No corresponding Rust source|

### `getMergeableInfo()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| get | 394 | not-applicable | — | — | No corresponding Rust source|

### `abandonChange()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| abandon | 410 | not-applicable | — | — | No corresponding Rust source|
| abandon with message | 419 | not-applicable | — | — | No corresponding Rust source|

### `submitChange()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| submit | 434 | not-applicable | — | — | No corresponding Rust source|

### `moveChange()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| move change to different branch | 445 | not-applicable | — | — | No corresponding Rust source|

### `getBranchChange()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null when no changes found | 460 | not-applicable | — | — | No corresponding Rust source|
| returns single change when only one found | 474 | not-applicable | — | — | No corresponding Rust source|
| returns first change when multiple found without targetBranch | 492 | not-applicable | — | — | No corresponding Rust source|
| returns matching change when targetBranch specified and match found | 514 | not-applicable | — | — | No corresponding Rust source|
| returns first change when targetBranch specified but no match found | 537 | not-applicable | — | — | No corresponding Rust source|

### `getMessages()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| no messages | 562 | not-applicable | — | — | No corresponding Rust source|
| with messages | 570 | not-applicable | — | — | No corresponding Rust source|

### `addMessage()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| add with tag | 590 | not-applicable | — | — | No corresponding Rust source|
| add without tag | 602 | not-applicable | — | — | No corresponding Rust source|
| add too big message | 613 | not-applicable | — | — | No corresponding Rust source|

### `checkForExistingMessage()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| msg not found | 633 | not-applicable | — | — | No corresponding Rust source|
| msg found | 643 | not-applicable | — | — | No corresponding Rust source|

### `addMessageIfNotAlreadyExists()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| msg not found | 662 | not-applicable | — | — | No corresponding Rust source|
| msg already exists | 685 | not-applicable | — | — | No corresponding Rust source|

### `setLabel()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| setLabel | 704 | not-applicable | — | — | No corresponding Rust source|

### `setHashtags()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| add hashtags | 719 | not-applicable | — | — | No corresponding Rust source|
| remove hashtags | 731 | not-applicable | — | — | No corresponding Rust source|
| add and remove hashtags in single call | 743 | not-applicable | — | — | No corresponding Rust source|
| does nothing when no hashtags provided | 759 | not-applicable | — | — | No corresponding Rust source|

### `addReviewer()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| add | 770 | not-applicable | — | — | No corresponding Rust source|

### `addAssignee()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| add | 783 | not-applicable | — | — | No corresponding Rust source|

### `getFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| getFile() - repo and branch | 795 | not-applicable | — | — | No corresponding Rust source|

---

