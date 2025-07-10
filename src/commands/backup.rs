use std::fs::{create_dir, rename};
use std::path::Path;

pub fn backup_log_file(path: &Path) {
    let file_name = path.file_name().expect("not a file");
    println!("Moving {}...", file_name.display());

    let mut new_path = path.to_owned().clone();
    new_path.pop();
    new_path = new_path.join(Path::new("bak"));
    if !new_path.is_dir() {
        create_dir(&new_path).expect("could not create backup dir");
    }
    new_path = new_path.join(file_name);

    match rename(&path, &new_path) {
        Ok(_result) => {
            print!("Moved.\n");
        }
        Err(err) => {
            print!("Failed: {}\n", err);
        }
    }
}
