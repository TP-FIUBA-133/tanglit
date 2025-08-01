use std::{io, path::Path};

use include_dir::{Dir, include_dir};

static CONFIG_RESOURCES_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/resources/config");

pub fn create_default_config(config_dir_path: &Path) -> io::Result<()> {
    let config_subdir = &CONFIG_RESOURCES_DIR;

    config_subdir.extract_soft(config_dir_path)?;
    Ok(())
}
