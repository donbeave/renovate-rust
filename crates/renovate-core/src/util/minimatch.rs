use globset::{Glob, GlobSetBuilder};

pub fn minimatch(input: &str, pattern: &str) -> bool {
    if pattern == "*" {
        return !input.contains('/');
    }
    if pattern == "**" {
        return true;
    }
    let Ok(glob) = Glob::new(pattern) else {
        return input == pattern;
    };
    let Ok(set) = GlobSetBuilder::new().add(glob).build() else {
        return false;
    };
    set.is_match(input)
}

pub fn is_minimatch_star(pattern: &str) -> bool {
    pattern == "*" || pattern == "**"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minimatch_star_matches_non_slash() {
        assert!(minimatch("foo", "*"));
    }

    #[test]
    fn minimatch_star_no_match_with_slash() {
        assert!(!minimatch("foo/bar", "*"));
    }

    #[test]
    fn minimatch_double_star_matches_all() {
        assert!(minimatch("foo/bar/baz", "**"));
    }

    #[test]
    fn minimatch_exact_match() {
        assert!(minimatch("foo", "foo"));
    }

    #[test]
    fn minimatch_no_match() {
        assert!(!minimatch("foo", "bar"));
    }

    #[test]
    fn minimatch_glob_pattern() {
        assert!(minimatch("foo.js", "*.js"));
    }

    #[test]
    fn minimatch_question_mark() {
        assert!(minimatch("f", "?"));
    }

    #[test]
    fn is_minimatch_star_true() {
        assert!(is_minimatch_star("*"));
        assert!(is_minimatch_star("**"));
    }

    #[test]
    fn is_minimatch_star_false() {
        assert!(!is_minimatch_star("*.js"));
        assert!(!is_minimatch_star("foo"));
    }
}
