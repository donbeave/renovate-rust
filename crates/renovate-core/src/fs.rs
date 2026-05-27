/// Returns the parent directory of a file path.
///
/// Mirrors `getParentDir` from `lib/util/fs/index.ts`, which uses
/// `upath.parse(fileName).dir`. Trailing slashes are stripped before
/// computing the parent. Returns an empty string when there is no parent.
pub fn get_parent_dir(file_name: &str) -> String {
    let trimmed = file_name.trim_end_matches('/');

    if trimmed.is_empty() {
        return String::new();
    }

    match trimmed.rfind('/') {
        None => String::new(),
        Some(0) => String::from("/"),
        Some(pos) => trimmed[..pos].to_string(),
    }
}

/// Checks whether a relative or absolute `path` resolves to a location
/// inside `base_dir`.
///
/// Mirrors the path-validation logic in `isValidPath` from
/// `lib/util/fs/util.ts`. Backslashes are normalized to forward slashes
/// (matching `upath` behaviour). Returns `false` when the resolved path
/// escapes `base_dir`.
pub fn is_valid_path(path: &str, base_dir: &str) -> bool {
    // Normalize backslashes → forward slashes (upath behaviour)
    let normalized = path.replace('\\', "/");

    let resolved = if normalized.starts_with('/') {
        // Absolute path: use as-is (after normalizing)
        normalize_path(&normalized)
    } else {
        // Relative: join with base_dir then normalize
        normalize_path(&format!("{}/{}", base_dir, normalized))
    };

    resolved.starts_with(base_dir)
}

/// Resolves `..` and `.` segments in a Unix path string without touching the
/// filesystem. Does not handle symlinks.
fn normalize_path(path: &str) -> String {
    let mut parts: Vec<&str> = Vec::new();
    for segment in path.split('/') {
        match segment {
            "" | "." => {}
            ".." => {
                parts.pop();
            }
            other => parts.push(other),
        }
    }
    format!("/{}", parts.join("/"))
}

/// Returns a sibling file path in the same directory as `file_name`.
///
/// Mirrors `getSiblingFileName` from `lib/util/fs/index.ts`, which uses
/// `upath.join(getParentDir(fileName), siblingName)`. Note that joining
/// `.` with a sibling resolves to the sibling directly (like `path.join`).
pub fn get_sibling_file_name(file_name: &str, sibling_name: &str) -> String {
    let parent = get_parent_dir(file_name);
    match parent.as_str() {
        "" | "." => sibling_name.to_string(),
        _ => format!("{}/{}", parent, sibling_name),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "isValidPath($value) == $expected" — util/fs/util.spec.ts line 53
    #[test]
    fn is_valid_path_cases() {
        let base = "/bar";
        let cases = [
            (".", true),
            ("./...", true),
            ("foo", true),
            ("foo/bar", true),
            ("./foo/bar", true),
            ("./foo/bar/...", true),
            ("..", false),
            ("....", true),
            ("./foo/..", true),
            ("./foo/..../bar", true),
            ("./..", false),
            ("\\foo", false), // backslash → treated as /foo (absolute)
            ("foo'", true),
            ("fo\"o", true),
            ("fo&o", true),
            ("f;oo", true),
            ("f o o", true),
            ("/", false),
            ("/foo", false),
            ("&&", true),
            (";", true),
            ("./[foo]/bar", true),
        ];
        for (path, expected) in cases {
            assert_eq!(
                is_valid_path(path, base),
                expected,
                "is_valid_path({:?}, {:?})",
                path,
                base
            );
        }
    }

    // Ported: "('$dir') -> '$expected'" — util/fs/index.spec.ts line 77
    #[test]
    fn get_parent_dir_cases() {
        let cases = [
            ("/foo/bar/", "/foo"),
            ("/foo/bar", "/foo"),
            ("/foo/", "/"),
            ("/foo", "/"),
            ("foo/bar/", "foo"),
            ("foo/bar", "foo"),
            ("foo/", ""),
            ("foo", ""),
            ("", ""),
            (".", ""),
            ("..", ""),
            ("./foo", "."),
            ("../foo", ".."),
        ];
        for (input, expected) in cases {
            assert_eq!(
                get_parent_dir(input),
                expected,
                "get_parent_dir({:?})",
                input
            );
        }
    }

    // Ported: "('$file', '$sibling') -> '$expected'" — util/fs/index.spec.ts line 98
    #[test]
    fn get_sibling_file_name_cases() {
        let cases = [
            ("/foo/bar", "baz", "/foo/baz"),
            ("foo/bar", "baz", "foo/baz"),
            ("foo/", "baz", "baz"),
            ("foo", "baz", "baz"),
            ("./foo", "baz", "baz"),
            ("../foo", "baz", "../baz"),
        ];
        for (file, sibling, expected) in cases {
            assert_eq!(
                get_sibling_file_name(file, sibling),
                expected,
                "get_sibling_file_name({:?}, {:?})",
                file,
                sibling
            );
        }
    }
}
