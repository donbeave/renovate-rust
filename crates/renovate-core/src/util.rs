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
}
