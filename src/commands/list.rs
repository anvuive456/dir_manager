use std::fs::{self};
use std::io::{self};

use colored::Colorize;

pub fn list_files(show_size: bool) -> io::Result<()> {
    let current_dir = ".";
    let entries = fs::read_dir(current_dir)?;

    for entry in entries {
        let entry = entry?;
        let file_name = entry.file_name();

        let file_name_lossy = file_name.to_string_lossy();

        let metadata = entry.metadata()?;
        let size = metadata.len();
        let is_hidden = file_name_lossy.starts_with('.');

        let display_name = if is_hidden {
            file_name_lossy.italic().dimmed()
        } else {
            file_name_lossy.normal()
        };

        println!(
            "{} {}",
            display_name,
            if show_size {
                format!("({} bytes)", size).blue()
            } else {
                format!("").normal()
            }
        );
    }

    Ok(())
}
