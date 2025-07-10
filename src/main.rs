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
    for entry in read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            println!("Ignoring path as it is a directoru: {}", path.display());
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
            }
        }
    }
    let duration = start.elapsed();
    println!("Completed in: {:?}", duration);
    Ok(())
}

enum Operation {
    List,
    Delete,
    Backup,
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
}
