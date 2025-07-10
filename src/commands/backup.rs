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

    match rename(path, &new_path) {
        Ok(_result) => {
            println!("Moved.");
        }
        Err(err) => {
            println!("Failed: {err}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_backup_log_file_creates_bak_dir() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.log");
        
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "test content").unwrap();
        
        backup_log_file(&file_path);
        
        let bak_dir = temp_dir.path().join("bak");
        assert!(bak_dir.exists());
        assert!(bak_dir.is_dir());
    }

    #[test]
    fn test_backup_log_file_moves_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.log");
        
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "test content").unwrap();
        
        backup_log_file(&file_path);
        
        let backup_path = temp_dir.path().join("bak").join("test.log");
        assert!(!file_path.exists());
        assert!(backup_path.exists());
    }

    #[test]
    fn test_backup_log_file_preserves_content() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.log");
        let test_content = "test content\nline 2\nline 3";
        
        let mut file = File::create(&file_path).unwrap();
        write!(file, "{}", test_content).unwrap();
        
        backup_log_file(&file_path);
        
        let backup_path = temp_dir.path().join("bak").join("test.log");
        let backup_content = fs::read_to_string(&backup_path).unwrap();
        assert_eq!(backup_content, test_content);
    }

    #[test]
    fn test_backup_log_file_existing_bak_dir() {
        let temp_dir = TempDir::new().unwrap();
        let bak_dir = temp_dir.path().join("bak");
        fs::create_dir(&bak_dir).unwrap();
        
        let file_path = temp_dir.path().join("test.log");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "test content").unwrap();
        
        backup_log_file(&file_path);
        
        let backup_path = bak_dir.join("test.log");
        assert!(!file_path.exists());
        assert!(backup_path.exists());
    }

    #[test]
    fn test_backup_log_file_with_special_characters() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test file with spaces.log");
        
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "test content").unwrap();
        
        backup_log_file(&file_path);
        
        let backup_path = temp_dir.path().join("bak").join("test file with spaces.log");
        assert!(!file_path.exists());
        assert!(backup_path.exists());
    }
}
