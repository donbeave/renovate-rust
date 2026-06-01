use std::fs;
use std::io;
use std::path::{Path, PathBuf};

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

fn resolve_under_base(base_dir: &Path, path: &str) -> Result<PathBuf, io::Error> {
    ensure_base_path(path, &base_dir.to_string_lossy())
        .map(PathBuf::from)
        .map_err(|err| io::Error::new(io::ErrorKind::PermissionDenied, err))
}

pub fn read_local_file(base_dir: &Path, path: &str) -> io::Result<Option<Vec<u8>>> {
    let full_path = resolve_under_base(base_dir, path)?;
    match fs::read(full_path) {
        Ok(content) => Ok(Some(content)),
        Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(err) => Err(err),
    }
}

pub fn read_local_string(base_dir: &Path, path: &str) -> io::Result<Option<String>> {
    let Some(content) = read_local_file(base_dir, path)? else {
        return Ok(None);
    };
    String::from_utf8(content)
        .map(Some)
        .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))
}

pub fn write_local_file(base_dir: &Path, path: &str, content: impl AsRef<[u8]>) -> io::Result<()> {
    let full_path = resolve_under_base(base_dir, path)?;
    if let Some(parent) = full_path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(full_path, content)
}

pub fn delete_local_file(base_dir: &Path, path: &str) -> io::Result<()> {
    let full_path = resolve_under_base(base_dir, path)?;
    fs::remove_file(full_path)
}

pub fn rename_local_file(base_dir: &Path, source: &str, target: &str) -> io::Result<()> {
    let source_path = resolve_under_base(base_dir, source)?;
    let target_path = resolve_under_base(base_dir, target)?;
    if let Some(parent) = target_path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::rename(source_path, target_path)
}

pub fn ensure_dir(path: &Path) -> io::Result<()> {
    fs::create_dir_all(path)
}

pub fn ensure_local_dir(base_dir: &Path, path: &str) -> io::Result<PathBuf> {
    let full_path = resolve_under_base(base_dir, path)?;
    ensure_dir(&full_path)?;
    Ok(full_path)
}

pub fn private_cache_dir(cache_dir: &Path) -> PathBuf {
    cache_dir.join("__renovate-private-cache")
}

pub fn ensure_cache_dir(cache_dir: &Path, namespace: &str) -> io::Result<PathBuf> {
    let full_path = resolve_under_base(cache_dir, &format!("others/{namespace}"))?;
    ensure_dir(&full_path)?;
    Ok(full_path)
}

pub fn local_path_exists(base_dir: &Path, path: &str) -> io::Result<bool> {
    let full_path = resolve_under_base(base_dir, path)?;
    Ok(full_path.exists())
}

pub fn local_path_is_file(base_dir: &Path, path: &str) -> io::Result<bool> {
    let full_path = resolve_under_base(base_dir, path)?;
    Ok(full_path.is_file())
}

pub fn local_path_is_symbolic_link(base_dir: &Path, path: &str) -> io::Result<bool> {
    let full_path = resolve_under_base(base_dir, path)?;
    Ok(fs::symlink_metadata(full_path)
        .map(|metadata| metadata.file_type().is_symlink())
        .unwrap_or(false))
}

pub fn read_local_symlink(base_dir: &Path, path: &str) -> io::Result<Option<PathBuf>> {
    let full_path = resolve_under_base(base_dir, path)?;
    match fs::read_link(full_path) {
        Ok(target) => Ok(Some(target)),
        Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(err) => Err(err),
    }
}

pub fn read_local_directory(base_dir: &Path, path: &str) -> io::Result<Vec<String>> {
    let full_path = resolve_under_base(base_dir, path)?;
    let mut entries = Vec::new();
    for entry in fs::read_dir(full_path)? {
        let entry = entry?;
        entries.push(entry.file_name().to_string_lossy().into_owned());
    }
    entries.sort();
    Ok(entries)
}

