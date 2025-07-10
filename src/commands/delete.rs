use std::fs::remove_file;
use std::path::Path;

pub fn delete_log_file(path: &Path) {
    print!("Removing {}...", path.display());
    match remove_file(&path) {
        Ok(_result) => {
            print!("Removed.\n");
        }
        Err(err) => {
            print!("Failed: {}\n", err);
        }
    }
}
