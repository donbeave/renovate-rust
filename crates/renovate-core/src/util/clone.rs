//! Deep clone utilities — mirrors `lib/util/clone.ts`.
//!
//! Rust has the `Clone` trait natively, so this module provides simple wrappers
//! that ensure a full deep clone of a value.

/// Produce a deep clone of a `Clone`-able value.
///
/// In Rust, `Clone::clone()` already performs a deep clone for owned types.
/// This function exists as a named counterpart to the TypeScript `clone()`
/// utility from `lib/util/clone.ts`.
/// @parity lib/util/clone.ts full
pub fn deep_clone<T: Clone>(value: &T) -> T {
    value.clone()
}

/// Clone every element in a slice.
pub fn clone_slice<T: Clone>(slice: &[T]) -> Vec<T> {
    slice.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deep_clone_copies_value() {
        let original = String::from("hello");
        let cloned = deep_clone(&original);
        assert_eq!(cloned, original);
        assert_ne!(cloned.as_ptr(), original.as_ptr());
    }

    #[test]
    fn deep_clone_vec() {
        let original = vec![1, 2, 3];
        let cloned = deep_clone(&original);
        assert_eq!(cloned, original);
    }

    #[test]
    fn deep_clone_struct() {
        #[derive(Clone, PartialEq, Debug)]
        struct Foo {
            x: i32,
            s: String,
        }
        let original = Foo {
            x: 42,
            s: "test".to_owned(),
        };
        let cloned = deep_clone(&original);
        assert_eq!(cloned, original);
    }

    #[test]
    fn clone_slice_copies_all() {
        let data = [1, 2, 3];
        let cloned = clone_slice(&data);
        assert_eq!(cloned, vec![1, 2, 3]);
    }

    #[test]
    fn clone_slice_empty() {
        let data: [i32; 0] = [];
        let cloned = clone_slice(&data);
        assert!(cloned.is_empty());
    }
}
