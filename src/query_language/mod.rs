use regex::Regex;
use std::fs;
use std::io;
use std::path::Path;

pub mod ql {
    use crate::commands::{list::list_files, search::search_pattern_recursive};

    use super::*;

    pub fn execute_query(query: &str) -> io::Result<()> {
        let commands: Vec<&str> = query
            .split(';')
            .map(|cmd| cmd.trim())
            .filter(|cmd| !cmd.is_empty())
            .collect();

        for command in commands {
            match parse_command(command) {
                Ok(parsed_command) => execute_command(parsed_command)?,
                Err(err) => eprintln!("Error parsing command '{}': {}", command, err),
            }
        }

        Ok(())
    }

    #[derive(Debug)]
    enum Command {
        List {
            show_size: bool,
        },
        Search {
            pattern: String,
            path: String,
            condition: Option<String>,
        },
        Create {
            kind: String,
            name: String,
            path: String,
        },
        Delete {
            pattern: String,
            path: String,
            force: bool,
        },
        Move {
            pattern: String,
            from: String,
            to: String,
        },
        Copy {
            pattern: String,
            from: String,
            to: String,
        },
    }

    fn parse_command(command: &str) -> Result<Command, String> {
        let search_re =
            Regex::new(r"^SEARCH name: \'(.+?)\' IN \'(.+?)\'(?: WHERE (.+))?$").unwrap();
        let create_re = Regex::new(r"^CREATE (file|folder): \'(.+?)\' IN: \'(.+?)\'$").unwrap();
        let delete_re = Regex::new(r"^DELETE name: \'(.+?)\' IN \'(.+?)\'( FORCE)?$").unwrap();
        let move_re = Regex::new(r"^MOVE name: \'(.+?)\' FROM \'(.+?)\' TO \'(.+?)\'$").unwrap();
        let copy_re = Regex::new(r"^COPY name: \'(.+?)\' FROM \'(.+?)\' TO \'(.+?)\'$").unwrap();

        if let Some(caps) = search_re.captures(command) {
            Ok(Command::Search {
                pattern: caps[1].to_string(),
                path: caps[2].to_string(),
                condition: caps.get(3).map(|m| m.as_str().to_string()),
            })
        } else if let Some(caps) = create_re.captures(command) {
            Ok(Command::Create {
                kind: caps[1].to_string(),
                name: caps[2].to_string(),
                path: caps[3].to_string(),
            })
        } else if let Some(caps) = delete_re.captures(command) {
            Ok(Command::Delete {
                pattern: caps[1].to_string(),
                path: caps[2].to_string(),
                force: caps.get(3).is_some(),
            })
        } else if let Some(caps) = move_re.captures(command) {
            Ok(Command::Move {
                pattern: caps[1].to_string(),
                from: caps[2].to_string(),
                to: caps[3].to_string(),
            })
        } else if let Some(caps) = copy_re.captures(command) {
            Ok(Command::Copy {
                pattern: caps[1].to_string(),
                from: caps[2].to_string(),
                to: caps[3].to_string(),
            })
        } else {
            Err("Invalid command syntax".to_string())
        }
    }

    fn execute_command(command: Command) -> io::Result<()> {
        match command {
            Command::Search {
                pattern,
                path,
                condition,
            } => {
                println!(
                    "Searching for '{}' in '{}' with condition '{:?}'",
                    pattern, path, condition
                );

                if let Err(e) =
                    search_pattern_recursive(pattern.as_str(), path.as_str(), None, 1, false, None)
                {
                    eprintln!("{}", e)
                }
                // Call the existing search function here.
            }
            Command::Create { kind, name, path } => {
                let full_path = Path::new(&path).join(&name);
                if kind == "file" {
                    fs::File::create(&full_path)?;
                    println!("File '{}' created successfully.", full_path.display());
                } else if kind == "folder" {
                    fs::create_dir_all(&full_path)?;
                    println!("Folder '{}' created successfully.", full_path.display());
                }
            }
            Command::Delete {
                pattern,
                path,
                force,
            } => {
                println!(
                    "Deleting '{}' in '{}' with force '{}'",
                    pattern, path, force
                );
                // Implement deletion logic here.
            }
            Command::Move { pattern, from, to } => {
                println!("Moving '{}' from '{}' to '{}'", pattern, from, to);
                // Implement move logic here.
            }
            Command::Copy { pattern, from, to } => {
                println!("Copying '{}' from '{}' to '{}'", pattern, from, to);
                // Implement copy logic here.
            }
            Command::List { show_size } => {
                if let Err(err) = list_files(show_size) {
                    eprintln!("{}", err);
                }
            }
        }

        Ok(())
    }
}
