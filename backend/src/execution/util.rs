use std::fs;
use std::path::{Path, PathBuf};

pub fn find_file_in_dir(dir: &Path, filename: &str) -> Option<PathBuf> {
    fs::read_dir(dir).ok().and_then(|entries| {
        entries
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .find(|path| {
                path.file_stem()
                    .map(|stem| stem == filename)
                    .unwrap_or(false)
            })
    })
}
