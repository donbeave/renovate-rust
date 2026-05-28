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
        Some(pos) => trimmed[..pos].to_owned(),
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
        "" | "." => sibling_name.to_owned(),
        _ => format!("{}/{}", parent, sibling_name),
    }
}

pub const FILE_ACCESS_VIOLATION_ERROR: &str = "FILE_ACCESS_VIOLATION_ERROR";

/// Resolve `path` relative to `base_dir` and return the absolute path.
///
/// Returns `Err(FILE_ACCESS_VIOLATION_ERROR)` when the resolved path escapes
/// `base_dir`.  Mirrors `ensureLocalPath` / `ensureCachePath` from
/// `lib/util/fs/util.ts`.
pub fn ensure_base_path(path: &str, base_dir: &str) -> Result<String, &'static str> {
    let normalized = path.replace('\\', "/");
    let base = normalize_path(base_dir);
    let resolved = if normalized.is_empty() {
        base.clone()
    } else if normalized.starts_with('/') {
        normalize_path(&normalized)
    } else {
        normalize_path(&format!("{}/{}", base, normalized))
    };
    // Require resolved == base OR resolved starts with base + '/' to prevent
    // prefix confusion ("/foo" matching "/foobar").
    let within = resolved == base || resolved.starts_with(&format!("{}/", base));
    if !within {
        return Err(FILE_ACCESS_VIOLATION_ERROR);
    }
    Ok(resolved)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "ensureLocalPath('$path', '$fullPath')" — util/fs/util.spec.ts line 14
    // Ported: "ensureCachePath('$path', '$fullPath')" — util/fs/util.spec.ts line 33
    #[test]
    fn ensure_base_path_resolves_relative_paths() {
        let local_dir = "/foo";
        let cache_dir = "/bar";
        // Empty path → base_dir itself
        assert_eq!(ensure_base_path("", local_dir), Ok("/foo".to_owned()));
        assert_eq!(ensure_base_path("", cache_dir), Ok("/bar".to_owned()));
        // Relative subpath → joined with base
        assert_eq!(
            ensure_base_path("baz", local_dir),
            Ok("/foo/baz".to_owned())
        );
        assert_eq!(
            ensure_base_path("baz", cache_dir),
            Ok("/bar/baz".to_owned())
        );
    }

    // Ported: "ensureLocalPath('$path', '${localDir}') - throws" — util/fs/util.spec.ts line 22
    // Ported: "ensureCachePath('$path', '${cacheDir}') - throws" — util/fs/util.spec.ts line 41
    #[test]
    fn ensure_base_path_rejects_escaping_paths() {
        let local_dir = "/foo";
        let cache_dir = "/bar";
        for path in &[
            "..",
            "../etc/passwd",
            "/foo/../bar",
            "/foo/../../etc/passwd",
            "/baz",
        ] {
            assert_eq!(
                ensure_base_path(path, local_dir),
                Err(FILE_ACCESS_VIOLATION_ERROR),
                "ensure_base_path({path:?}, {local_dir:?}) should be Err"
            );
        }
        for path in &[
            "..",
            "../etc/passwd",
            "/bar/../foo",
            "/bar/../../etc/passwd",
            "/baz",
            r#"/baz""#,
        ] {
            assert_eq!(
                ensure_base_path(path, cache_dir),
                Err(FILE_ACCESS_VIOLATION_ERROR),
                "ensure_base_path({path:?}, {cache_dir:?}) should be Err"
            );
        }
        // Prefix confusion guard: /foobar must not match when base is /foo
        assert_eq!(
            ensure_base_path("/foobar", "/foo"),
            Err(FILE_ACCESS_VIOLATION_ERROR)
        );
        assert_eq!(
            ensure_base_path("../foobar/x", "/foo"),
            Err(FILE_ACCESS_VIOLATION_ERROR)
        );
    }

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
