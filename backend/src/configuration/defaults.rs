use include_dir::{include_dir, Dir, DirEntry};
use std::{fs, io, path::Path};

static CONFIG_RESOURCES_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/resources/config");

pub fn extract_soft(source: &Dir, dest_path: &Path) -> io::Result<()> {
    for entry in source.entries() {
        let path = dest_path.join(entry.path());

        match entry {
            DirEntry::Dir(d) => {
                fs::create_dir_all(&path)?;
                extract_soft(d, dest_path)?;
            }
            DirEntry::File(file) => {
                if !path.exists() {
                    fs::write(path, file.contents())?;
                }
            }
        }
    }
    Ok(())
}

pub fn create_default_config(config_dir_path: &Path) -> io::Result<()> {
    fs::create_dir_all(config_dir_path)?;
    extract_soft(&CONFIG_RESOURCES_DIR, config_dir_path)?;
    Ok(())
}
