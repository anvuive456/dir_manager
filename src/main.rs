use clap::{Parser, Subcommand};
mod commands;
mod query_language;

use commands::{create::create, list::list_files, search::search_pattern_recursive};
use query_language::ql::execute_query;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    List {
        #[arg(short, long, default_value_t = false)]
        size: bool,
    },
    Create(CreateArgs),

    /// Search in files
    Search {
        pattern: String,
        /// The folder to search in (defaults to current directory).
        #[arg(short = 'F', long, default_value = ".")]
        folder: String,

        /// File extension to filter by (e.g., ".txt").
        #[arg(short, long)]
        extension: Option<String>,

        /// Show lines around the match (-C for grep equivalent).
        #[arg(short = 'C', long, default_value_t = 1)]
        context: usize,

        #[arg(short, long, default_value_t = false)]
        fuzzy: bool,

        #[arg(short, long = "fuzzy-threshold")]
        threshold: Option<i64>,
    },

    Query {
        ql: String,
    },
}

#[derive(Parser, Debug, Clone)]
#[command(args_conflicts_with_subcommands = true)]
struct CreateArgs {
    #[command(subcommand)]
    cmd: CreateCommands,
}

#[derive(Subcommand, Debug, Clone)]
enum CreateCommands {
    /// Create a new folder.
    Folder {
        /// The name of the folder to create.
        name: String,
    },
    /// Create a new file.
    File {
        /// The name of the file to create (e.g., "my_file.txt").
        name: String,

        /// The content within file
        #[arg(short, long)]
        within: Option<String>,
    },
}

fn main() {
    let args = Args::parse();

    match args.cmd {
        Commands::List { size } => {
            if let Err(e) = list_files(size) {
                eprintln!("Error listing files: {}", e);
            }
        }
        Commands::Create(arg) => match arg.cmd {
            CreateCommands::File { name, within } => {
                if let Err(e) = create("file", name.as_str(), within) {
                    eprintln!("Error creating file: {}", e,);
                }
            }
            CreateCommands::Folder { name } => {
                if let Err(e) = create("folder", name.as_str(), None) {
                    eprintln!("Error creating folder: {}", e);
                }
            }
        },
        Commands::Search {
            pattern,
            folder,
            extension,
            context,
            fuzzy,
            threshold,
        } => {
            if let Err(er) = search_pattern_recursive(
                &pattern,
                &folder,
                extension.as_ref(),
                context,
                fuzzy,
                threshold,
            ) {
                eprintln!("Error searching: {}", er);
            }
        }
        Commands::Query { ql } => {
            if let Err(er) = execute_query(ql.as_str()) {
                eprintln!("Error searching: {}", er);
            }
        }
    }
}
