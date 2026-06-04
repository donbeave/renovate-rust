# Source Mapping — `util`

[← all groups](README.md)

**Coverage:** 33/172 in-scope files mapped (full=33 partial=0 stub=0 pending=139 out-of-scope=0 opt-out=3) across 13 modules.

### `util/_root`

| TS source | Status | Rust file(s) | Note |
|---|---|---|---|
| `lib/util/array.ts` | full | [`crates/renovate-core/src/util/array.rs`](../../../crates/renovate-core/src/util/array.rs) | — |
| `lib/util/assign-keys.ts` | full | [`crates/renovate-core/src/util.rs`](../../../crates/renovate-core/src/util.rs) | — |
| `lib/util/check-token.ts` | full | [`crates/renovate-core/src/util.rs`](../../../crates/renovate-core/src/util.rs) | — |
| `lib/util/clone.ts` | full | [`crates/renovate-core/src/util/clone.rs`](../../../crates/renovate-core/src/util/clone.rs) | — |
| `lib/util/coerce.ts` | full | [`crates/renovate-core/src/util.rs`](../../../crates/renovate-core/src/util.rs) | — |
| `lib/util/common.ts` | full | [`crates/renovate-core/src/util.rs`](../../../crates/renovate-core/src/util.rs) | — |
| `lib/util/compress.ts` | full | [`crates/renovate-core/src/util.rs`](../../../crates/renovate-core/src/util.rs) | — |
| `lib/util/date.ts` | full | [`crates/renovate-core/src/util.rs`](../../../crates/renovate-core/src/util.rs) | — |
| `lib/util/emoji.ts` | full | [`crates/renovate-core/src/util/emoji.rs`](../../../crates/renovate-core/src/util/emoji.rs) | — |
| `lib/util/env.ts` | full | [`crates/renovate-core/src/util.rs`](../../../crates/renovate-core/src/util.rs) | — |
| `lib/util/filter-map.ts` | full | [`crates/renovate-core/src/util.rs`](../../../crates/renovate-core/src/util.rs) | — |
| `lib/util/fingerprint.ts` | full | [`crates/renovate-core/src/util.rs`](../../../crates/renovate-core/src/util.rs) | — |
| `lib/util/hash.ts` | full | [`crates/renovate-core/src/util/hash.rs`](../../../crates/renovate-core/src/util/hash.rs) | — |
| `lib/util/host-rules.ts` | full | [`crates/renovate-core/src/util/host_rules.rs`](../../../crates/renovate-core/src/util/host_rules.rs) | — |
| `lib/util/html.ts` | opt-out | — | Thin wrapper around node-html-parser specific to TypeScript runtime APIs; no Rust equivalent in the current implementation set. |
| `lib/util/ignore.ts` | full | [`crates/renovate-core/src/util.rs`](../../../crates/renovate-core/src/util.rs) | — |
| `lib/util/interpolator.ts` | full | [`crates/renovate-core/src/config/secrets.rs`](../../../crates/renovate-core/src/config/secrets.rs)<br>[`crates/renovate-core/src/util.rs`](../../../crates/renovate-core/src/util.rs) | — |
| `lib/util/jsonata.ts` | full | [`crates/renovate-core/src/util/jsonata.rs`](../../../crates/renovate-core/src/util/jsonata.rs) | — |
| `lib/util/lazy.ts` | full | [`crates/renovate-core/src/util.rs`](../../../crates/renovate-core/src/util.rs) | — |
| `lib/util/markdown.ts` | full | [`crates/renovate-core/src/util.rs`](../../../crates/renovate-core/src/util.rs) | — |
| `lib/util/mask.ts` | full | [`crates/renovate-core/src/util.rs`](../../../crates/renovate-core/src/util.rs) | — |
| `lib/util/memoize.ts` | full | [`crates/renovate-core/src/util.rs`](../../../crates/renovate-core/src/util.rs) | — |
| `lib/util/minimatch.ts` | full | [`crates/renovate-core/src/util/minimatch.rs`](../../../crates/renovate-core/src/util/minimatch.rs) | — |
| `lib/util/modules.ts` | opt-out | — | Node.js module loading helper using filesystem discovery and dynamic imports; Rust uses compile-time module wiring and does not expose a generic runtime module loader. |
| `lib/util/mutex.ts` | opt-out | — | TypeScript async mutex API uses async-mutex with namespaced runtime locks and timeout-based acquire; Rust uses a scoped mutex type without this namespace/awaited lock contract. |
| `lib/util/number.ts` | full | [`crates/renovate-core/src/util.rs`](../../../crates/renovate-core/src/util.rs) | — |
| `lib/util/object.ts` | full | [`crates/renovate-core/src/util.rs`](../../../crates/renovate-core/src/util.rs) | — |
| `lib/util/pretty-time.ts` | full | [`crates/renovate-core/src/util.rs`](../../../crates/renovate-core/src/util.rs) | — |
| `lib/util/promises.ts` | full | [`crates/renovate-core/src/util/promises.rs`](../../../crates/renovate-core/src/util/promises.rs) | — |
| `lib/util/range.ts` | full | [`crates/renovate-core/src/util.rs`](../../../crates/renovate-core/src/util.rs) | — |
| `lib/util/regex.ts` | full | [`crates/renovate-core/src/util/regex.rs`](../../../crates/renovate-core/src/util/regex.rs) | — |
| `lib/util/result.ts` | pending | — | — |
| `lib/util/s3.ts` | pending | — | — |
| `lib/util/sample.ts` | full | [`crates/renovate-core/src/util.rs`](../../../crates/renovate-core/src/util.rs) | — |
| `lib/util/sanitize.ts` | pending | — | — |
| `lib/util/split.ts` | full | [`crates/renovate-core/src/util.rs`](../../../crates/renovate-core/src/util.rs) | — |
| `lib/util/stats.ts` | pending | — | — |
| `lib/util/streams.ts` | pending | — | — |
| `lib/util/string-match.ts` | full | [`crates/renovate-core/src/string_match.rs`](../../../crates/renovate-core/src/string_match.rs) | — |
| `lib/util/string.ts` | pending | — | — |
| `lib/util/stringify.ts` | pending | — | — |
| `lib/util/timestamp.ts` | pending | — | — |
| `lib/util/toml.ts` | pending | — | — |
| `lib/util/unicode.ts` | full | [`crates/renovate-core/src/util.rs`](../../../crates/renovate-core/src/util.rs) | — |
| `lib/util/uniq.ts` | full | [`crates/renovate-core/src/util.rs`](../../../crates/renovate-core/src/util.rs) | — |
| `lib/util/url.ts` | pending | — | — |
| `lib/util/yaml.ts` | pending | — | — |

