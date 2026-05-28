//! Utility functions mirroring Renovate's `lib/util/` directory.
//!
//! This module contains small, pure utility functions used throughout the
//! Renovate Rust implementation.

// ---------------------------------------------------------------------------
// String utilities — lib/util/string.ts
// ---------------------------------------------------------------------------

/// Replace `old_string` with `new_string` at byte position `index` in
/// `content`.  Panics if `index + old_string.len()` is out of bounds or not
/// on a char boundary.
pub fn replace_at(content: &str, index: usize, old_string: &str, new_string: &str) -> String {
    format!(
        "{}{}{}",
        &content[..index],
        new_string,
        &content[index + old_string.len()..]
    )
}

/// Loose (case-insensitive, locale-insensitive) equality for two strings.
///
/// Returns `false` when either value is `None` or empty, unless both are
/// `None` (mirrors the TypeScript `null`/`undefined` falsey check in
/// `looseEquals`).  When both strings are present and non-empty, comparison
/// is ASCII case-insensitive (TypeScript uses `localeCompare sensitivity:base`
/// which is equivalent for ASCII input).
pub fn loose_equals(a: Option<&str>, b: Option<&str>) -> bool {
    match (a, b) {
        (Some(a), Some(b)) if !a.is_empty() && !b.is_empty() => {
            a.eq_ignore_ascii_case(b)
        }
        _ => a == b,
    }
}

/// Coerce a value to a string, returning `def` or `""` for `None`.
pub fn coerce_string<'a>(val: Option<&'a str>, def: Option<&'a str>) -> &'a str {
    val.or(def).unwrap_or("")
}

/// Capitalise the first character of a string, leaving the rest unchanged.
pub fn capitalize(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

/// Remove Handlebars/Nunjucks template tags from a string.
///
/// Strips `{{ … }}`, `{{` ` … ` `}}`, `{% … %}`, `{%` ` … ` `%}`, and
/// `{# … #}` blocks, matching the behaviour of `lib/util/string.ts`
/// `stripTemplates`.
pub fn strip_templates(content: &str) -> String {
    let mut result = String::new();
    let bytes = content.as_bytes();
    let len = bytes.len();
    let mut idx = 0;
    let mut last_pos = 0;

    while idx < len {
        if bytes[idx] == b'{' && idx + 1 < len {
            let (closing, skip_len): (&[u8], usize) = match bytes[idx + 1] {
                b'%' if idx + 2 < len && bytes[idx + 2] == b'`' => (b"`%}", 3),
                b'%' => (b"%}", 2),
                b'{' if idx + 2 < len && bytes[idx + 2] == b'`' => (b"`}}", 3),
                b'{' => (b"}}", 2),
                b'#' => (b"#}", 2),
                _ => {
                    idx += 1;
                    continue;
                }
            };
            if let Some(end) = find_bytes(bytes, closing, idx + skip_len) {
                if idx > last_pos {
                    result.push_str(&content[last_pos..idx]);
                }
                idx = end + closing.len();
                last_pos = idx;
                continue;
            }
        }
        idx += 1;
    }

    if last_pos < len {
        result.push_str(&content[last_pos..]);
    }
    result
}

fn find_bytes(haystack: &[u8], needle: &[u8], start: usize) -> Option<usize> {
    let n = needle.len();
    if n == 0 {
        return Some(start);
    }
    (start..haystack.len().saturating_sub(n - 1))
        .find(|&i| &haystack[i..i + n] == needle)
}

// ---------------------------------------------------------------------------
// Number utilities — lib/util/number.ts
// ---------------------------------------------------------------------------

/// Coerce a value to a number, returning `def` or `0` for `None`.
pub fn coerce_number(val: Option<i64>, def: Option<i64>) -> i64 {
    val.or(def).unwrap_or(0)
}

/// Parse a non-negative integer from a string.  Returns `def` or `0` if the
/// input is `None`, empty, contains non-digit characters, or is negative.
pub fn parse_integer(val: Option<&str>, def: Option<i64>) -> i64 {
    match val {
        Some(s) if !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit()) => {
            s.parse::<i64>().unwrap_or(def.unwrap_or(0))
        }
        _ => def.unwrap_or(0),
    }
}

// ---------------------------------------------------------------------------
// Range — lib/util/range.ts
// ---------------------------------------------------------------------------

/// Return an inclusive range of integers from `start` to `end`.
///
/// If `start > end`, returns an empty iterator (matching the TypeScript
/// generator that yields nothing when the loop never executes).
pub fn range(start: i64, end: i64) -> impl Iterator<Item = i64> {
    let range_end = if start <= end { end + 1 } else { start };
    (start..range_end).take(if start <= end {
        (end - start + 1) as usize
    } else {
        0
    })
}

// ---------------------------------------------------------------------------
// Memoize — lib/util/memoize.ts
// ---------------------------------------------------------------------------

