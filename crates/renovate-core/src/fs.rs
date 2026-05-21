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
