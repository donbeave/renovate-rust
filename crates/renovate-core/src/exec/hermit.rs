use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::exec::error::ExecError;
use crate::exec::types::BinarySource;

pub fn is_hermit(binary_source: &BinarySource) -> bool {
    binary_source == &BinarySource::Hermit
}

pub fn find_hermit_cwd(start_dir: &Path) -> Option<PathBuf> {
    let mut dir = start_dir.to_path_buf();
    loop {
        let hermit_bin = dir.join("bin").join("hermit");
        if hermit_bin.exists() {
            return Some(dir);
        }
        match dir.parent() {
            Some(parent) => dir = parent.to_path_buf(),
            None => return None,
        }
    }
}

pub async fn get_hermit_envs(
    cwd: &Path,
    process_env: &HashMap<String, String>,
) -> Result<HashMap<String, String>, ExecError> {
    let hermit_dir = find_hermit_cwd(cwd).ok_or_else(|| {
        ExecError::new(
            "could not find hermit directory",
            "hermit env",
        )
    })?;

    let hermit_bin = hermit_dir.join("bin").join("hermit");
    let cmd = format!("{} env -r", hermit_bin.display());

    let opts = crate::exec::types::ExecOptions {
        cwd: Some(hermit_dir.to_string_lossy().to_string()),
        ..Default::default()
    };

    let result = crate::exec::raw::raw_exec(&cmd, &opts, process_env).await?;
    Ok(parse_hermit_env_output(&result.stdout))
}

/// Parse the stdout of `hermit env -r` into key-value pairs.
fn parse_hermit_env_output(output: &str) -> HashMap<String, String> {
    let mut envs = HashMap::new();
    for line in output.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((key, value)) = line.split_once('=') {
            envs.insert(key.trim().to_owned(), value.trim().to_owned());
        }
    }
    envs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_hermit_true() {
        assert!(is_hermit(&BinarySource::Hermit));
    }

    #[test]
    fn is_hermit_false() {
        assert!(!is_hermit(&BinarySource::Global));
        assert!(!is_hermit(&BinarySource::Docker));
        assert!(!is_hermit(&BinarySource::Install));
    }

    #[test]
    fn find_hermit_cwd_nonexistent() {
        let result = find_hermit_cwd(Path::new("/tmp/definitely_no_hermit_here"));
        assert!(result.is_none());
    }

    // Ported: ("$dir") === $expected (hermit: $hermitLocation)
    //         — util/exec/hermit.spec.ts line 30
    #[test]
    fn find_hermit_cwd_in_same_dir() {
        let dir = tempfile::tempdir().unwrap();
        let hermit_bin = dir.path().join("bin").join("hermit");
        std::fs::create_dir_all(hermit_bin.parent().unwrap()).unwrap();
        std::fs::write(&hermit_bin, "").unwrap();
        let result = find_hermit_cwd(dir.path());
        assert_eq!(result, Some(dir.path().to_path_buf()));
    }

    #[test]
    fn find_hermit_cwd_in_nested_dir() {
        let dir = tempfile::tempdir().unwrap();
        let hermit_bin = dir.path().join("bin").join("hermit");
        std::fs::create_dir_all(hermit_bin.parent().unwrap()).unwrap();
        std::fs::write(&hermit_bin, "").unwrap();
        let nested = dir.path().join("nested").join("other").join("directory");
        std::fs::create_dir_all(&nested).unwrap();
        let result = find_hermit_cwd(&nested);
        assert_eq!(result, Some(dir.path().to_path_buf()));
    }

    #[test]
    fn find_hermit_cwd_in_parent_dir() {
        let dir = tempfile::tempdir().unwrap();
        let hermit_bin = dir.path().join("bin").join("hermit");
        std::fs::create_dir_all(hermit_bin.parent().unwrap()).unwrap();
        std::fs::write(&hermit_bin, "").unwrap();
        let nested = dir.path().join("nested");
        std::fs::create_dir_all(&nested).unwrap();
        let result = find_hermit_cwd(&nested);
        assert_eq!(result, Some(dir.path().to_path_buf()));
    }

    // Ported: "should return hermit environment variables when hermit env returns successfully"
    //         — util/exec/hermit.spec.ts line 62
    #[test]
    fn parse_hermit_env_output_parses_valid_lines() {
        let output = "GOBIN=/usr/src/app/repository-a/.hermit/go/bin\nPATH=/usr/src/app/repository-a/bin\n";
        let envs = parse_hermit_env_output(output);
        assert_eq!(envs.get("GOBIN").unwrap(), "/usr/src/app/repository-a/.hermit/go/bin");
        assert_eq!(envs.get("PATH").unwrap(), "/usr/src/app/repository-a/bin");
    }

    #[test]
    fn parse_hermit_env_output_skips_comments_and_empty_lines() {
        let output = "# comment\n\nFOO=bar\n\n# another\nBAZ=qux\n";
        let envs = parse_hermit_env_output(output);
        assert_eq!(envs.len(), 2);
        assert_eq!(envs.get("FOO").unwrap(), "bar");
        assert_eq!(envs.get("BAZ").unwrap(), "qux");
    }

    #[test]
    fn parse_hermit_env_output_empty() {
        let envs = parse_hermit_env_output("");
        assert!(envs.is_empty());
    }
}