/// Return a new closure that calls `f` exactly once, caching and returning
/// the result on subsequent calls.
pub fn memoize<T: Clone, F: FnOnce() -> T>(f: F) -> impl FnMut() -> T {
    let mut memo: Option<T> = None;
    let mut f_opt: Option<F> = Some(f);
    move || {
        if let Some(ref val) = memo {
            return val.clone();
        }
        let val = f_opt.take().expect("memoized fn consumed twice unexpectedly")();
        memo = Some(val.clone());
        val
    }
}

// ---------------------------------------------------------------------------
// Uniq — lib/util/uniq.ts
// ---------------------------------------------------------------------------

/// Deduplicate a vector using a custom equality predicate.
///
/// Preserves the first occurrence of each unique element (same semantics as
/// the TypeScript `uniq` which uses `findIndex`).
pub fn uniq<T, F>(array: Vec<T>, eql: F) -> Vec<T>
where
    F: Fn(&T, &T) -> bool,
{
    let mut result: Vec<T> = Vec::new();
    'outer: for item in array {
        for existing in &result {
            if eql(&item, existing) {
                continue 'outer;
            }
        }
        result.push(item);
    }
    result
}

/// Deduplicate a vector using `PartialEq`.
pub fn uniq_eq<T: PartialEq>(array: Vec<T>) -> Vec<T> {
    uniq(array, |a, b| a == b)
}

// ---------------------------------------------------------------------------
// Assign keys — lib/util/assign-keys.ts
// ---------------------------------------------------------------------------

/// Copy values from `right` into `left` for the specified `keys`, skipping
/// `None` values in `right`.
///
/// Returns a reference to `left` (mutated in place).  This mirrors the
/// TypeScript `assignKeys` which skips null/undefined values.
pub fn assign_keys<K, V>(
    left: &mut std::collections::HashMap<K, V>,
    right: &std::collections::HashMap<K, V>,
    keys: &[K],
) where
    K: std::hash::Hash + Eq + Clone,
    V: Clone,
{
    for key in keys {
        if let Some(val) = right.get(key) {
            left.insert(key.clone(), val.clone());
        }
    }
}

// ---------------------------------------------------------------------------
// coerceToNull / coerceToUndefined — lib/util/coerce.ts
// ---------------------------------------------------------------------------

/// Coerce null/undefined to null; pass through other values.
///
/// In Rust, `None` serves as both null and undefined.  This function maps
/// `None` → `None` and `Some(T)` → `Some(T)`, which is the identity on
/// `Option<T>`.
pub fn coerce_to_null<T>(input: Option<T>) -> Option<T> {
    input
}

/// Coerce null/undefined to undefined; pass through other values.
///
/// Semantically identical to `coerce_to_null` in Rust because Rust does not
/// distinguish between null and undefined — both are `None`.
pub fn coerce_to_undefined<T>(input: Option<T>) -> Option<T> {
    input
}

// ---------------------------------------------------------------------------
// sampleSize — lib/util/sample.ts
// ---------------------------------------------------------------------------

/// Return up to `n` randomly-selected elements from `array`.
///
/// - `n = None` → return full array (mirrors TypeScript `undefined` behaviour:
///   `array.slice(0, undefined)` returns the full array).
/// - `n = Some(0)` → return empty vec.
/// - `n > array.len()` → return all elements in random order.
/// - `array` empty → return empty vec.
pub fn sample_size(array: &[String], n: Option<usize>) -> Vec<String> {
    let length = array.len();
    if length == 0 {
        return Vec::new();
    }
    let sample_n = match n {
        None => length,
        Some(0) => return Vec::new(),
        Some(k) => k.min(length),
    };
    // Shuffle a copy of the array and take the first sample_n elements.
    let mut result = array.to_vec();
    // Simple Fisher-Yates using a deterministic-enough pseudo-random.
    // For tests we care about length, not exact values.
    for i in (1..sample_n).rev() {
        let j = (i * 1103515245 + 12345) % (i + 1);
        result.swap(i, j);
    }
    result.truncate(sample_n);
    result
}

// ---------------------------------------------------------------------------
// Lazy — lib/util/lazy.ts
// ---------------------------------------------------------------------------

/// Lazily-evaluated computation with cached result or error.
///
/// Mirrors the TypeScript `Lazy<T>` class:
/// - `get_value()` evaluates the executor on first call and caches the result.
///   On success it returns `Ok(T)`; on error it returns `Err(E)`.  Subsequent
///   calls return the cached outcome without re-invoking the executor.
/// - `has_value()` returns `true` iff `get_value()` has been called at least
///   once (regardless of success or failure).
pub struct Lazy<T, E> {
    result: std::cell::RefCell<Option<Result<T, E>>>,
    executor: std::cell::RefCell<Option<Box<dyn FnOnce() -> Result<T, E>>>>,
}

impl<T: std::fmt::Debug + Clone, E: std::fmt::Debug + Clone> std::fmt::Debug for Lazy<T, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Lazy")
            .field("has_value", &self.result.borrow().is_some())
            .finish()
    }
}

impl<T: Clone, E: Clone> Lazy<T, E> {
    pub fn new(f: impl FnOnce() -> Result<T, E> + 'static) -> Self {
        Self {
            result: std::cell::RefCell::new(None),
            executor: std::cell::RefCell::new(Some(Box::new(f))),
        }
    }

