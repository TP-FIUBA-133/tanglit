mod user;

use std::io;
use user::create_configuration_dirs;

pub use user::{get_default_config_dir, get_default_temp_dir};

/// Creates and initializes the default configuration directories.
pub fn init_configuration() -> io::Result<()> {
    create_configuration_dirs()?;
    // any additional initialization logic should go here
    Ok(())
}
