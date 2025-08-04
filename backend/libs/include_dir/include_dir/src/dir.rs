use crate::{file::File, DirEntry};
use std::fs;
use std::path::Path;

/// A directory.
#[derive(Debug, Clone, PartialEq)]
pub struct Dir<'a> {
    path: &'a str,
    entries: &'a [DirEntry<'a>],
}

impl<'a> Dir<'a> {
    /// Create a new [`Dir`].
    pub const fn new(path: &'a str, entries: &'a [DirEntry<'a>]) -> Self {
        Dir { path, entries }
    }

    /// The full path for this [`Dir`], relative to the directory passed to
    /// [`crate::include_dir!()`].
    pub fn path(&self) -> &'a Path {
        Path::new(self.path)
    }

    /// The entries within this [`Dir`].
    pub const fn entries(&self) -> &'a [DirEntry<'a>] {
        self.entries
    }

    /// Get a list of the files in this directory.
    pub fn files(&self) -> impl Iterator<Item = &'a File<'a>> + 'a {
        self.entries().iter().filter_map(DirEntry::as_file)
    }

    /// Get a list of the sub-directories inside this directory.
    pub fn dirs(&self) -> impl Iterator<Item = &'a Dir<'a>> + 'a {
        self.entries().iter().filter_map(DirEntry::as_dir)
    }

    /// Recursively search for a [`DirEntry`] with a particular path.
    pub fn get_entry<S: AsRef<Path>>(&self, path: S) -> Option<&'a DirEntry<'a>> {
        let path = path.as_ref();

        for entry in self.entries() {
            if entry.path() == path {
                return Some(entry);
            }

            if let DirEntry::Dir(d) = entry {
                if let Some(nested) = d.get_entry(path) {
                    return Some(nested);
                }
            }
        }

        None
    }

    /// Look up a file by name.
    pub fn get_file<S: AsRef<Path>>(&self, path: S) -> Option<&'a File<'a>> {
        self.get_entry(path).and_then(DirEntry::as_file)
    }

    /// Look up a dir by name.
    pub fn get_dir<S: AsRef<Path>>(&self, path: S) -> Option<&'a Dir<'a>> {
        self.get_entry(path).and_then(DirEntry::as_dir)
    }

    /// Does this directory contain `path`?
    pub fn contains<S: AsRef<Path>>(&self, path: S) -> bool {
        self.get_entry(path).is_some()
    }

    /// Create directories and extract all files to real filesystem.
    /// Creates parent directories of `path` if they do not already exist.
    /// Fails if some files already exist.
    /// In case of error, partially extracted directory may remain on the filesystem.
    pub fn extract<S: AsRef<Path>>(&self, base_path: S) -> std::io::Result<()> {
        let base_path = base_path.as_ref();

        for entry in self.entries() {
            let path = base_path.join(entry.path());

            match entry {
                DirEntry::Dir(d) => {
                    fs::create_dir_all(&path)?;
                    d.extract(base_path)?;
                }
                DirEntry::File(f) => {
                    fs::write(&path, f.contents())?;
                    set_file_permissions(f, &path)?;
                }
            }
        }

        Ok(())
    }

    /// Create directories and extract all files to real filesystem.
    /// Creates parent directories of `path` if they do not already exist.
    /// Skips over files that already exist.
    /// In case of error, partially extracted directory may remain on the filesystem.
    pub fn extract_soft<S: AsRef<Path>>(&self, base_path: S) -> std::io::Result<()> {
        let base_path = base_path.as_ref();

        for entry in self.entries() {
            let path = base_path.join(entry.path());

            match entry {
                DirEntry::Dir(d) => {
                    fs::create_dir_all(&path)?;
                    d.extract_soft(base_path)?;
                }
                DirEntry::File(f) => {
                    if !path.exists() {
                        fs::write(&path, f.contents())?;
                        set_file_permissions(f, &path)?;
                    }
                }
            }
        }

        Ok(())
    }
}

#[cfg(all(unix, feature = "metadata"))]
fn set_file_permissions(file: &File<'_>, path: &Path) -> std::io::Result<()> {
    use std::os::unix::fs::PermissionsExt;
    if let Some(metadata) = file.metadata() {
        if metadata.is_executable() {
            let mut permissions = fs::metadata(path)?.permissions();
            let current_mode = permissions.mode();
            // Add execute bits (owner, group, other) to existing permissions
            let new_mode = current_mode | 0o111; // 0o111 = execute bits for owner, group, and others
            permissions.set_mode(new_mode);
            fs::set_permissions(path, permissions)?;
        }
    }
    Ok(())
}

#[cfg(not(all(unix, feature = "metadata")))]
fn set_file_permissions(_file: &File, _path: &Path) -> std::io::Result<()> {
    // No-op on non-Unix platforms or when metadata feature is disabled
    // In Windows, file execution is given by the extension
    Ok(())
}
