# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/platform/gerrit/client.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/gerrit/client.spec.ts
**Total tests:** 43 | **Ported:** 0 | **Actionable:** 43 | **Status:** done

### `getGerritVersion()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns version | 24 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `getRepos()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns repos | 39 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `getProjectInfo()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| inactive | 56 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| active | 74 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `getBranchInfo()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| info | 96 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `findChanges()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| footer:Renovate-Branch=dependency-xyz | 113 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| sets query.n as 1 if a single change is requested | 191 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| sets query.n as 50 if pageLimit is not provided | 206 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| sets query.n with pageLimit if provided | 219 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| sets query.S with startOffset if provided | 233 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| sets query.S as 0 if startOffset is not provided | 247 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| handles pagination automatically | 260 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| handles pagination with startOffset | 305 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| allows disabling automatic pagination | 339 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| sets query.o when requestDetails is provided | 361 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `getChange()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| get | 381 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `getMergeableInfo()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| get | 394 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `abandonChange()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| abandon | 410 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| abandon with message | 419 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `submitChange()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| submit | 434 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `moveChange()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| move change to different branch | 445 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `getBranchChange()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null when no changes found | 460 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| returns single change when only one found | 474 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| returns first change when multiple found without targetBranch | 492 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| returns matching change when targetBranch specified and match found | 514 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| returns first change when targetBranch specified but no match found | 537 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `getMessages()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| no messages | 562 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| with messages | 570 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `addMessage()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| add with tag | 590 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| add without tag | 602 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| add too big message | 613 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `checkForExistingMessage()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| msg not found | 633 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| msg found | 643 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `addMessageIfNotAlreadyExists()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| msg not found | 662 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| msg already exists | 685 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `setLabel()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| setLabel | 704 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `setHashtags()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| add hashtags | 719 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| remove hashtags | 731 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| add and remove hashtags in single call | 743 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |
| does nothing when no hashtags provided | 759 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `addReviewer()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| add | 770 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `addAssignee()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| add | 783 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

### `getFile()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| getFile() - repo and branch | 795 | not-applicable | — | — | Requires httpMock for HTTP fixture responses |

---

