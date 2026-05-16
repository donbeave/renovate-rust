# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/modules/datasource/azure-pipelines-tasks/index.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/datasource/azure-pipelines-tasks/index.spec.ts
**Total tests:** 10 | **Ported:** 1 | **Actionable:** 1 | **Status:** ported

### `modules/datasource/azure-pipelines-tasks/index`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns null for unknown task | 21 | not-applicable | — | — | Renovate's Azure Pipelines Tasks `getReleases` release-list/null contract and process-wide mocked fallback registries are not implemented in Rust; Rust exposes a latest-version summary over the fallback JSON files. |
| supports built-in tasks | 36 | not-applicable | — | — | Renovate's Azure Pipelines Tasks `getReleases` release-list response shape is not implemented in Rust; Rust exposes a latest-version summary over the fallback JSON files. |
| supports marketplace tasks | 49 | not-applicable | — | — | Renovate's Azure Pipelines Tasks `getReleases` release-list response shape is not implemented in Rust; Rust exposes a latest-version summary over the fallback JSON files. |
| is case insensitive | 64 | not-applicable | — | — | Renovate's Azure Pipelines Tasks `getReleases` release-list response shape is not implemented in Rust; Rust exposes a latest-version summary over the fallback JSON files. |
| returns organization task with single version | 77 | not-applicable | — | — | Azure DevOps organization task API lookup, hostRules authentication, changelog mapping, and deprecation metadata are not implemented in the Rust datasource. |
| identifies task based on task id | 112 | not-applicable | — | — | Azure DevOps organization task API lookup and task identity matching are not implemented in the Rust datasource. |
| identifies task based on contributionIdentifier and id | 134 | not-applicable | — | — | Azure DevOps organization task API lookup and contributionIdentifier matching are not implemented in the Rust datasource. |
| identifies task based on contributionIdentifier and name | 157 | not-applicable | — | — | Azure DevOps organization task API lookup and contributionIdentifier matching are not implemented in the Rust datasource. |
| returns organization task with multiple versions | 180 | not-applicable | — | — | Azure DevOps organization task API lookup, changelog mapping, and deprecation metadata are not implemented in the Rust datasource. |

### `modules/datasource/azure-pipelines-tasks/index › compare semver`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| when versions is $a | 222 | ported | `azure_pipelines_tasks.rs` | `cmp_version_sorts_semver_cases` | — |

---

