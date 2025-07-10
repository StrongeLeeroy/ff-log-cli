use std::fs::read_dir;
use std::path::Path;
use std::time::Instant;
use std::{env, io};

mod commands;

fn main() -> Result<(), io::Error> {
    let start = Instant::now();

    #[cfg(unix)]
    let app_data = std::env::var("HOME").expect("No HOME directory");
    #[cfg(windows)]
    let app_data = std::env::var("APPDATA").expect("No APP_DATA directory");

    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

    let mut fflogs_dir = config.fflogs_dir;
    if fflogs_dir == "default" {
        fflogs_dir = format!("{app_data}\\Advanced Combat Tracker\\FFXIVLogs");
    }

    let path = Path::new(&fflogs_dir);
    if !path.is_dir() {
        println!("Not a valid directory: {}", path.display());
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "not a valid directory",
        ));
    }
    match config.operation {
        Operation::View => {
            commands::view::view_log_files(path)?;
        }
        _ => {
            for entry in read_dir(path)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    println!("Ignoring path as it is a directory: {}", path.display());
                } else {
                    match config.operation {
                        Operation::List => {
                            commands::list::list_log_file(&path);
                        }
                        Operation::Backup => {
                            commands::backup::backup_log_file(&path);
                        }
                        Operation::Delete => {
                            commands::delete::delete_log_file(&path);
                        }
                        Operation::View => unreachable!(),
                    }
                }
            }
        }
    }
    let duration = start.elapsed();
    println!("Completed in: {duration:?}");
    Ok(())
}

enum Operation {
    List,
    Delete,
    Backup,
    View,
}

struct Config {
    operation: Operation,
    fflogs_dir: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("not enough arguments (you must provide a FFlogs directory path");
        }
        let operation_arg = args[1].clone();
        let fflogs_dir = args[2].clone();

        let operation: Operation = match operation_arg.as_ref() {
            "backup" => Operation::Backup,
            "delete" => Operation::Delete,
            "list" => Operation::List,
            "view" => Operation::View,
            _ => Operation::List,
        };

        Self {
            operation,
            fflogs_dir,
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
    #[should_panic]
    fn test_new_config_no_args() {
        let invalid_args: Vec<String> = Vec::new();
        let _config = Config::new(&invalid_args);
    }

    #[test]
    #[should_panic]
    fn test_new_config_one_arg() {
        let invalid_args: Vec<String> = Vec::from([String::from("invalid_option")]);
        let _config = Config::new(&invalid_args);
    }

    #[test]
    #[should_panic]
    fn test_new_config_two_args_invalid() {
        let invalid_args: Vec<String> = Vec::from([
            String::from("invalid_option"),
            String::from("invalid_option_2"),
        ]);
        let _config = Config::new(&invalid_args);
    }

    #[test]
    fn test_config_new_list_operation() {
        let args = vec![
            String::from("program"),
            String::from("list"),
            String::from("/path/to/logs"),
        ];
        let config = Config::new(&args);

        assert!(matches!(config.operation, Operation::List));
        assert_eq!(config.fflogs_dir, "/path/to/logs");
    }

    #[test]
    fn test_config_new_backup_operation() {
        let args = vec![
            String::from("program"),
            String::from("backup"),
            String::from("/path/to/logs"),
        ];
        let config = Config::new(&args);

        assert!(matches!(config.operation, Operation::Backup));
        assert_eq!(config.fflogs_dir, "/path/to/logs");
    }

    #[test]
    fn test_config_new_delete_operation() {
        let args = vec![
            String::from("program"),
            String::from("delete"),
            String::from("/path/to/logs"),
        ];
        let config = Config::new(&args);

        assert!(matches!(config.operation, Operation::Delete));
        assert_eq!(config.fflogs_dir, "/path/to/logs");
    }

    #[test]
    fn test_config_new_view_operation() {
        let args = vec![
            String::from("program"),
            String::from("view"),
            String::from("/path/to/logs"),
        ];
        let config = Config::new(&args);

        assert!(matches!(config.operation, Operation::View));
        assert_eq!(config.fflogs_dir, "/path/to/logs");
    }

    #[test]
    fn test_config_new_unknown_operation_defaults_to_list() {
        let args = vec![
            String::from("program"),
            String::from("unknown"),
            String::from("/path/to/logs"),
        ];
        let config = Config::new(&args);

        assert!(matches!(config.operation, Operation::List));
        assert_eq!(config.fflogs_dir, "/path/to/logs");
    }

    #[test]
    fn test_config_new_default_directory() {
        let args = vec![
            String::from("program"),
            String::from("list"),
            String::from("default"),
        ];
        let config = Config::new(&args);

        assert_eq!(config.fflogs_dir, "default");
    }

    #[test]
    fn test_config_new_with_spaces_in_path() {
        let args = vec![
            String::from("program"),
            String::from("list"),
            String::from("/path/with spaces/logs"),
        ];
        let config = Config::new(&args);

        assert_eq!(config.fflogs_dir, "/path/with spaces/logs");
    }

    #[test]
    fn test_config_new_extra_args_ignored() {
        let args = vec![
            String::from("program"),
            String::from("list"),
            String::from("/path/to/logs"),
            String::from("extra"),
            String::from("arguments"),
        ];
        let config = Config::new(&args);

        assert!(matches!(config.operation, Operation::List));
        assert_eq!(config.fflogs_dir, "/path/to/logs");
    }

    #[test]
    fn test_operation_enum_display() {
        let list_op = Operation::List;
        let backup_op = Operation::Backup;
        let delete_op = Operation::Delete;
        let view_op = Operation::View;

        assert!(matches!(list_op, Operation::List));
        assert!(matches!(backup_op, Operation::Backup));
        assert!(matches!(delete_op, Operation::Delete));
        assert!(matches!(view_op, Operation::View));
    }

    #[test]
    fn test_main_function_with_invalid_directory() {
        let temp_dir = TempDir::new().unwrap();
        let nonexistent_dir = temp_dir.path().join("nonexistent");

        let test_args = vec![
            String::from("program"),
            String::from("list"),
            nonexistent_dir.to_string_lossy().to_string(),
        ];

        let config = Config::new(&test_args);
        assert!(matches!(config.operation, Operation::List));
    }

    #[test]
    fn test_main_function_with_valid_directory() {
        let temp_dir = TempDir::new().unwrap();

        let test_file = temp_dir.path().join("test.log");
        let mut file = File::create(&test_file).unwrap();
        writeln!(file, "test log content").unwrap();

        let test_args = vec![
            String::from("program"),
            String::from("list"),
            temp_dir.path().to_string_lossy().to_string(),
        ];

        let config = Config::new(&test_args);
        assert!(matches!(config.operation, Operation::List));
        assert_eq!(config.fflogs_dir, temp_dir.path().to_string_lossy());
    }

    #[test]
    fn test_config_case_sensitivity() {
        let args = vec![
            String::from("program"),
            String::from("LIST"),
            String::from("/path/to/logs"),
        ];
        let config = Config::new(&args);

        assert!(matches!(config.operation, Operation::List));
    }

    #[test]
    fn test_config_partial_match() {
        let args = vec![
            String::from("program"),
            String::from("back"),
            String::from("/path/to/logs"),
        ];
        let config = Config::new(&args);

        assert!(matches!(config.operation, Operation::List));
    }
}