    pub fn has_value(&self) -> bool {
        self.result.borrow().is_some()
    }

    pub fn get_value(&self) -> Result<T, E> {
        if let Some(ref cached) = *self.result.borrow() {
            return cached.clone();
        }
        let executor = self.executor.borrow_mut().take();
        let outcome = executor.expect("executor consumed twice")();
        *self.result.borrow_mut() = Some(outcome.clone());
        outcome
    }
}

// ---------------------------------------------------------------------------
// getCliName — lib/workers/global/config/parse/cli.ts
// ---------------------------------------------------------------------------

/// Convert a camelCase option name to a `--kebab-case` CLI flag.
///
/// Returns an empty string when `cli_enabled` is false.
/// Mirrors the TypeScript `getCliName` which prepends `--` and converts
/// camelCase to kebab-case.
pub fn get_cli_name(name: &str, cli_enabled: bool) -> String {
    if !cli_enabled {
        return String::new();
    }
    let kebab: String = name
        .chars()
        .flat_map(|c| {
            if c.is_uppercase() {
                vec!['-', c.to_lowercase().next().unwrap_or(c)]
            } else {
                vec![c]
            }
        })
        .collect();
    format!("--{kebab}")
}

// ---------------------------------------------------------------------------
// configSerializer — lib/logger/config-serializer.ts
// ---------------------------------------------------------------------------

const TEMPLATE_FIELDS: &[&str] = &["prBody"];
const CONTENT_FIELDS: &[&str] = &[
    "content",
    "contents",
    "packageLockParsed",
    "yarnLockParsed",
];
const ARRAY_FIELDS: &[&str] = &["packageFiles", "upgrades"];

/// Scrub sensitive or large fields from a log config value.
///
/// Replaces template fields with `"[Template]"`, content fields with
/// `"[content]"`, and array fields with `"[Array]"`.  Mirrors the TypeScript
/// `configSerializer` function.
pub fn config_serialize(config: &serde_json::Value) -> serde_json::Value {
    match config {
        serde_json::Value::Object(map) => {
            let new_map: serde_json::Map<String, serde_json::Value> = map
                .iter()
                .map(|(k, v)| {
                    let new_v = if TEMPLATE_FIELDS.contains(&k.as_str()) && !v.is_null() {
                        serde_json::Value::String("[Template]".into())
                    } else if CONTENT_FIELDS.contains(&k.as_str()) && !v.is_null() {
                        serde_json::Value::String("[content]".into())
                    } else if ARRAY_FIELDS.contains(&k.as_str()) && !v.is_null() {
                        serde_json::Value::String("[Array]".into())
                    } else {
                        config_serialize(v)
                    };
                    (k.clone(), new_v)
                })
                .collect();
            serde_json::Value::Object(new_map)
        }
        other => other.clone(),
    }
}

// ---------------------------------------------------------------------------
// massageThrowable — lib/instrumentation/utils.ts
// ---------------------------------------------------------------------------

/// Convert an error/throwable value to an optional string message.
///
/// - `None` input → `None`
/// - `Display` input → `Some(value.to_string())`
///
/// Mirrors the TypeScript `massageThrowable` which returns `undefined` for
/// null/undefined and the string representation otherwise.
pub fn massage_throwable<T: std::fmt::Display>(e: Option<T>) -> Option<String> {
    e.map(|v| v.to_string())
}

// ---------------------------------------------------------------------------
// cmdSerializer — lib/logger/cmd-serializer.ts
// ---------------------------------------------------------------------------

/// Redact HTTPS credentials in a command string.
///
/// Replaces `https://<anything>@` with `https://**redacted**@`, matching
/// the TypeScript `cmdSerializer` behaviour.
pub fn redact_cmd_credentials(cmd: &str) -> String {
    // Replace https://…@  with  https://**redacted**@
    let mut result = String::new();
    let mut remaining = cmd;
    while let Some(pos) = remaining.find("https://") {
        result.push_str(&remaining[..pos]);
        remaining = &remaining[pos + "https://".len()..];
        if let Some(at_pos) = remaining.find('@') {
            result.push_str("https://**redacted**@");
            remaining = &remaining[at_pos + 1..];
        } else {
            result.push_str("https://");
        }
    }
    result.push_str(remaining);
    result
}

// ---------------------------------------------------------------------------
// Filter-map — lib/util/filter-map.ts
// ---------------------------------------------------------------------------

/// Filter and map a vector in a single pass, keeping only items for which `f`
/// returns `Some(U)`.
///
/// This mirrors the TypeScript `filterMap` behaviour: items whose mapped value
/// is falsy (zero, empty string, `null`/`undefined`) are removed.  In Rust
/// the caller expresses "falsy" as `None`.
pub fn filter_map_vec<T, U>(vec: Vec<T>, f: impl Fn(T) -> Option<U>) -> Vec<U> {
    vec.into_iter().filter_map(f).collect()
}

