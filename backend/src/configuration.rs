mod defaults;
mod user;

use std::io;
use user::create_configuration_dirs;

pub use defaults::*;
pub use user::{get_default_config_dir, get_default_temp_dir};

/// Creates and initializes the default configuration directories.
pub fn init_configuration() -> io::Result<()> {
    create_configuration_dirs()?;
    create_default_config(get_default_config_dir())?;
    // any additional initialization logic should go here
    Ok(())
}