pub fn find_local_sibling_or_parent(
    base_dir: &Path,
    file_name: &str,
    sibling_name: &str,
) -> io::Result<Option<String>> {
    if Path::new(file_name).is_absolute() || Path::new(sibling_name).is_absolute() {
        return Ok(None);
    }

    let mut parent = get_parent_dir(file_name);
    loop {
        let candidate = if parent.is_empty() {
            sibling_name.to_owned()
        } else {
            format!("{parent}/{sibling_name}")
        };
        if local_path_exists(base_dir, &candidate)? {
            return Ok(Some(candidate));
        }
        if parent.is_empty() {
            return Ok(None);
        }
        parent = get_parent_dir(&parent);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

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

    // Ported: "reads buffer" — util/fs/index.spec.ts line 112
    // Ported: "reads string" — util/fs/index.spec.ts line 118
    // Ported: "returns null if file is not found" — util/fs/index.spec.ts line 124
    #[test]
    fn local_file_read_cases() {
        let dir = tempdir().unwrap();
        write_local_file(dir.path(), "file.txt", "foobar").unwrap();

        assert_eq!(
            read_local_file(dir.path(), "file.txt").unwrap(),
            Some(b"foobar".to_vec())
        );
        assert_eq!(
            read_local_string(dir.path(), "file.txt")
                .unwrap()
                .as_deref(),
            Some("foobar")
        );
        assert_eq!(read_local_file(dir.path(), "missing.txt").unwrap(), None);
    }

    // Ported: "outputs file" — util/fs/index.spec.ts line 203
    // Ported: "deletes file" — util/fs/index.spec.ts line 218
    // Ported: "renames file" — util/fs/index.spec.ts line 229
    #[test]
    fn local_file_write_delete_and_rename() {
        let dir = tempdir().unwrap();
        write_local_file(dir.path(), "foo/bar/file.txt", "foobar").unwrap();
        assert_eq!(
            fs::read_to_string(dir.path().join("foo/bar/file.txt")).unwrap(),
            "foobar"
        );

        rename_local_file(dir.path(), "foo/bar/file.txt", "foo/bar/renamed.txt").unwrap();
        assert!(!dir.path().join("foo/bar/file.txt").exists());
        assert!(dir.path().join("foo/bar/renamed.txt").is_file());

        delete_local_file(dir.path(), "foo/bar/renamed.txt").unwrap();
        assert!(!dir.path().join("foo/bar/renamed.txt").exists());
    }

    // Ported: "creates directory" — util/fs/index.spec.ts line 243
    // Ported: "creates local directory" — util/fs/index.spec.ts line 253
    // Ported: "prefers environment variables over global config" — util/fs/index.spec.ts line 263
    // Ported: "returns cache dir" — util/fs/index.spec.ts line 272
    #[test]
    fn directory_and_cache_helpers() {
        let local = tempdir().unwrap();
        let cache = tempdir().unwrap();

        ensure_dir(&local.path().join("foo/bar")).unwrap();
        assert!(local.path().join("foo/bar").is_dir());

        let local_dir = ensure_local_dir(local.path(), "baz/qux").unwrap();
        assert_eq!(local_dir, local.path().join("baz/qux"));
        assert!(local_dir.is_dir());

        let cache_dir = ensure_cache_dir(cache.path(), "bundler").unwrap();
        assert_eq!(cache_dir, cache.path().join("others/bundler"));
        assert!(cache_dir.is_dir());
        assert_eq!(
            private_cache_dir(cache.path()),
            cache.path().join("__renovate-private-cache")
        );
    }

    // Ported: "returns true for file" — util/fs/index.spec.ts line 279
    // Ported: "returns true for directory" — util/fs/index.spec.ts line 285
    // Ported: "returns false" — util/fs/index.spec.ts line 289
    // Ported: "returns true for valid local path" — util/fs/index.spec.ts line 295
    // Ported: "returns false" — util/fs/index.spec.ts line 299
    #[test]
    fn local_path_status_helpers() {
        let dir = tempdir().unwrap();
        write_local_file(dir.path(), "file.txt", "foobar").unwrap();

        assert!(local_path_exists(dir.path(), "file.txt").unwrap());
        assert!(local_path_exists(dir.path(), ".").unwrap());
        assert!(!local_path_exists(dir.path(), "missing.txt").unwrap());
        assert!(local_path_is_file(dir.path(), "file.txt").unwrap());
        assert!(!local_path_is_file(dir.path(), ".").unwrap());
        assert!(!local_path_is_file(dir.path(), "missing.txt").unwrap());
        assert!(is_valid_path("./foo/...", &dir.path().to_string_lossy()));
        assert!(!is_valid_path("/file.txt", &dir.path().to_string_lossy()));
    }

    // Ported: "reads symlink" — util/fs/index.spec.ts line 305
    // Ported: "return null when link not exists" — util/fs/index.spec.ts line 317
    // Ported: "returns true for symlink" — util/fs/index.spec.ts line 453
    #[cfg(unix)]
    #[test]
    fn local_symlink_helpers() {
        let dir = tempdir().unwrap();
        write_local_file(dir.path(), "test/test.txt", "").unwrap();
        std::os::unix::fs::symlink(
            dir.path().join("test/test.txt"),
            dir.path().join("test/test"),
        )
        .unwrap();

        assert!(
            read_local_symlink(dir.path(), "test/test")
                .unwrap()
                .is_some()
        );
        assert_eq!(
            read_local_symlink(dir.path(), "test/not-exists").unwrap(),
            None
        );
        assert!(local_path_is_symbolic_link(dir.path(), "test/test").unwrap());
        assert!(!local_path_is_symbolic_link(dir.path(), "test/test.txt").unwrap());
        ensure_local_dir(dir.path(), "test/dir").unwrap();
        assert!(!local_path_is_symbolic_link(dir.path(), "test/dir").unwrap());
        assert!(!local_path_is_symbolic_link(dir.path(), "missing").unwrap());
    }

    // Ported: "returns path for file" — util/fs/index.spec.ts line 331
    // Ported: "immediately returns null when either path is absolute" — util/fs/index.spec.ts line 355
    #[test]
    fn find_local_sibling_or_parent_cases() {
        let dir = tempdir().unwrap();
        write_local_file(dir.path(), "crates/one/Cargo.toml", "foo").unwrap();
        write_local_file(dir.path(), "Cargo.lock", "bar").unwrap();

        assert_eq!(
            find_local_sibling_or_parent(dir.path(), "crates/one/Cargo.toml", "Cargo.lock")
                .unwrap()
                .as_deref(),
            Some("Cargo.lock")
        );
        assert_eq!(
            find_local_sibling_or_parent(dir.path(), "crates/one/Cargo.toml", "Cargo.mock")
                .unwrap(),
            None
        );

        write_local_file(dir.path(), "crates/one/Cargo.lock", "").unwrap();
        assert_eq!(
            find_local_sibling_or_parent(dir.path(), "crates/one/Cargo.toml", "Cargo.lock")
                .unwrap()
                .as_deref(),
            Some("crates/one/Cargo.lock")
        );
        assert_eq!(
            find_local_sibling_or_parent(dir.path(), "/etc/hosts", "other").unwrap(),
            None
        );
        assert_eq!(
            find_local_sibling_or_parent(dir.path(), "other", "/etc/hosts").unwrap(),
            None
        );
    }

    // Ported: "returns dir content" — util/fs/index.spec.ts line 362
    // Ported: "return empty array for non existing directory" — util/fs/index.spec.ts line 380
    // Ported: "return empty array for a existing but empty directory" — util/fs/index.spec.ts line 384
    #[test]
    fn read_local_directory_cases() {
        let dir = tempdir().unwrap();
        write_local_file(dir.path(), "test/Cargo.toml", "").unwrap();
        write_local_file(dir.path(), "test/Cargo.lock", "").unwrap();

        assert_eq!(
            read_local_directory(dir.path(), "test").unwrap(),
            vec!["Cargo.lock".to_owned(), "Cargo.toml".to_owned()]
        );
        write_local_file(dir.path(), "test/subdir/Cargo.lock", "").unwrap();
        assert_eq!(
            read_local_directory(dir.path(), "test").unwrap(),
            vec![
                "Cargo.lock".to_owned(),
                "Cargo.toml".to_owned(),
                "subdir".to_owned()
            ]
        );
        assert!(read_local_directory(dir.path(), "missing").is_err());

        ensure_local_dir(dir.path(), "empty").unwrap();
        assert!(
            read_local_directory(dir.path(), "empty")
                .unwrap()
                .is_empty()
        );
    }
}
