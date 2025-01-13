use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

use colored::Colorize;

pub fn create(kind: &str, name: &str, within: Option<String>) -> io::Result<()> {
    match kind {
        "folder" => {
            if !Path::new(name).exists() {
                fs::create_dir(name)?;
                println!("Folder '{}' created successfully.", name);
            } else {
                println!("Folder '{}' already exists.", name);
            }
        }
        "file" => {
            let path = Path::new(name);
            // Not require extension. Eg: "Podfile,..."
            // if path.extension().is_none() {
            //     eprintln!(
            //         "Invalid file name '{}'. A file extension is required.",
            //         name
            //     );
            //     return Err(io::Error::new(
            //         io::ErrorKind::InvalidInput,
            //         "File extension required",
            //     ));
            // }

            if !path.exists() {
                let mut created_file = File::create(path)?;
                println!("File '{}' created successfully.", name.blue());
                if let Some(within) = within {
                    created_file.write_all(within.as_bytes())?;
                }
            } else {
                println!("File '{}' already exists.", name);
            }
        }
        _ => {
            eprintln!("Invalid kind '{}'. Use 'folder' or 'file'.", kind);
        }
    }
    Ok(())
}
