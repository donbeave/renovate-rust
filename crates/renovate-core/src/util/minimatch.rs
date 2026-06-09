use globset::{Glob, GlobBuilder, GlobMatcher};
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct MinimatchOptions {
    pub dot: bool,
    pub nocase: bool,
    pub match_base: bool,
    pub noglobstar: bool,
}

/// Return `true` when the pattern is exactly `*` or `**`.
pub fn is_minimatch_star(pattern: &str) -> bool {
    pattern == "*" || pattern == "**"
}

#[derive(Debug, Clone)]
pub struct Minimatch {
    pattern: String,
    options: MinimatchOptions,
    is_glob_pattern: bool,
    matcher: Option<GlobMatcher>,
}

impl Minimatch {
    pub fn r#match(&self, file_name: &str) -> bool {
        let mut target = file_name;

        if self.options.match_base && !self.pattern.contains('/') {
            target = file_name.rsplit('/').next().unwrap_or(file_name);
        }

        if !self.options.dot
            && pattern_has_hidden_segments(target)
            && !self.pattern.starts_with('.')
        {
            return false;
        }

        if self.is_glob_pattern {
            if let Some(matcher) = &self.matcher {
                return matcher.is_match(target);
            }
            return if self.options.nocase {
                target.eq_ignore_ascii_case(&self.pattern)
            } else {
                target == self.pattern
            };
        }

        if self.options.nocase {
            target.eq_ignore_ascii_case(&self.pattern)
        } else {
            target == self.pattern
        }
    }
}

static MINIMATCH_CACHE: LazyLock<Mutex<HashMap<String, Minimatch>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

fn cache_key(pattern: &str, options: &MinimatchOptions) -> String {
    format!(
        "{pattern}:{:x}{:x}{:x}{:x}",
        u8::from(options.dot),
        u8::from(options.nocase),
        u8::from(options.match_base),
        u8::from(options.noglobstar)
    )
}

fn pattern_is_glob(pattern: &str) -> bool {
    pattern.chars().any(|ch| ['*', '?', '[', '{'].contains(&ch))
}

fn pattern_has_hidden_segments(pattern: &str) -> bool {
    pattern
        .split('/')
        .any(|segment| segment.starts_with('.') && segment != "." && segment != "..")
}

fn build_matcher(pattern: &str, options: &MinimatchOptions) -> Option<GlobMatcher> {
    if !pattern_is_glob(pattern) {
        return None;
    }

    let normalized = if options.noglobstar {
        pattern.replace("**", "*")
    } else {
        pattern.to_owned()
    };

    let mut builder = GlobBuilder::new(&normalized);
    builder.literal_separator(true);

    if options.nocase {
        builder.case_insensitive(true);
    }

    builder
        .build()
        .ok()
        .map(|glob: Glob| glob.compile_matcher())
}

/// @parity lib/util/minimatch.ts full
/// Create a compiled minimatch-like matcher with optional caching.
pub fn minimatch(pattern: &str, options: Option<&MinimatchOptions>, use_cache: bool) -> Minimatch {
    let options = options.copied().unwrap_or_default();
    let key = cache_key(pattern, &options);

    if use_cache {
        if let Ok(cache) = MINIMATCH_CACHE.lock() {
            if let Some(result) = cache.get(&key) {
                return result.clone();
            }
        }
    }

    let instance = Minimatch {
        pattern: pattern.to_owned(),
        is_glob_pattern: pattern_is_glob(pattern),
        options,
        matcher: build_matcher(pattern, &options),
    };

    if use_cache {
        if let Ok(mut cache) = MINIMATCH_CACHE.lock() {
            cache.insert(key, instance.clone());
        }
    }

    instance
}

/// Return a predicate for filename filtering.
pub fn minimatch_filter(
    pattern: &str,
    options: Option<&MinimatchOptions>,
    use_cache: bool,
) -> Box<dyn Fn(&str) -> bool + Send + Sync> {
    let matcher = minimatch(pattern, options, use_cache);
    Box::new(move |file_name| matcher.r#match(file_name))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cs_options() -> MinimatchOptions {
        MinimatchOptions {
            match_base: true,
            nocase: true,
            ..Default::default()
        }
    }

    #[test]
    fn minimatch_filter_matches() {
        assert!(minimatch("foo", None, true).r#match("foo"));
        assert!(minimatch("@opentelemetry{/,}**", None, true).r#match("@opentelemetry/http"));
        assert!(
            minimatch("*.{cs,vb,fs}proj", Some(&cs_options()), true).r#match("foo/bar.BAR.fsproj")
        );
        assert!(minimatch("foo", Some(&MinimatchOptions::default()), true).r#match("foo"));
        assert!(!minimatch("foo", Some(&MinimatchOptions::default()), false).r#match(".foo"));
    }

    #[test]
    fn minimatch_star_checks() {
        assert!(is_minimatch_star("*"));
        assert!(is_minimatch_star("**"));
    }
}
