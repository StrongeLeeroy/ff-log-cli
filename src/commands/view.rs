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