use std::fs::remove_file;
use std::path::Path;

pub fn delete_log_file(path: &Path) {
    print!("Removing {}...", path.display());
    match remove_file(path) {
        Ok(_result) => {
            println!("Removed.");
        }
        Err(err) => {
            println!("Failed: {err}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_delete_log_file_removes_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.log");

        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "test content").unwrap();

        assert!(file_path.exists());

        delete_log_file(&file_path);

        assert!(!file_path.exists());
    }

    #[test]
    fn test_delete_log_file_with_special_characters() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test file with spaces.log");

        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "test content").unwrap();

        assert!(file_path.exists());

        delete_log_file(&file_path);

        assert!(!file_path.exists());
    }

    #[test]
    fn test_delete_log_file_empty_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("empty.log");

        File::create(&file_path).unwrap();

        assert!(file_path.exists());

        delete_log_file(&file_path);

        assert!(!file_path.exists());
    }

    #[test]
    fn test_delete_log_file_large_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("large.log");

        let mut file = File::create(&file_path).unwrap();
        for i in 0..1000 {
            writeln!(file, "Line {} with some content", i).unwrap();
        }

        assert!(file_path.exists());

        delete_log_file(&file_path);

        assert!(!file_path.exists());
    }

    #[test]
    fn test_delete_log_file_nonexistent() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("nonexistent.log");

        assert!(!file_path.exists());

        delete_log_file(&file_path);

        assert!(!file_path.exists());
    }
}
