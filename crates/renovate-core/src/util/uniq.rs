use std::collections::HashSet;
use std::hash::Hash;

pub fn uniq<T: Eq + Hash + Clone>(items: &[T]) -> Vec<T> {
    let mut seen = HashSet::new();
    let mut result = Vec::with_capacity(items.len());
    for item in items {
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
    fn uniq_removes_duplicates() {
        assert_eq!(uniq(&[1, 2, 3, 2, 1]), vec![1, 2, 3]);
    }

    #[test]
    fn uniq_preserves_order() {
        assert_eq!(
            uniq(&[3, 1, 4, 1, 5, 9, 2, 6, 5]),
            vec![3, 1, 4, 5, 9, 2, 6]
        );
    }

    #[test]
    fn uniq_empty() {
        let empty: Vec<i32> = vec![];
        assert!(uniq(&empty).is_empty());
    }

    #[test]
    fn uniq_no_duplicates() {
        assert_eq!(uniq(&[1, 2, 3]), vec![1, 2, 3]);
    }

    #[test]
    fn uniq_all_same() {
        assert_eq!(uniq(&[5, 5, 5]), vec![5]);
    }

    #[test]
    fn uniq_strings() {
        assert_eq!(
            uniq(&["foo", "bar", "foo", "baz"]),
            vec!["foo", "bar", "baz"]
        );
    }
}
