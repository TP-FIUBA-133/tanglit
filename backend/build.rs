use std::fs::File;
use std::path::Path;
use std::time::SystemTime;

fn main() {
    println!("cargo:rerun-if-changed=resources/config");

    // Touch configuration.rs to force its recompilation
    let config_file = Path::new("src/configuration/defaults.rs");
    if config_file.exists() {
        if let Ok(file) = File::options().write(true).open(config_file) {
            let now = SystemTime::now();
            if let Err(e) = file.set_times(
                std::fs::FileTimes::new()
                    .set_accessed(now)
                    .set_modified(now),
            ) {
                eprintln!("Warning: Failed to touch configuration.rs: {}", e);
            }
        }
    }
}
