//! String splitting utilities — mirrors `lib/util/split.ts`.

/// Split a string on whitespace, returning each word as a `String`.
pub fn split_at_whitespace(s: &str) -> Vec<String> {
    s.split_whitespace().map(str::to_owned).collect()
}

/// Split a comma-separated string, trimming each element.
///
/// Empty elements after trimming are removed.
pub fn split_csv(s: &str) -> Vec<String> {
    s.split(',')
        .map(|part| part.trim().to_owned())
        .filter(|part| !part.is_empty())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_at_whitespace_basic() {
        assert_eq!(
            split_at_whitespace("hello world foo"),
            vec!["hello", "world", "foo"]
        );
    }

    #[test]
    fn split_at_whitespace_multiple_spaces() {
        assert_eq!(
            split_at_whitespace("  hello   world  "),
            vec!["hello", "world"]
        );
    }

    #[test]
    fn split_at_whitespace_tabs() {
        assert_eq!(split_at_whitespace("hello\tworld"), vec!["hello", "world"]);
    }

    #[test]
    fn split_at_whitespace_newlines() {
        assert_eq!(split_at_whitespace("hello\nworld"), vec!["hello", "world"]);
    }

    #[test]
    fn split_at_whitespace_empty() {
        let result = split_at_whitespace("");
        assert!(result.is_empty());
    }

    #[test]
    fn split_at_whitespace_only_whitespace() {
        let result = split_at_whitespace("   \t  \n  ");
        assert!(result.is_empty());
    }

    #[test]
    fn split_at_whitespace_single_word() {
        assert_eq!(split_at_whitespace("hello"), vec!["hello"]);
    }

    #[test]
    fn split_csv_basic() {
        assert_eq!(split_csv("a, b, c"), vec!["a", "b", "c"]);
    }

    #[test]
    fn split_csv_no_spaces() {
        assert_eq!(split_csv("a,b,c"), vec!["a", "b", "c"]);
    }

    #[test]
    fn split_csv_trailing_comma() {
        assert_eq!(split_csv("a, b,"), vec!["a", "b"]);
    }

    #[test]
    fn split_csv_leading_comma() {
        assert_eq!(split_csv(",a,b"), vec!["a", "b"]);
    }

    #[test]
    fn split_csv_empty() {
        let result = split_csv("");
        assert!(result.is_empty());
    }

    #[test]
    fn split_csv_only_commas() {
        let result = split_csv(",,,");
        assert!(result.is_empty());
    }

    #[test]
    fn split_csv_spaces_only() {
        let result = split_csv(" , , ");
        assert!(result.is_empty());
    }

    #[test]
    fn split_csv_single_element() {
        assert_eq!(split_csv("hello"), vec!["hello"]);
    }

    #[test]
    fn split_csv_preserves_inner_spaces() {
        assert_eq!(
            split_csv("hello world, foo bar"),
            vec!["hello world", "foo bar"]
        );
    }

    #[test]
    fn split_csv_trims_each_element() {
        assert_eq!(split_csv("  a  ,  b  ,  c  "), vec!["a", "b", "c"]);
    }
}