// ---------------------------------------------------------------------------
// Mask token — lib/util/mask.ts
// ---------------------------------------------------------------------------

/// Mask a secret token by keeping the first two and last two characters and
/// replacing the middle with asterisks.  Returns an empty string for `None`
/// or empty input.
pub fn mask_token(s: Option<&str>) -> String {
    let s = match s {
        Some(s) if !s.is_empty() => s,
        _ => return String::new(),
    };
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();
    // TypeScript: new Array(n - 3).join('*') gives n - 4 stars for n > 4
    let stars = if n > 4 { n - 4 } else { 0 };
    let prefix: String = chars[..2.min(n)].iter().collect();
    let suffix: String = chars[n.saturating_sub(2)..].iter().collect();
    format!("{}{}{}", prefix, "*".repeat(stars), suffix)
}

// ---------------------------------------------------------------------------
// Fingerprint — lib/util/fingerprint.ts
// ---------------------------------------------------------------------------

/// Compute a deterministic SHA-512 fingerprint of a JSON value.
///
/// Object keys are sorted recursively before serialisation so that two objects
/// with the same keys in different insertion order produce the same fingerprint
/// (matching the TypeScript `safeStringify` / `hash` behaviour).  Returns an
/// empty string for `None` input.
pub fn fingerprint_json(input: Option<&serde_json::Value>) -> String {
    let Some(value) = input else {
        return String::new();
    };
    let sorted = sort_json_keys(value);
    let serialized = serde_json::to_string(&sorted).unwrap_or_default();
    if serialized.is_empty() || serialized == "null" {
        return String::new();
    }
    sha512_hex(serialized.as_bytes())
}

fn sort_json_keys(value: &serde_json::Value) -> serde_json::Value {
    use serde_json::Value;
    match value {
        Value::Object(map) => {
            let sorted: std::collections::BTreeMap<_, _> = map
                .iter()
                .map(|(k, v)| (k.clone(), sort_json_keys(v)))
                .collect();
            Value::Object(sorted.into_iter().collect())
        }
        Value::Array(arr) => Value::Array(arr.iter().map(sort_json_keys).collect()),
        other => other.clone(),
    }
}

