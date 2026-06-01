use rand::Rng;
use rand::seq::SliceRandom;

pub fn sample_one<T: Clone>(items: &[T]) -> Option<T> {
    if items.is_empty() {
        return None;
    }
    let mut rng = rand::rng();
    let idx = rng.random_range(0..items.len());
    Some(items[idx].clone())
}

pub fn shuffle<T: Clone>(items: &mut [T]) {
    let mut rng = rand::rng();
    items.shuffle(&mut rng);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_one_returns_some() {
        let items = vec![1, 2, 3];
        let result = sample_one(&items);
        assert!(result.is_some());
        assert!(items.contains(&result.unwrap()));
    }

    #[test]
    fn sample_one_empty_returns_none() {
        let items: Vec<i32> = vec![];
        assert!(sample_one(&items).is_none());
    }

    #[test]
    fn sample_one_single_element() {
        let items = vec![42];
        assert_eq!(sample_one(&items), Some(42));
    }

    #[test]
    fn shuffle_preserves_elements() {
        let mut items = vec![1, 2, 3, 4, 5];
        let original = items.clone();
        shuffle(&mut items);
        let mut sorted = items.clone();
        sorted.sort();
        let mut original_sorted = original;
        original_sorted.sort();
        assert_eq!(sorted, original_sorted);
    }

    #[test]
    fn shuffle_empty() {
        let mut items: Vec<i32> = vec![];
        shuffle(&mut items);
        assert!(items.is_empty());
    }

    #[test]
    fn shuffle_single() {
        let mut items = vec![42];
        shuffle(&mut items);
        assert_eq!(items, vec![42]);
    }
}
