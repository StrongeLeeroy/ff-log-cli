use std::path::Path;

pub fn list_log_file(path: &Path) {
    let file_name = path.file_name().expect("not a file");
    println!("- {}", file_name.display());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_list_log_file_basic() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.log");
        
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "test content").unwrap();
        
        list_log_file(&file_path);
    }

    #[test]
    fn test_list_log_file_with_special_characters() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test file with spaces.log");
        
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "test content").unwrap();
        
        list_log_file(&file_path);
    }

    #[test]
    fn test_list_log_file_empty_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("empty.log");
        
        File::create(&file_path).unwrap();
        
        list_log_file(&file_path);
    }

    #[test]
    fn test_list_log_file_different_extensions() {
        let temp_dir = TempDir::new().unwrap();
        
        let test_files = vec![
            "test.log",
            "combat.txt",
            "ffxiv.dat",
            "no_extension"
        ];
        
        for file_name in test_files {
            let file_path = temp_dir.path().join(file_name);
            File::create(&file_path).unwrap();
            list_log_file(&file_path);
        }
    }

    #[test]
    fn test_list_log_file_unicode_filename() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("測試檔案.log");
        
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "test content").unwrap();
        
        list_log_file(&file_path);
    }

    #[test]
    #[should_panic(expected = "not a file")]
    fn test_list_log_file_invalid_path() {
        let temp_dir = TempDir::new().unwrap();
        let invalid_path = temp_dir.path();
        
        list_log_file(&invalid_path);
    }
}
