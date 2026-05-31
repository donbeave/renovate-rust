# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/platform/gerrit/client.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/gerrit/client.spec.ts
**Total tests:** 43 | **Ported:** 0 | **Actionable:** 43 | **Status:** pending

### `getGerritVersion()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns version | 24 | pending | — | — | No corresponding Rust source|

### `getRepos()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns repos | 39 | pending | — | — | No corresponding Rust source|

### `getProjectInfo()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| inactive | 56 | pending | — | — | No corresponding Rust source|
| active | 74 | pending | — | — | No corresponding Rust source|

### `getBranchInfo()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| info | 96 | pending | — | — | No corresponding Rust source|

### `findChanges()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| footer:Renovate-Branch=dependency-xyz | 113 | pending | — | — | No corresponding Rust source|
| sets query.n as 1 if a single change is requested | 191 | pending | — | — | No corresponding Rust source|
| sets query.n as 50 if pageLimit is not provided | 206 | pending | — | — | No corresponding Rust source|
| sets query.n with pageLimit if provided | 219 | pending | — | — | No corresponding Rust source|
| sets query.S with startOffset if provided | 233 | pending | — | — | No corresponding Rust source|
| sets query.S as 0 if startOffset is not provided | 247 | pending | — | — | No corresponding Rust source|
| handles pagination automatically | 260 | pending | — | — | No corresponding Rust source|
| handles pagination with startOffset | 305 | pending | — | — | No corresponding Rust source|
| allows disabling automatic pagination | 339 | pending | — | — | No corresponding Rust source|
| sets query.o when requestDetails is provided | 361 | pending | — | — | No corresponding Rust source|

### `getChange()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| get | 381 | pending | — | — | No corresponding Rust source|

### `getMergeableInfo()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| get | 394 | pending | — | — | No corresponding Rust source|

### `abandonChange()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| abandon | 410 | pending | — | — | No corresponding Rust source|
| abandon with message | 419 | pending | — | — | No corresponding Rust source|

### `submitChange()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| submit | 434 | pending | — | — | No corresponding Rust source|

### `moveChange()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| move change to different branch | 445 | pending | — | — | No corresponding Rust source|

### `getBranchChange()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null when no changes found | 460 | pending | — | — | No corresponding Rust source|
| returns single change when only one found | 474 | pending | — | — | No corresponding Rust source|
| returns first change when multiple found without targetBranch | 492 | pending | — | — | No corresponding Rust source|
| returns matching change when targetBranch specified and match found | 514 | pending | — | — | No corresponding Rust source|
| returns first change when targetBranch specified but no match found | 537 | pending | — | — | No corresponding Rust source|

### `getMessages()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| no messages | 562 | pending | — | — | No corresponding Rust source|
| with messages | 570 | pending | — | — | No corresponding Rust source|

### `addMessage()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| add with tag | 590 | pending | — | — | No corresponding Rust source|
| add without tag | 602 | pending | — | — | No corresponding Rust source|
| add too big message | 613 | pending | — | — | No corresponding Rust source|

### `checkForExistingMessage()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| msg not found | 633 | pending | — | — | No corresponding Rust source|
| msg found | 643 | pending | — | — | No corresponding Rust source|

### `addMessageIfNotAlreadyExists()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| msg not found | 662 | pending | — | — | No corresponding Rust source|
| msg already exists | 685 | pending | — | — | No corresponding Rust source|

### `setLabel()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| setLabel | 704 | pending | — | — | No corresponding Rust source|

### `setHashtags()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| add hashtags | 719 | pending | — | — | No corresponding Rust source|
| remove hashtags | 731 | pending | — | — | No corresponding Rust source|
| add and remove hashtags in single call | 743 | pending | — | — | No corresponding Rust source|
| does nothing when no hashtags provided | 759 | pending | — | — | No corresponding Rust source|

### `addReviewer()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| add | 770 | pending | — | — | No corresponding Rust source|

### `addAssignee()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| add | 783 | pending | — | — | No corresponding Rust source|

### `getFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| getFile() - repo and branch | 795 | pending | — | — | No corresponding Rust source|

---

