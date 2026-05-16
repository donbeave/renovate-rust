# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/constants/platform.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/constants/platform.spec.ts
**Total tests:** 8 | **Ported:** 8 | **Actionable:** 8 | **Status:** ported

### `constants/platform`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should be part of the GITEA_API_USING_HOST_TYPES | 24 | ported | `platform_constants.rs` | `gitea_api_using_host_types_include_gitea_tags_and_platform` | — |
| should be part of the FORGEJO_API_USING_HOST_TYPES | 31 | ported | `platform_constants.rs` | `forgejo_api_using_host_types_include_expected_host_types` | — |
| should be part of the GITLAB_API_USING_HOST_TYPES | 45 | ported | `platform_constants.rs` | `gitlab_api_using_host_types_include_expected_datasources_and_platform` | — |
| should be not part of the GITLAB_API_USING_HOST_TYPES | 62 | ported | `platform_constants.rs` | `gitlab_api_using_host_types_do_not_include_github` | — |
| should be part of the GITHUB_API_USING_HOST_TYPES | 66 | ported | `platform_constants.rs` | `github_api_using_host_types_include_expected_datasources_and_platform` | — |
| should be not part of the GITHUB_API_USING_HOST_TYPES | 84 | ported | `platform_constants.rs` | `github_api_using_host_types_do_not_include_gitlab` | — |
| should be part of the BITBUCKET_API_USING_HOST_TYPES | 88 | ported | `platform_constants.rs` | `bitbucket_api_using_host_types_include_bitbucket_tags_and_platform` | — |
| should be part of the BITBUCKET_SERVER_API_USING_HOST_TYPES | 95 | ported | `platform_constants.rs` | `bitbucket_server_api_using_host_types_include_server_tags_and_platform` | — |

---