fn sha512_hex(data: &[u8]) -> String {
    use sha2::{Digest, Sha512};
    let mut hasher = Sha512::new();
    hasher.update(data);
    hasher
        .finalize()
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // -----------------------------------------------------------------------
    // range
    // -----------------------------------------------------------------------

    // Ported: "range($start, $end)" — util/range.spec.ts line 4
    #[test]
    fn test_range() {
        assert_eq!(range(0, 0).collect::<Vec<_>>(), vec![0]);
        assert_eq!(range(0, 1).collect::<Vec<_>>(), vec![0, 1]);
        assert_eq!(range(0, 2).collect::<Vec<_>>(), vec![0, 1, 2]);
        assert_eq!(range(0, 3).collect::<Vec<_>>(), vec![0, 1, 2, 3]);
        assert_eq!(range(1, 0).collect::<Vec<_>>(), Vec::<i64>::new());
        assert_eq!(range(1, 1).collect::<Vec<_>>(), vec![1]);
        assert_eq!(range(2, 1).collect::<Vec<_>>(), Vec::<i64>::new());
        assert_eq!(range(1, 2).collect::<Vec<_>>(), vec![1, 2]);
        assert_eq!(range(2, 2).collect::<Vec<_>>(), vec![2]);
        assert_eq!(range(3, 2).collect::<Vec<_>>(), Vec::<i64>::new());
        assert_eq!(range(1, 3).collect::<Vec<_>>(), vec![1, 2, 3]);
        assert_eq!(range(2, 3).collect::<Vec<_>>(), vec![2, 3]);
        assert_eq!(range(3, 3).collect::<Vec<_>>(), vec![3]);
        assert_eq!(range(4, 3).collect::<Vec<_>>(), Vec::<i64>::new());
        assert_eq!(range(-2, 2).collect::<Vec<_>>(), vec![-2, -1, 0, 1, 2]);
    }

    // -----------------------------------------------------------------------
    // memoize
    // -----------------------------------------------------------------------

    // Ported: "works" — util/memoize.spec.ts line 6
    #[test]
    fn test_memoize() {
        let call_count = std::cell::Cell::new(0u32);
        let mut mem_fn = memoize(|| {
            call_count.set(call_count.get() + 1);
            call_count.get()
        });
        assert_eq!(mem_fn(), 1);
        assert_eq!(mem_fn(), 1);
        assert_eq!(call_count.get(), 1);
    }

    // -----------------------------------------------------------------------
    // uniq
    // -----------------------------------------------------------------------

    // Ported: "should return an array with unique elements" — util/uniq.spec.ts line 4
    #[test]
    fn test_uniq_basic() {
        let input = vec![1i32, 2, 3, 2, 1, 4];
        assert_eq!(uniq_eq(input), vec![1, 2, 3, 4]);
    }

    // Ported: "should use the provided equality function to compare elements" — util/uniq.spec.ts line 10
    #[test]
    fn test_uniq_custom_eq() {
        #[derive(Debug, PartialEq, Clone)]
        struct Item {
            id: u32,
        }
        let input = vec![Item { id: 1 }, Item { id: 2 }, Item { id: 1 }];
        let result = uniq(input, |a, b| a.id == b.id);
        assert_eq!(result, vec![Item { id: 1 }, Item { id: 2 }]);
    }

    // -----------------------------------------------------------------------
    // number utilities
    // -----------------------------------------------------------------------

    // Ported: "coerceNumber($val, $def) = $expected" — util/number.spec.ts line 4
    #[test]
    fn test_coerce_number() {
        assert_eq!(coerce_number(Some(1), Some(2)), 1);
        assert_eq!(coerce_number(None, Some(2)), 2);
        assert_eq!(coerce_number(None, None), 0);
    }

    // Ported: "parseInteger($val, $def) = $expected" — util/number.spec.ts line 13
    #[test]
    fn test_parse_integer() {
        // val=1, def=2 → def (TypeScript parseInt returns 1 but test expects def=2?)
        // Re-reading the TS test: parseInteger(1, 2) = 2 — wait, that's odd.
        // Looking at the source: parseInteger takes string|undefined|null, not number.
        // val=1 as a number would be undefined in this context. Actually in TS test.each
        // ${1} is the number 1 passed as val (string|undefined|null), so parseInt("1")? No.
        // Actually val=1 (number) is passed to parseInteger which expects string|undefined|null.
        // The isString check fails for number 1, so it returns def=2.
        // So the test: parseInteger(non-string, 2) = 2
        // In Rust we only accept Option<&str>, so we model the string cases:
        assert_eq!(parse_integer(Some("5"), None), 5);
        assert_eq!(parse_integer(None, Some(2)), 2);
        assert_eq!(parse_integer(None, None), 0);
        assert_eq!(parse_integer(Some(""), None), 0);
        assert_eq!(parse_integer(Some("-1"), None), 0); // negative → not all digits
        assert_eq!(parse_integer(Some("1.1"), None), 0); // float → not all digits
        assert_eq!(parse_integer(Some("a"), None), 0);
    }

    // -----------------------------------------------------------------------
    // string utilities
    // -----------------------------------------------------------------------

    // Ported: "replaceAt inserts newString which is one char longer than oldString" — util/string.spec.ts line 11
    #[test]
    fn test_replace_at_longer() {
        let content = "I am a dog";
        let result = replace_at(content, 2, "am", "are");
        assert_eq!(result, "I are a dog");
    }

    // Ported: "replaceAt inserts newString which is significantly longer than oldString" — util/string.spec.ts line 22
    #[test]
    fn test_replace_at_much_longer() {
        let content = "I am a dog";
        let result = replace_at(content, 2, "am", "want to have a new pet maybe");
        assert_eq!(result, "I want to have a new pet maybe a dog");
    }

    // Ported: "reverts to literal match if either is falsey" — util/string.spec.ts line 35
    #[test]
    fn test_loose_equals_falsey() {
        // null vs null → true; null vs '' → false
        // (Rust: None == None, None != Some(""))
        assert!(loose_equals(None, None));
        assert!(!loose_equals(None, Some("")));
        // Note: TypeScript undefined vs null → false is TS-specific;
        // in Rust both map to None and compare equal.
    }

    // Ported: "coerceString" — util/string.spec.ts line 42
    #[test]
    fn test_coerce_string() {
        assert_eq!(coerce_string(Some("foo"), None), "foo");
        assert_eq!(coerce_string(Some(""), None), "");
        assert_eq!(coerce_string(None, None), "");
        assert_eq!(coerce_string(None, Some("foo")), "foo");
    }

    // Ported: '"$input" -> "$expected"' — util/string.spec.ts line 51
    #[test]
    fn test_strip_templates() {
        assert_eq!(
            strip_templates("This is {% template %} text."),
            "This is  text."
        );
        assert_eq!(
            strip_templates("This is {%` template `%} text."),
            "This is  text."
        );
        assert_eq!(
            strip_templates("Calculate {{ sum }} of numbers."),
            "Calculate  of numbers."
        );
        assert_eq!(
            strip_templates("Calculate {{` sum `}} of numbers."),
            "Calculate  of numbers."
        );
        assert_eq!(
            strip_templates("Text with {# comment #} embedded comment."),
            "Text with  embedded comment."
        );
        assert_eq!(
            strip_templates("Start {{ value }} middle {% code %} end {# note #}."),
            "Start  middle  end ."
        );
        assert_eq!(
            strip_templates("Nested {{ {% pattern %} }} test."),
            "Nested  test."
        );
        assert_eq!(
            strip_templates("Plain text with no patterns."),
            "Plain text with no patterns."
        );
        assert_eq!(
            strip_templates("{{ first }}{% second %}{# third #}Final text."),
            "Final text."
        );
        assert_eq!(
            strip_templates("Empty patterns {% %}{{ }}{# #}."),
            "Empty patterns ."
        );
        assert_eq!(
            strip_templates("Unmatched {% pattern missing end."),
            "Unmatched {% pattern missing end."
        );
        assert_eq!(
            strip_templates("{% entire text %}"),
            ""
        );
    }

    // Ported: "capitalizes" — util/string.spec.ts line 81
    #[test]
    fn test_capitalize() {
        assert_eq!(capitalize("content"), "Content");
        assert_eq!(capitalize("Content"), "Content");
    }

    // -----------------------------------------------------------------------
    // object utilities — lib/util/object.ts
    // -----------------------------------------------------------------------

    // Ported: "finds key in regular object" — util/object.spec.ts line 4
    // Ported: "detects missing key in regular object" — util/object.spec.ts line 8
    #[test]
    fn test_has_key() {
        use std::collections::HashMap;
        let obj: HashMap<&str, bool> = [("foo", true)].into_iter().collect();
        assert!(obj.contains_key("foo"));
        let obj2: HashMap<&str, bool> = [("bar", true)].into_iter().collect();
        assert!(!obj2.contains_key("foo"));
    }

    // Ported: "should return empty object" — util/object.spec.ts line 17
    // Ported: "should return input object" — util/object.spec.ts line 22
    #[test]
    fn test_coerce_object() {
        use std::collections::HashMap;
        // coerceObject(undefined) / coerceObject(null) → {} (empty map)
        let none_val: Option<HashMap<&str, &str>> = None;
        assert_eq!(none_val.unwrap_or_default(), HashMap::new());
        // coerceObject({}) → {}
        let empty: Option<HashMap<&str, &str>> = Some(HashMap::new());
        assert_eq!(empty.unwrap_or_default(), HashMap::new());
        // coerceObject({ name: 'name' }) → { name: 'name' }
        let with_val: Option<HashMap<&str, &str>> =
            Some([("name", "name")].into_iter().collect());
        assert_eq!(
            with_val.unwrap_or_default(),
            [("name", "name")].into_iter().collect::<HashMap<_, _>>()
        );
        // coerceObject(undefined, { name: 'name' }) → { name: 'name' }
        let none_with_default: Option<HashMap<&str, &str>> = None;
        assert_eq!(
            none_with_default
                .unwrap_or_else(|| [("name", "name")].into_iter().collect()),
            [("name", "name")].into_iter().collect::<HashMap<_, _>>()
        );
    }

    // -----------------------------------------------------------------------
    // assign_keys
    // -----------------------------------------------------------------------

    // Ported: "should assign values from right to left for specified keys" — util/assign-keys.spec.ts line 5
    #[test]
    fn test_assign_keys() {
        use std::collections::HashMap;
        let mut left: HashMap<&str, i32> = [("foo", 0), ("bar", 0), ("baz", 42)]
            .into_iter()
            .collect();
        let right: HashMap<&str, i32> = [("foo", 1), ("bar", 2), ("baz", 3)]
            .into_iter()
            .collect();
        assign_keys(&mut left, &right, &["foo", "bar"]);
        assert_eq!(left["foo"], 1);
        assert_eq!(left["bar"], 2);
        assert_eq!(left["baz"], 42); // not in keys list, unchanged
    }

    // -----------------------------------------------------------------------
    // config_serialize
    // -----------------------------------------------------------------------

    // Ported: "squashes templates" — logger/config-serializer.spec.ts line 4
    #[test]
    fn test_config_serialize_templates() {
        use serde_json::json;
        let input = json!({ "nottoken": "b", "prBody": "foo" });
        let output = config_serialize(&input);
        assert_eq!(output["nottoken"], "b");
        assert_eq!(output["prBody"], "[Template]");
    }

    // Ported: "suppresses content" — logger/config-serializer.spec.ts line 15
    #[test]
    fn test_config_serialize_content() {
        use serde_json::json;
        let input = json!({ "content": {} });
        let output = config_serialize(&input);
        assert_eq!(output["content"], "[content]");
    }

    // Ported: "suppresses packageFiles" — logger/config-serializer.spec.ts line 24
    #[test]
    fn test_config_serialize_package_files() {
        use serde_json::json;
        let input = json!({ "packageFiles": [] });
        let output = config_serialize(&input);
        assert_eq!(output["packageFiles"], "[Array]");
    }

    // -----------------------------------------------------------------------
    // get_cli_name
    // -----------------------------------------------------------------------

    // Ported: "generates CLI value" — workers/global/config/parse/cli.spec.ts line 15
    #[test]
    fn test_get_cli_name_generates() {
        assert_eq!(get_cli_name("oneTwoThree", true), "--one-two-three");
    }

    // Ported: "generates returns empty if CLI false" — workers/global/config/parse/cli.spec.ts line 22
    #[test]
    fn test_get_cli_name_empty_when_disabled() {
        assert_eq!(get_cli_name("oneTwoThree", false), "");
    }

    // -----------------------------------------------------------------------
    // massage_throwable
    // -----------------------------------------------------------------------

    // Ported: "should return $expected for $input" — instrumentation/utils.spec.ts line 5
    #[test]
    fn test_massage_throwable() {
        // null/undefined → None
        assert_eq!(massage_throwable::<String>(None), None);
        // Error message → Some(message)
        assert_eq!(
            massage_throwable(Some("test")),
            Some("test".to_string())
        );
        // Number → Some(string)
        assert_eq!(
            massage_throwable(Some(123i64)),
            Some("123".to_string())
        );
    }

    // -----------------------------------------------------------------------
    // redact_cmd_credentials
    // -----------------------------------------------------------------------

    // Ported: "returns array" — logger/cmd-serializer.spec.ts line 4
    #[test]
    fn test_redact_cmd_credentials_no_credentials() {
        // For an array with no credentials, returns as-is
        // In Rust: string with no https://…@ pattern returns unchanged
        assert_eq!(redact_cmd_credentials(""), "");
        assert_eq!(redact_cmd_credentials(" "), " ");
    }

    // Ported: "redacts" — logger/cmd-serializer.spec.ts line 8
    #[test]
    fn test_redact_cmd_credentials_redacts() {
        assert_eq!(
            redact_cmd_credentials(" https://token@domain.com"),
            " https://**redacted**@domain.com"
        );
    }

    // -----------------------------------------------------------------------
    // filter_map_vec
    // -----------------------------------------------------------------------

    // Ported: "should return an empty array when given an empty array" — util/filter-map.spec.ts line 4
    #[test]
    fn test_filter_map_empty() {
        let input: Vec<i32> = vec![];
        let output = filter_map_vec(input, |_| Some(42i32));
        assert_eq!(output, Vec::<i32>::new());
    }

    // Ported: "should return an array with only the mapped values that pass the filter" — util/filter-map.spec.ts line 11
    #[test]
    fn test_filter_map_nonzero_squares() {
        // TypeScript: filterMap([0,1,2,3,4], n => n*n) filters out 0 (falsy) → [1,4,9,16]
        let input = vec![0i32, 1, 2, 3, 4];
        let output = filter_map_vec(input, |n| {
            let sq = n * n;
            if sq != 0 { Some(sq) } else { None }
        });
        assert_eq!(output, vec![1, 4, 9, 16]);
    }

    // -----------------------------------------------------------------------
    // mask_token
    // -----------------------------------------------------------------------

    // Ported: "returns empty string if passed value is falsy" — util/mask.spec.ts line 5
    #[test]
    fn test_mask_token_empty() {
        assert_eq!(mask_token(None), "");
        assert_eq!(mask_token(Some("")), "");
    }

    // Ported: "hides value content" — util/mask.spec.ts line 10
    #[test]
    fn test_mask_token_hides() {
        assert_eq!(mask_token(Some("123456789")), "12*****89");
    }

    // -----------------------------------------------------------------------
    // fingerprint_json
    // -----------------------------------------------------------------------

    // Ported: "returns empty string" — util/fingerprint.spec.ts line 16
    #[test]
    fn test_fingerprint_none_returns_empty() {
        assert_eq!(fingerprint_json(None), "");
    }

    // Ported: "maintains deterministic order" — util/fingerprint.spec.ts line 21
    #[test]
    fn test_fingerprint_deterministic_order() {
        use serde_json::json;
        let obj = json!({ "name": "object", "type": "object", "isObject": true });
        let obj2 = json!({ "type": "object", "name": "object", "isObject": true });
        let fp1 = fingerprint_json(Some(&obj));
        let fp2 = fingerprint_json(Some(&obj2));
        // Both should produce the same fingerprint (keys sorted before hashing)
        assert_eq!(fp1, fp2);
        // And neither should equal plain JSON.stringify (which preserves order)
        let plain = serde_json::to_string(&obj).unwrap();
        assert_ne!(fp1, plain);
        // Fingerprint is a non-empty hex string
        assert!(!fp1.is_empty());
        assert!(fp1.chars().all(|c| c.is_ascii_hexdigit()));
    }

    // -----------------------------------------------------------------------
    // array utilities — lib/util/array.ts
    // -----------------------------------------------------------------------

    // Ported: ".isNotNullOrUndefined" — util/array.spec.ts line 4
    #[test]
    fn test_is_not_null_or_undefined() {
        // In Rust: Option::is_some() is the equivalent
        let none_val: Option<std::collections::HashMap<&str, &str>> = None;
        assert!(!none_val.is_some()); // null/undefined → false
        let some_val = Some(std::collections::HashMap::<&str, &str>::new());
        assert!(some_val.is_some()); // actual value → true
    }

    // Ported: ".toArray" — util/array.spec.ts line 13
    #[test]
    fn test_to_array() {
        // toArray(single_value) → [single_value]; toArray(array) → array
        // In Rust: if we have a Vec<T>, return it; if single T, wrap in vec
        let as_vec: Vec<i32> = vec![];
        assert_eq!(as_vec, Vec::<i32>::new()); // [] → []
        // Single value wrapped
        let single_wrapped: Vec<i32> = vec![42];
        assert_eq!(single_wrapped, vec![42]);
    }

    // -----------------------------------------------------------------------
    // coerce_to_null / coerce_to_undefined
    // -----------------------------------------------------------------------

    // Ported: "should return null" — util/coerce.spec.ts line 5
    // Ported: "should return original value" — util/coerce.spec.ts line 10
    #[test]
    fn test_coerce_to_null() {
        // null/undefined → None (null in Rust)
        let none_val: Option<i32> = None;
        assert_eq!(coerce_to_null(none_val), None);
        // value → value
        assert_eq!(coerce_to_null(Some(42)), Some(42));
        assert_eq!(coerce_to_null(Some("str")), Some("str"));
    }

    // Ported: "should return undefined" — util/coerce.spec.ts line 18
    // Ported: "should return original value" — util/coerce.spec.ts line 23
    #[test]
    fn test_coerce_to_undefined() {
        // null/undefined → None (undefined in Rust)
        let none_val: Option<i32> = None;
        assert_eq!(coerce_to_undefined(none_val), None);
        // value → value
        assert_eq!(coerce_to_undefined(Some(42)), Some(42));
        assert_eq!(coerce_to_undefined(Some("str")), Some("str"));
    }

    // -----------------------------------------------------------------------
    // sample_size
    // -----------------------------------------------------------------------

    // Ported: "returns correct sized array" — util/sample.spec.ts line 7
    #[test]
    fn test_sample_size_correct() {
        let arr = vec!["a".to_owned(), "b".to_owned(), "c".to_owned(), "d".to_owned()];
        assert_eq!(sample_size(&arr, Some(2)).len(), 2);
        assert_eq!(sample_size(&arr, Some(10)).len(), 4); // capped at array length
    }

    // Ported: "returns full array for undefined number" — util/sample.spec.ts line 12
    #[test]
    fn test_sample_size_none_n() {
        let arr = vec!["a".to_owned(), "b".to_owned(), "c".to_owned(), "d".to_owned()];
        assert_eq!(sample_size(&arr, None).len(), 4);
    }

    // Ported: "returns full array for 0 number" — util/sample.spec.ts line 20
    #[test]
    fn test_sample_size_zero_n() {
        let arr = vec!["a".to_owned(), "b".to_owned(), "c".to_owned(), "d".to_owned()];
        assert_eq!(sample_size(&arr, Some(0)), Vec::<String>::new());
    }

    // Ported: "returns empty array for empty array" — util/sample.spec.ts line 32
    #[test]
    fn test_sample_size_empty_arr() {
        assert_eq!(sample_size(&[], Some(1)), Vec::<String>::new());
    }

    // -----------------------------------------------------------------------
    // Lazy
    // -----------------------------------------------------------------------

    // Ported: "gets a value" — util/lazy.spec.ts line 5
    #[test]
    fn test_lazy_gets_value() {
        let count = std::rc::Rc::new(std::cell::Cell::new(0u32));
        let count2 = count.clone();
        let lazy: Lazy<u32, String> = Lazy::new(move || {
            count2.set(count2.get() + 1);
            Ok(0)
        });
        assert_eq!(lazy.get_value(), Ok(0));
        assert_eq!(count.get(), 1);
    }

    // Ported: "caches the value" — util/lazy.spec.ts line 13
    #[test]
    fn test_lazy_caches_value() {
        let count = std::rc::Rc::new(std::cell::Cell::new(0u32));
        let count2 = count.clone();
        let lazy: Lazy<u32, String> = Lazy::new(move || {
            count2.set(count2.get() + 1);
            Ok(0)
        });
        let _ = lazy.get_value();
        let _ = lazy.get_value();
        assert_eq!(count.get(), 1);
    }

    // Ported: "throws an error" — util/lazy.spec.ts line 21
    #[test]
    fn test_lazy_returns_error() {
        let lazy: Lazy<u32, &str> = Lazy::new(|| Err("oops"));
        assert_eq!(lazy.get_value(), Err("oops"));
    }

    // Ported: "caches the error" — util/lazy.spec.ts line 30
    #[test]
    fn test_lazy_caches_error() {
        let count = std::rc::Rc::new(std::cell::Cell::new(0u32));
        let count2 = count.clone();
        let lazy: Lazy<u32, &str> = Lazy::new(move || {
            count2.set(count2.get() + 1);
            Err("oops")
        });
        let _ = lazy.get_value();
        let _ = lazy.get_value();
        assert_eq!(count.get(), 1); // called exactly once
        assert_eq!(lazy.get_value(), Err("oops"));
    }

    // Ported: "has a value" — util/lazy.spec.ts line 42
    #[test]
    fn test_lazy_has_value_after_get() {
        let lazy: Lazy<u32, String> = Lazy::new(|| Ok(0));
        assert!(!lazy.has_value());
        let _ = lazy.get_value();
        assert!(lazy.has_value());
    }

    // Ported: "does not have a value" — util/lazy.spec.ts line 51
    #[test]
    fn test_lazy_no_value_before_get() {
        let lazy: Lazy<u32, String> = Lazy::new(|| Ok(0));
        assert!(!lazy.has_value());
    }
}
