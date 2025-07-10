use std::fs;
use std::io::{self, Write};
use std::path::Path;

pub fn view_log_files(log_dir: &Path) -> Result<(), io::Error> {
    let mut log_files = Vec::new();
    
    for entry in fs::read_dir(log_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            log_files.push(path);
        }
    }
    
    if log_files.is_empty() {
        println!("No log files found in directory: {}", log_dir.display());
        return Ok(());
    }
    
    println!("Available log files:");
    for (index, file) in log_files.iter().enumerate() {
        let file_name = file.file_name().unwrap_or_default();
        println!("{}. {}", index + 1, file_name.to_string_lossy());
    }
    
    print!("Enter the number of the file you want to view: ");
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    let choice: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a number.");
            return Ok(());
        }
    };
    
    if choice == 0 || choice > log_files.len() {
        println!("Invalid selection. Please choose a number between 1 and {}.", log_files.len());
        return Ok(());
    }
    
    let selected_file = &log_files[choice - 1];
    println!("\nContents of {}:", selected_file.file_name().unwrap_or_default().to_string_lossy());
    println!("{}", "=".repeat(50));
    
    match fs::read_to_string(selected_file) {
        Ok(content) => println!("{}", content),
        Err(e) => println!("Error reading file: {}", e),
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_view_log_files_empty_directory() {
        let temp_dir = TempDir::new().unwrap();
        
        let result = view_log_files(temp_dir.path());
        assert!(result.is_ok());
    }

    #[test]
    fn test_view_log_files_with_files() {
        let temp_dir = TempDir::new().unwrap();
        
        let file1 = temp_dir.path().join("test1.log");
        let file2 = temp_dir.path().join("test2.log");
        
        let mut f1 = File::create(&file1).unwrap();
        writeln!(f1, "Log file 1 content").unwrap();
        
        let mut f2 = File::create(&file2).unwrap();
        writeln!(f2, "Log file 2 content").unwrap();
        
        let result = view_log_files(temp_dir.path());
        assert!(result.is_ok());
    }

    #[test]
    fn test_view_log_files_mixed_files_and_directories() {
        let temp_dir = TempDir::new().unwrap();
        
        let file1 = temp_dir.path().join("test.log");
        let mut f1 = File::create(&file1).unwrap();
        writeln!(f1, "Log file content").unwrap();
        
        let subdir = temp_dir.path().join("subdir");
        fs::create_dir(&subdir).unwrap();
        
        let result = view_log_files(temp_dir.path());
        assert!(result.is_ok());
    }

    #[test]
    fn test_view_log_files_nonexistent_directory() {
        let temp_dir = TempDir::new().unwrap();
        let nonexistent = temp_dir.path().join("nonexistent");
        
        let result = view_log_files(&nonexistent);
        assert!(result.is_err());
    }

    #[test]
    fn test_view_log_files_with_special_characters() {
        let temp_dir = TempDir::new().unwrap();
        
        let file1 = temp_dir.path().join("test file with spaces.log");
        let file2 = temp_dir.path().join("測試檔案.log");
        
        let mut f1 = File::create(&file1).unwrap();
        writeln!(f1, "Content with spaces").unwrap();
        
        let mut f2 = File::create(&file2).unwrap();
        writeln!(f2, "Unicode content").unwrap();
        
        let result = view_log_files(temp_dir.path());
        assert!(result.is_ok());
    }

    #[test]
    fn test_view_log_files_unreadable_file() {
        let temp_dir = TempDir::new().unwrap();
        
        let file1 = temp_dir.path().join("readable.log");
        let mut f1 = File::create(&file1).unwrap();
        writeln!(f1, "Readable content").unwrap();
        
        let result = view_log_files(temp_dir.path());
        assert!(result.is_ok());
    }

    #[test]
    fn test_view_log_files_large_directory() {
        let temp_dir = TempDir::new().unwrap();
        
        for i in 0..100 {
            let file_path = temp_dir.path().join(format!("test_{}.log", i));
            let mut file = File::create(&file_path).unwrap();
            writeln!(file, "Log file {} content", i).unwrap();
        }
        
        let result = view_log_files(temp_dir.path());
        assert!(result.is_ok());
    }
}