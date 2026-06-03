# Module: `util/_root`

[← all modules](../../README.md)

**Coverage:** 227/411 tests ported across 45 spec files.

| Spec file | it() | ported | pending | Rust test file(s) | Status |
|---|--:|--:|--:|---|---|
| [`lib/util/array.spec.ts`](../../lib/util/array.spec.ts.md) | 2 | 2 | 0 | [`crates/renovate-core/src/util.rs:6506`](../../../../../crates/renovate-core/src/util.rs#L6506) | ported |
| [`lib/util/assign-keys.spec.ts`](../../lib/util/assign-keys.spec.ts.md) | 1 | 1 | 0 | [`crates/renovate-core/src/util.rs:5811`](../../../../../crates/renovate-core/src/util.rs#L5811) | ported |
| [`lib/util/check-token.spec.ts`](../../lib/util/check-token.spec.ts.md) | 34 | 30 | 4 | [`crates/renovate-core/src/util.rs:6532`](../../../../../crates/renovate-core/src/util.rs#L6532) | partial |
| [`lib/util/clone.spec.ts`](../../lib/util/clone.spec.ts.md) | 3 | 2 | 1 | [`crates/renovate-core/src/util.rs:9705`](../../../../../crates/renovate-core/src/util.rs#L9705) | partial |
| [`lib/util/coerce.spec.ts`](../../lib/util/coerce.spec.ts.md) | 4 | 3 | 1 | [`crates/renovate-core/src/util.rs:7907`](../../../../../crates/renovate-core/src/util.rs#L7907) | partial |
| [`lib/util/common.spec.ts`](../../lib/util/common.spec.ts.md) | 22 | 18 | 4 | [`crates/renovate-core/src/util.rs:8913`](../../../../../crates/renovate-core/src/util.rs#L8913) | partial |
| [`lib/util/compress.spec.ts`](../../lib/util/compress.spec.ts.md) | 1 | 1 | 0 | [`crates/renovate-core/src/util.rs:12203`](../../../../../crates/renovate-core/src/util.rs#L12203) | ported |
| [`lib/util/date.spec.ts`](../../lib/util/date.spec.ts.md) | 8 | 8 | 0 | [`crates/renovate-core/src/util.rs:9575`](../../../../../crates/renovate-core/src/util.rs#L9575) | ported |
| [`lib/util/emoji.spec.ts`](../../lib/util/emoji.spec.ts.md) | 11 | 0 | 11 | — | pending |
| [`lib/util/env.spec.ts`](../../lib/util/env.spec.ts.md) | 2 | 2 | 0 | [`crates/renovate-core/src/util.rs:12120`](../../../../../crates/renovate-core/src/util.rs#L12120) | ported |
| [`lib/util/filter-map.spec.ts`](../../lib/util/filter-map.spec.ts.md) | 2 | 2 | 0 | [`crates/renovate-core/src/util.rs:6437`](../../../../../crates/renovate-core/src/util.rs#L6437) | ported |
| [`lib/util/fingerprint.spec.ts`](../../lib/util/fingerprint.spec.ts.md) | 10 | 2 | 8 | [`crates/renovate-core/src/util.rs:6478`](../../../../../crates/renovate-core/src/util.rs#L6478) | partial |
| [`lib/util/hash.spec.ts`](../../lib/util/hash.spec.ts.md) | 4 | 4 | 0 | [`crates/renovate-core/src/util.rs:9480`](../../../../../crates/renovate-core/src/util.rs#L9480) | ported |
| [`lib/util/host-rules.spec.ts`](../../lib/util/host-rules.spec.ts.md) | 29 | 27 | 2 | [`crates/renovate-core/src/util/host_rules.rs:426`](../../../../../crates/renovate-core/src/util/host_rules.rs#L426) | partial |
| [`lib/util/html.spec.ts`](../../lib/util/html.spec.ts.md) | 4 | 0 | 4 | — | pending |
| [`lib/util/ignore.spec.ts`](../../lib/util/ignore.spec.ts.md) | 5 | 4 | 1 | [`crates/renovate-core/src/string_match.rs:579`](../../../../../crates/renovate-core/src/string_match.rs#L579) | partial |
| [`lib/util/interpolator.spec.ts`](../../lib/util/interpolator.spec.ts.md) | 10 | 10 | 0 | [`crates/renovate-core/src/config/secrets.rs:383`](../../../../../crates/renovate-core/src/config/secrets.rs#L383)<br>[`crates/renovate-core/src/util.rs:9105`](../../../../../crates/renovate-core/src/util.rs#L9105) | ported |
| [`lib/util/jsonata.spec.ts`](../../lib/util/jsonata.spec.ts.md) | 6 | 0 | 6 | — | pending |
| [`lib/util/lazy.spec.ts`](../../lib/util/lazy.spec.ts.md) | 6 | 6 | 0 | [`crates/renovate-core/src/util.rs:8089`](../../../../../crates/renovate-core/src/util.rs#L8089) | ported |
| [`lib/util/markdown.spec.ts`](../../lib/util/markdown.spec.ts.md) | 3 | 3 | 0 | [`crates/renovate-core/src/util.rs:9375`](../../../../../crates/renovate-core/src/util.rs#L9375) | ported |
| [`lib/util/mask.spec.ts`](../../lib/util/mask.spec.ts.md) | 2 | 2 | 0 | [`crates/renovate-core/src/util.rs:6461`](../../../../../crates/renovate-core/src/util.rs#L6461) | ported |
| [`lib/util/memoize.spec.ts`](../../lib/util/memoize.spec.ts.md) | 1 | 1 | 0 | [`crates/renovate-core/src/util.rs:5600`](../../../../../crates/renovate-core/src/util.rs#L5600) | ported |
| [`lib/util/minimatch.spec.ts`](../../lib/util/minimatch.spec.ts.md) | 5 | 2 | 3 | [`crates/renovate-core/src/string_match.rs:616`](../../../../../crates/renovate-core/src/string_match.rs#L616) | partial |
| [`lib/util/mutex.spec.ts`](../../lib/util/mutex.spec.ts.md) | 3 | 0 | 3 | — | pending |
| [`lib/util/number.spec.ts`](../../lib/util/number.spec.ts.md) | 2 | 2 | 0 | [`crates/renovate-core/src/util.rs:5640`](../../../../../crates/renovate-core/src/util.rs#L5640) | ported |
| [`lib/util/object.spec.ts`](../../lib/util/object.spec.ts.md) | 5 | 4 | 1 | [`crates/renovate-core/src/util.rs:5770`](../../../../../crates/renovate-core/src/util.rs#L5770) | partial |
| [`lib/util/pretty-time.spec.ts`](../../lib/util/pretty-time.spec.ts.md) | 3 | 3 | 0 | [`crates/renovate-core/src/util.rs:9653`](../../../../../crates/renovate-core/src/util.rs#L9653) | ported |
| [`lib/util/promises.spec.ts`](../../lib/util/promises.spec.ts.md) | 6 | 0 | 6 | — | pending |
| [`lib/util/range.spec.ts`](../../lib/util/range.spec.ts.md) | 1 | 1 | 0 | [`crates/renovate-core/src/util.rs:5576`](../../../../../crates/renovate-core/src/util.rs#L5576) | ported |
| [`lib/util/regex.spec.ts`](../../lib/util/regex.spec.ts.md) | 6 | 1 | 5 | [`crates/renovate-core/src/util.rs:9354`](../../../../../crates/renovate-core/src/util.rs#L9354) | partial |
| [`lib/util/result.spec.ts`](../../lib/util/result.spec.ts.md) | 85 | 0 | 85 | — | pending |
| [`lib/util/s3.spec.ts`](../../lib/util/s3.spec.ts.md) | 6 | 3 | 3 | [`crates/renovate-core/src/util.rs:10163`](../../../../../crates/renovate-core/src/util.rs#L10163) | partial |
| [`lib/util/sample.spec.ts`](../../lib/util/sample.spec.ts.md) | 7 | 4 | 3 | [`crates/renovate-core/src/util.rs:7935`](../../../../../crates/renovate-core/src/util.rs#L7935) | partial |
| [`lib/util/sanitize.spec.ts`](../../lib/util/sanitize.spec.ts.md) | 3 | 3 | 0 | [`crates/renovate-core/src/util.rs:9427`](../../../../../crates/renovate-core/src/util.rs#L9427) | ported |
| [`lib/util/split.spec.ts`](../../lib/util/split.spec.ts.md) | 1 | 1 | 0 | [`crates/renovate-core/src/util.rs:10142`](../../../../../crates/renovate-core/src/util.rs#L10142) | ported |
| [`lib/util/stats.spec.ts`](../../lib/util/stats.spec.ts.md) | 33 | 12 | 21 | [`crates/renovate-core/src/util.rs:6028`](../../../../../crates/renovate-core/src/util.rs#L6028) | partial |
| [`lib/util/streams.spec.ts`](../../lib/util/streams.spec.ts.md) | 1 | 0 | 1 | — | pending |
| [`lib/util/string-match.spec.ts`](../../lib/util/string-match.spec.ts.md) | 25 | 24 | 1 | [`crates/renovate-core/src/string_match.rs:248`](../../../../../crates/renovate-core/src/string_match.rs#L248) | partial |
| [`lib/util/string.spec.ts`](../../lib/util/string.spec.ts.md) | 6 | 6 | 0 | [`crates/renovate-core/src/util.rs:5673`](../../../../../crates/renovate-core/src/util.rs#L5673) | ported |
| [`lib/util/timestamp.spec.ts`](../../lib/util/timestamp.spec.ts.md) | 1 | 1 | 0 | [`crates/renovate-core/src/timestamp.rs:150`](../../../../../crates/renovate-core/src/timestamp.rs#L150) | ported |
| [`lib/util/toml.spec.ts`](../../lib/util/toml.spec.ts.md) | 3 | 3 | 0 | [`crates/renovate-core/src/util.rs:9521`](../../../../../crates/renovate-core/src/util.rs#L9521) | ported |
| [`lib/util/unicode.spec.ts`](../../lib/util/unicode.spec.ts.md) | 5 | 4 | 1 | [`crates/renovate-core/src/util.rs:9925`](../../../../../crates/renovate-core/src/util.rs#L9925) | partial |
| [`lib/util/uniq.spec.ts`](../../lib/util/uniq.spec.ts.md) | 2 | 2 | 0 | [`crates/renovate-core/src/util.rs:5617`](../../../../../crates/renovate-core/src/util.rs#L5617) | ported |
| [`lib/util/url.spec.ts`](../../lib/util/url.spec.ts.md) | 13 | 13 | 0 | [`crates/renovate-core/src/util.rs:9147`](../../../../../crates/renovate-core/src/util.rs#L9147) | ported |
| [`lib/util/yaml.spec.ts`](../../lib/util/yaml.spec.ts.md) | 19 | 10 | 9 | [`crates/renovate-core/src/util.rs:8771`](../../../../../crates/renovate-core/src/util.rs#L8771) | partial |