### `util/cache`

| TS source | Status | Rust file(s) | Note |
|---|---|---|---|
| `lib/util/cache/memory/index.ts` | pending | — | — |
| `lib/util/cache/package/backend.ts` | pending | — | — |
| `lib/util/cache/package/impl/base.ts` | pending | — | — |
| `lib/util/cache/package/impl/file.ts` | pending | — | — |
| `lib/util/cache/package/impl/redis.ts` | pending | — | — |
| `lib/util/cache/package/impl/sqlite.ts` | pending | — | — |
| `lib/util/cache/package/index.ts` | pending | — | — |
| `lib/util/cache/package/key.ts` | pending | — | — |
| `lib/util/cache/package/namespaces.ts` | pending | — | — |
| `lib/util/cache/package/ttl.ts` | pending | — | — |
| `lib/util/cache/package/types.ts` | pending | — | — |
| `lib/util/cache/package/with-cache.ts` | pending | — | — |
| `lib/util/cache/repository/common.ts` | pending | — | — |
| `lib/util/cache/repository/http-cache.ts` | pending | — | — |
| `lib/util/cache/repository/impl/base.ts` | pending | — | — |
| `lib/util/cache/repository/impl/cache-factory.ts` | pending | — | — |
| `lib/util/cache/repository/impl/local.ts` | pending | — | — |
| `lib/util/cache/repository/impl/null.ts` | pending | — | — |
| `lib/util/cache/repository/impl/s3.ts` | pending | — | — |
| `lib/util/cache/repository/index.ts` | pending | — | — |
| `lib/util/cache/repository/init.ts` | pending | — | — |
| `lib/util/cache/repository/schema.ts` | pending | — | — |
| `lib/util/cache/repository/types.ts` | pending | — | — |

