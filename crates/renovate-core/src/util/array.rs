//! Array utility functions — mirrors `lib/util/array.ts`.

//! @parity lib/util/array.ts full

use std::collections::HashSet;
use std::hash::Hash;

/// Split array at the first element matching `predicate`.
///
/// Returns `(before, from_match_inclusive)`. If no element matches,
/// returns `(full_array, empty)`.
///
/// Mirrors `splitAt` from `lib/util/array.ts`.
pub fn split_at<T: Clone>(arr: &[T], predicate: impl Fn(&T) -> bool) -> (Vec<T>, Vec<T>) {
    match arr.iter().position(predicate) {
        Some(pos) => (arr[..pos].to_vec(), arr[pos..].to_vec()),
        None => (arr.to_vec(), Vec::new()),
    }
}

/// Coerce a nullable value to an array.
///
/// Mirrors `coerceArray` from `lib/util/array.ts`.
pub fn coerce_array<T>(input: Option<Vec<T>>) -> Vec<T> {
    input.unwrap_or_default()
}

/// Return false for `None`, true otherwise.
///
/// Mirrors `isNotNullOrUndefined` from `lib/util/array.ts`.
pub fn is_not_null_or_undefined<T>(value: Option<T>) -> bool {
    value.is_some()
}

/// Input value that can be converted to a scalar array.
#[derive(Debug)]
pub enum ToArrayInput<T> {
    Value(T),
    Values(Vec<T>),
}

impl<T> From<T> for ToArrayInput<T> {
    fn from(value: T) -> Self {
        Self::Value(value)
    }
}

impl<T> From<Vec<T>> for ToArrayInput<T> {
    fn from(value: Vec<T>) -> Self {
        Self::Values(value)
    }
}

/// Convert a single value or an array into an array.
///
/// Mirrors `toArray` from `lib/util/array.ts`.
pub fn to_array<T>(value: impl Into<ToArrayInput<T>>) -> Vec<T> {
    match value.into() {
        ToArrayInput::Value(value) => vec![value],
        ToArrayInput::Values(values) => values,
    }
}

/// Preserve first-occurrence order while removing duplicates.
///
/// Mirrors `deduplicateArray` from `lib/util/array.ts`.
pub fn deduplicate_array<T: Eq + Hash + Clone>(array: &[T]) -> Vec<T> {
    deduplicate(array)
}

/// Deduplicate array elements preserving first-occurrence order.
///
/// Mirrors `deduplicate` from `lib/util/array.ts`.
pub fn deduplicate<T: Eq + Hash + Clone>(arr: &[T]) -> Vec<T> {
    let mut seen = HashSet::new();
    let mut result = Vec::with_capacity(arr.len());
    for item in arr {
        if seen.insert(item.clone()) {
            result.push(item.clone());
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_at_finds_match() {
        let arr = [1, 2, 3, 4, 5];
        let (before, after) = split_at(&arr, |x| *x == 3);
        assert_eq!(before, vec![1, 2]);
        assert_eq!(after, vec![3, 4, 5]);
    }

    #[test]
    fn split_at_no_match() {
        let arr = [1, 2, 3];
        let (before, after) = split_at(&arr, |x| *x == 10);
        assert_eq!(before, vec![1, 2, 3]);
        assert!(after.is_empty());
    }

    #[test]
    fn split_at_first_element() {
        let arr = [1, 2, 3];
        let (before, after) = split_at(&arr, |x| *x == 1);
        assert!(before.is_empty());
        assert_eq!(after, vec![1, 2, 3]);
    }

    #[test]
    fn split_at_last_element() {
        let arr = [1, 2, 3];
        let (before, after) = split_at(&arr, |x| *x == 3);
        assert_eq!(before, vec![1, 2]);
        assert_eq!(after, vec![3]);
    }

    #[test]
    fn split_at_empty() {
        let arr: Vec<i32> = vec![];
        let (before, after) = split_at(&arr, |x| *x == 1);
        assert!(before.is_empty());
        assert!(after.is_empty());
    }

    #[test]
    fn split_at_with_strings() {
        let arr = ["a", "b", "c"];
        let (before, after) = split_at(&arr, |s| *s == "b");
        assert_eq!(before, vec!["a"]);
        assert_eq!(after, vec!["b", "c"]);
    }

    #[test]
    fn deduplicate_removes_duplicates() {
        let arr = [1, 2, 3, 2, 1, 4];
        let result = deduplicate(&arr);
        assert_eq!(result, vec![1, 2, 3, 4]);
    }

    #[test]
    fn deduplicate_empty() {
        let arr: Vec<i32> = vec![];
        let result = deduplicate(&arr);
        assert!(result.is_empty());
    }

    #[test]
    fn deduplicate_no_duplicates() {
        let arr = [1, 2, 3];
        let result = deduplicate(&arr);
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn deduplicate_all_same() {
        let arr = [5, 5, 5];
        let result = deduplicate(&arr);
        assert_eq!(result, vec![5]);
    }

    #[test]
    fn deduplicate_preserves_order() {
        let arr = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
        let result = deduplicate(&arr);
        assert_eq!(result, vec![3, 1, 4, 5, 9, 2, 6]);
    }

    #[test]
    fn deduplicate_with_strings() {
        let arr = ["foo", "bar", "foo", "baz", "bar"];
        let result = deduplicate(&arr);
        assert_eq!(result, vec!["foo", "bar", "baz"]);
    }

    #[test]
    fn array_helpers_match_renovate_util_array() {
        assert!(is_not_null_or_undefined(Some(1)));
        assert!(!is_not_null_or_undefined::<i32>(None));
        assert_eq!(coerce_array::<i32>(None), Vec::<i32>::new());
        assert_eq!(coerce_array(Some(vec![1, 2, 2])), vec![1, 2, 2]);

        let single = to_array(42);
        assert_eq!(single, vec![42]);
        let array_values: ToArrayInput<i32> = ToArrayInput::Values(vec![1, 2, 3]);
        assert_eq!(to_array::<i32>(array_values), vec![1, 2, 3]);

        let deduplicated = deduplicate_array(&["a", "b", "a", "c", "b"]);
        assert_eq!(deduplicated, vec!["a", "b", "c"]);
    }
}
