# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/platform/gerrit/client.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/gerrit/client.spec.ts
**Total tests:** 43 | **Ported:** 0 | **Actionable:** 43 | **Status:** pending

### `getGerritVersion()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns version | 24 | pending | — | — | —|

### `getRepos()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns repos | 39 | pending | — | — | —|

### `getProjectInfo()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| inactive | 56 | pending | — | — | —|
| active | 74 | pending | — | — | —|

### `getBranchInfo()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| info | 96 | pending | — | — | —|

### `findChanges()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| footer:Renovate-Branch=dependency-xyz | 113 | pending | — | — | —|
| sets query.n as 1 if a single change is requested | 191 | pending | — | — | —|
| sets query.n as 50 if pageLimit is not provided | 206 | pending | — | — | —|
| sets query.n with pageLimit if provided | 219 | pending | — | — | —|
| sets query.S with startOffset if provided | 233 | pending | — | — | —|
| sets query.S as 0 if startOffset is not provided | 247 | pending | — | — | —|
| handles pagination automatically | 260 | pending | — | — | —|
| handles pagination with startOffset | 305 | pending | — | — | —|
| allows disabling automatic pagination | 339 | pending | — | — | —|
| sets query.o when requestDetails is provided | 361 | pending | — | — | —|

### `getChange()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| get | 381 | pending | — | — | —|

### `getMergeableInfo()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| get | 394 | pending | — | — | —|

### `abandonChange()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| abandon | 410 | pending | — | — | —|
| abandon with message | 419 | pending | — | — | —|

### `submitChange()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| submit | 434 | pending | — | — | —|

### `moveChange()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| move change to different branch | 445 | pending | — | — | —|

### `getBranchChange()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null when no changes found | 460 | pending | — | — | —|
| returns single change when only one found | 474 | pending | — | — | —|
| returns first change when multiple found without targetBranch | 492 | pending | — | — | —|
| returns matching change when targetBranch specified and match found | 514 | pending | — | — | —|
| returns first change when targetBranch specified but no match found | 537 | pending | — | — | —|

### `getMessages()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| no messages | 562 | pending | — | — | —|
| with messages | 570 | pending | — | — | —|

### `addMessage()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| add with tag | 590 | pending | — | — | —|
| add without tag | 602 | pending | — | — | —|
| add too big message | 613 | pending | — | — | —|

### `checkForExistingMessage()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| msg not found | 633 | pending | — | — | —|
| msg found | 643 | pending | — | — | —|

### `addMessageIfNotAlreadyExists()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| msg not found | 662 | pending | — | — | —|
| msg already exists | 685 | pending | — | — | —|

### `setLabel()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| setLabel | 704 | pending | — | — | —|

### `setHashtags()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| add hashtags | 719 | pending | — | — | —|
| remove hashtags | 731 | pending | — | — | —|
| add and remove hashtags in single call | 743 | pending | — | — | —|
| does nothing when no hashtags provided | 759 | pending | — | — | —|

### `addReviewer()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| add | 770 | pending | — | — | —|

### `addAssignee()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| add | 783 | pending | — | — | —|

### `getFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| getFile() - repo and branch | 795 | pending | — | — | —|

---

