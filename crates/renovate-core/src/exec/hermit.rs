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

    let mut envs = HashMap::new();
    for line in result.stdout.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((key, value)) = line.split_once('=') {
            envs.insert(key.trim().to_owned(), value.trim().to_owned());
        }
    }

    Ok(envs)
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
}
