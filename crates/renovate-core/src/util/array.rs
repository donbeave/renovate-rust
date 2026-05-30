//! Array utility functions — mirrors `lib/util/array.ts`.

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
}
