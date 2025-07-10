use clap::{Parser, Subcommand};
use std::fs::read_dir;
use std::io;
use std::path::Path;
use std::time::Instant;

mod commands;

fn main() -> Result<(), io::Error> {
    let start = Instant::now();

    #[cfg(unix)]
    let app_data = std::env::var("HOME").expect("No HOME directory");
    #[cfg(windows)]
    let app_data = std::env::var("APPDATA").expect("No APP_DATA directory");

    let config = Config::parse();

    let mut fflogs_dir = match &config.command {
        Command::List { fflogs_dir } => fflogs_dir.clone(),
        Command::Delete { fflogs_dir } => fflogs_dir.clone(),
        Command::Backup { fflogs_dir } => fflogs_dir.clone(),
        Command::View { fflogs_dir } => fflogs_dir.clone(),
    };

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
    match config.command {
        Command::View { .. } => {
            commands::view::view_log_files(path)?;
        }
        _ => {
            for entry in read_dir(path)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    println!("Ignoring path as it is a directory: {}", path.display());
                } else {
                    match config.command {
                        Command::List { .. } => {
                            commands::list::list_log_file(&path);
                        }
                        Command::Backup { .. } => {
                            commands::backup::backup_log_file(&path);
                        }
                        Command::Delete { .. } => {
                            commands::delete::delete_log_file(&path);
                        }
                        Command::View { .. } => unreachable!(),
                    }
                }
            }
        }
    }
    let duration = start.elapsed();
    println!("Completed in: {duration:?}");
    Ok(())
}

#[derive(Parser)]
#[command(name = "ff-log-cli")]
#[command(about = "A CLI tool for managing Final Fantasy XIV log files")]
#[command(version)]
struct Config {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// List log files
    List {
        /// Directory containing FFLogs files (use "default" for auto-detection)
        #[arg(short, long, default_value = "default")]
        fflogs_dir: String,
    },
    /// Delete log files
    Delete {
        /// Directory containing FFLogs files (use "default" for auto-detection)
        #[arg(short, long, default_value = "default")]
        fflogs_dir: String,
    },
    /// Backup log files to bak/ directory
    Backup {
        /// Directory containing FFLogs files (use "default" for auto-detection)
        #[arg(short, long, default_value = "default")]
        fflogs_dir: String,
    },
    /// View log files and their contents
    View {
        /// Directory containing FFLogs files (use "default" for auto-detection)
        #[arg(short, long, default_value = "default")]
        fflogs_dir: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;
    use std::fs::File;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_config_list_command() {
        let args = ["program", "list"];
        let config = Config::try_parse_from(args).unwrap();

        assert!(matches!(config.command, Command::List { .. }));
        if let Command::List { fflogs_dir } = config.command {
            assert_eq!(fflogs_dir, "default");
        }
    }

    #[test]
    fn test_config_backup_command() {
        let args = ["program", "backup"];
        let config = Config::try_parse_from(args).unwrap();

        assert!(matches!(config.command, Command::Backup { .. }));
        if let Command::Backup { fflogs_dir } = config.command {
            assert_eq!(fflogs_dir, "default");
        }
    }

    #[test]
    fn test_config_delete_command() {
        let args = ["program", "delete"];
        let config = Config::try_parse_from(args).unwrap();

        assert!(matches!(config.command, Command::Delete { .. }));
        if let Command::Delete { fflogs_dir } = config.command {
            assert_eq!(fflogs_dir, "default");
        }
    }

    #[test]
    fn test_config_view_command() {
        let args = ["program", "view"];
        let config = Config::try_parse_from(args).unwrap();

        assert!(matches!(config.command, Command::View { .. }));
        if let Command::View { fflogs_dir } = config.command {
            assert_eq!(fflogs_dir, "default");
        }
    }

    #[test]
    fn test_config_with_custom_directory() {
        let args = ["program", "list", "--fflogs-dir", "/path/to/logs"];
        let config = Config::try_parse_from(args).unwrap();

        assert!(matches!(config.command, Command::List { .. }));
        if let Command::List { fflogs_dir } = config.command {
            assert_eq!(fflogs_dir, "/path/to/logs");
        }
    }

    #[test]
    fn test_config_with_short_flag() {
        let args = ["program", "list", "-f", "/path/to/logs"];
        let config = Config::try_parse_from(args).unwrap();

        assert!(matches!(config.command, Command::List { .. }));
        if let Command::List { fflogs_dir } = config.command {
            assert_eq!(fflogs_dir, "/path/to/logs");
        }
    }

    #[test]
    fn test_config_with_spaces_in_path() {
        let args = ["program", "list", "--fflogs-dir", "/path/with spaces/logs"];
        let config = Config::try_parse_from(args).unwrap();

        if let Command::List { fflogs_dir } = config.command {
            assert_eq!(fflogs_dir, "/path/with spaces/logs");
        }
    }

    #[test]
    fn test_config_default_directory() {
        let args = ["program", "list"];
        let config = Config::try_parse_from(args).unwrap();

        if let Command::List { fflogs_dir } = config.command {
            assert_eq!(fflogs_dir, "default");
        }
    }

    #[test]
    fn test_command_enum_variants() {
        let list_cmd = Command::List {
            fflogs_dir: "default".to_string(),
        };
        let backup_cmd = Command::Backup {
            fflogs_dir: "default".to_string(),
        };
        let delete_cmd = Command::Delete {
            fflogs_dir: "default".to_string(),
        };
        let view_cmd = Command::View {
            fflogs_dir: "default".to_string(),
        };

        assert!(matches!(list_cmd, Command::List { .. }));
        assert!(matches!(backup_cmd, Command::Backup { .. }));
        assert!(matches!(delete_cmd, Command::Delete { .. }));
        assert!(matches!(view_cmd, Command::View { .. }));
    }

    #[test]
    fn test_config_invalid_command_fails() {
        let args = ["program", "invalid"];
        let result = Config::try_parse_from(args);

        assert!(result.is_err());
    }

    #[test]
    fn test_config_no_command_fails() {
        let args = ["program"];
        let result = Config::try_parse_from(args);

        assert!(result.is_err());
    }

    #[test]
    fn test_main_function_with_valid_directory() {
        let temp_dir = TempDir::new().unwrap();

        let test_file = temp_dir.path().join("test.log");
        let mut file = File::create(&test_file).unwrap();
        writeln!(file, "test log content").unwrap();

        let args = [
            "program",
            "list",
            "--fflogs-dir",
            &temp_dir.path().to_string_lossy(),
        ];
        let config = Config::try_parse_from(args).unwrap();

        assert!(matches!(config.command, Command::List { .. }));
        if let Command::List { fflogs_dir } = config.command {
            assert_eq!(fflogs_dir, temp_dir.path().to_string_lossy());
        }
    }
}
