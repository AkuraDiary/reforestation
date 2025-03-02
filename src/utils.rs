use std::fs;
use std::path::PathBuf;

pub fn resolve_path(target_dir: &str) -> PathBuf {
    fs::canonicalize(target_dir).unwrap_or_else(|_| target_dir.into())
}