### `util/exec`

| TS source | Status | Rust file(s) | Note |
|---|---|---|---|
| `lib/util/exec/common.ts` | pending | — | — |
| `lib/util/exec/containerbase.ts` | pending | — | — |
| `lib/util/exec/docker/index.ts` | pending | — | — |
| `lib/util/exec/env.ts` | pending | — | — |
| `lib/util/exec/exec-error.ts` | pending | — | — |
| `lib/util/exec/hermit.ts` | pending | — | — |
| `lib/util/exec/index.ts` | pending | — | — |
| `lib/util/exec/types.ts` | pending | — | — |
| `lib/util/exec/utils.ts` | pending | — | — |

### `util/fs`

| TS source | Status | Rust file(s) | Note |
|---|---|---|---|
| `lib/util/fs/index.ts` | pending | — | — |
| `lib/util/fs/util.ts` | pending | — | — |

### `util/git`

| TS source | Status | Rust file(s) | Note |
|---|---|---|---|
| `lib/util/git/auth.ts` | pending | — | — |
| `lib/util/git/author.ts` | pending | — | — |
| `lib/util/git/behind-base-branch-cache.ts` | pending | — | — |
| `lib/util/git/config.ts` | pending | — | — |
| `lib/util/git/conflicts-cache.ts` | pending | — | — |
| `lib/util/git/error.ts` | pending | — | — |
| `lib/util/git/index.ts` | pending | — | — |
| `lib/util/git/instrument.ts` | pending | — | — |
| `lib/util/git/modified-cache.ts` | pending | — | — |
| `lib/util/git/pristine.ts` | pending | — | — |
| `lib/util/git/private-key.ts` | pending | — | — |
| `lib/util/git/semantic.ts` | pending | — | — |
| `lib/util/git/set-branch-commit.ts` | pending | — | — |
| `lib/util/git/span-processor.ts` | pending | — | — |
| `lib/util/git/types.ts` | pending | — | — |
| `lib/util/git/update-date-cache.ts` | pending | — | — |
| `lib/util/git/url.ts` | pending | — | — |

### `util/github`

| TS source | Status | Rust file(s) | Note |
|---|---|---|---|
| `lib/util/github/graphql/cache-strategies/abstract-cache-strategy.ts` | pending | — | — |
| `lib/util/github/graphql/cache-strategies/memory-cache-strategy.ts` | pending | — | — |
| `lib/util/github/graphql/cache-strategies/package-cache-strategy.ts` | pending | — | — |
| `lib/util/github/graphql/datasource-fetcher.ts` | pending | — | — |
| `lib/util/github/graphql/index.ts` | pending | — | — |
| `lib/util/github/graphql/query-adapters/branches-query-adapter.ts` | pending | — | — |
| `lib/util/github/graphql/query-adapters/releases-query-adapter.ts` | pending | — | — |
| `lib/util/github/graphql/query-adapters/tags-query-adapter.ts` | pending | — | — |
| `lib/util/github/graphql/types.ts` | pending | — | — |
| `lib/util/github/graphql/util.ts` | pending | — | — |
| `lib/util/github/tags.ts` | pending | — | — |
| `lib/util/github/types.ts` | pending | — | — |
| `lib/util/github/url.ts` | pending | — | — |

### `util/http`

