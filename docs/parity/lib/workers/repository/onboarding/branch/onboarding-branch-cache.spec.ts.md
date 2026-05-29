# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/repository/onboarding/branch/onboarding-branch-cache.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/onboarding/branch/onboarding-branch-cache.spec.ts
**Total tests:** 20 | **Ported:** 0 | **Actionable:** 20 | **Status:** pending

### `workers/repository/onboarding/branch/onboarding-branch-cache › setOnboardingCache`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| does not create new cache | 24 | pending | — | — | —|
| sets new cache | 31 | pending | — | — | —|
| updates old cache | 45 | pending | — | — | —|

### `workers/repository/onboarding/branch/onboarding-branch-cache › deleteOnboardingCache`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| deletes cache | 68 | pending | — | — | —|

### `workers/repository/onboarding/branch/onboarding-branch-cache › hasOnboardingBranchChanged()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| return true if cache is absent | 84 | pending | — | — | —|
| returns true | 92 | pending | — | — | —|
| returns false | 108 | pending | — | — | —|
| returns false when branch is modified but has not changed since last run | 124 | pending | — | — | —|

### `workers/repository/onboarding/branch/onboarding-branch-cache › isOnboardingBranchModified()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| falls back to git if cache is absent | 142 | pending | — | — | —|
| falls back to git if onboarding branch is updated | 153 | pending | — | — | —|
| returns cached value | 172 | pending | — | — | —|

### `workers/repository/onboarding/branch/onboarding-branch-cache › isOnboardingBranchConflicted()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| falls back to git if cache is absent | 192 | pending | — | — | —|
| falls back to git if default branch is updated | 203 | pending | — | — | —|
| falls back to git if onboarding branch is modified | 222 | pending | — | — | —|
| returns cached value | 241 | pending | — | — | —|

### `workers/repository/onboarding/branch/onboarding-branch-cache › getOnboardingFileNameFromCache()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns cached value | 261 | pending | — | — | —|
| returns undefined | 271 | pending | — | — | —|

### `workers/repository/onboarding/branch/onboarding-branch-cache › getOnboardingConfigFromCache()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns cached value | 278 | pending | — | — | —|
| returns undefined | 288 | pending | — | — | —|

### `workers/repository/onboarding/branch/onboarding-branch-cache › setOnboardingConfigDetails()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns cached value | 295 | pending | — | — | —|

---
