use std::path::Path;

pub fn list_log_file(path: &Path) {
    let file_name = path.file_name().expect("not a file");
    println!("- {}", file_name.display());
}