| TS source | Status | Rust file(s) | Note |
|---|---|---|---|
| `lib/util/http/auth.ts` | pending | — | — |
| `lib/util/http/bitbucket-server.ts` | pending | — | — |
| `lib/util/http/bitbucket.ts` | pending | — | — |
| `lib/util/http/cache/abstract-http-cache-provider.ts` | pending | — | — |
| `lib/util/http/cache/memory-http-cache-provider.ts` | pending | — | — |
| `lib/util/http/cache/package-http-cache-provider.ts` | pending | — | — |
| `lib/util/http/cache/repository-http-cache-provider.ts` | pending | — | — |
| `lib/util/http/cache/schema.ts` | pending | — | — |
| `lib/util/http/cache/types.ts` | pending | — | — |
| `lib/util/http/errors.ts` | pending | — | — |
| `lib/util/http/forgejo.ts` | pending | — | — |
| `lib/util/http/gerrit.ts` | pending | — | — |
| `lib/util/http/gitea.ts` | pending | — | — |
| `lib/util/http/github.ts` | pending | — | — |
| `lib/util/http/gitlab.ts` | pending | — | — |
| `lib/util/http/got.ts` | pending | — | — |
| `lib/util/http/host-rules.ts` | pending | — | — |
| `lib/util/http/http.ts` | pending | — | — |
| `lib/util/http/index.ts` | pending | — | — |
| `lib/util/http/jira.ts` | pending | — | — |
| `lib/util/http/keep-alive.ts` | pending | — | — |
| `lib/util/http/legacy.ts` | pending | — | — |
| `lib/util/http/queue.ts` | pending | — | — |
| `lib/util/http/rate-limits.ts` | pending | — | — |
| `lib/util/http/retry-after.ts` | pending | — | — |
| `lib/util/http/scm-manager.ts` | pending | — | — |
| `lib/util/http/throttle.ts` | pending | — | — |
| `lib/util/http/types.ts` | pending | — | — |
| `lib/util/http/util.ts` | pending | — | — |
| `lib/util/http/www-authenticate.ts` | pending | — | — |

### `util/json-writer`

| TS source | Status | Rust file(s) | Note |
|---|---|---|---|
| `lib/util/json-writer/code-format.ts` | pending | — | — |
| `lib/util/json-writer/editor-config.ts` | pending | — | — |
| `lib/util/json-writer/indentation-type.ts` | pending | — | — |
| `lib/util/json-writer/index.ts` | pending | — | — |
| `lib/util/json-writer/json-writer.ts` | pending | — | — |

### `util/merge-confidence`

| TS source | Status | Rust file(s) | Note |
|---|---|---|---|
| `lib/util/merge-confidence/common.ts` | pending | — | — |
| `lib/util/merge-confidence/index.ts` | pending | — | — |
| `lib/util/merge-confidence/types.ts` | pending | — | — |

### `util/package-rules`

| TS source | Status | Rust file(s) | Note |
|---|---|---|---|
| `lib/util/package-rules/base-branches.ts` | pending | — | — |
| `lib/util/package-rules/base.ts` | pending | — | — |
| `lib/util/package-rules/categories.ts` | pending | — | — |
| `lib/util/package-rules/current-age.ts` | pending | — | — |
| `lib/util/package-rules/current-value.ts` | pending | — | — |
| `lib/util/package-rules/current-version.ts` | pending | — | — |
| `lib/util/package-rules/datasources.ts` | pending | — | — |
| `lib/util/package-rules/dep-names.ts` | pending | — | — |
| `lib/util/package-rules/dep-types.ts` | pending | — | — |
| `lib/util/package-rules/files.ts` | pending | — | — |
| `lib/util/package-rules/index.ts` | pending | — | — |
| `lib/util/package-rules/jsonata.ts` | pending | — | — |
| `lib/util/package-rules/managers.ts` | pending | — | — |
| `lib/util/package-rules/matchers.ts` | pending | — | — |
| `lib/util/package-rules/merge-confidence.ts` | pending | — | — |
| `lib/util/package-rules/new-value.ts` | pending | — | — |
| `lib/util/package-rules/package-names.ts` | pending | — | — |
| `lib/util/package-rules/registryurls.ts` | pending | — | — |
| `lib/util/package-rules/repositories.ts` | pending | — | — |
| `lib/util/package-rules/sourceurls.ts` | pending | — | — |
| `lib/util/package-rules/types.ts` | pending | — | — |
| `lib/util/package-rules/update-types.ts` | pending | — | — |

### `util/schema-utils`

| TS source | Status | Rust file(s) | Note |
|---|---|---|---|
| `lib/util/schema-utils/index.ts` | pending | — | — |

### `util/template`

| TS source | Status | Rust file(s) | Note |
|---|---|---|---|
| `lib/util/template/index.ts` | pending | — | — |

### `util/vulnerability`

| TS source | Status | Rust file(s) | Note |
|---|---|---|---|
| `lib/util/vulnerability/ecosystem.ts` | pending | — | — |
| `lib/util/vulnerability/utils.ts` | pending | — | — |

