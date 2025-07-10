use std::fs::remove_file;
use std::path::Path;

pub fn delete_log_file(path: &Path) {
    print!("Removing {}...", path.display());
    match remove_file(path) {
        Ok(_result) => {
            println!("Removed.");
        }
        Err(err) => {
            println!("Failed: {}", err);
        }
    }
}
