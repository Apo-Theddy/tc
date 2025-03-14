use super::CommandHandler;
use clap::{Args, Subcommand};
use colored::Colorize;
use std::{fs, path::Path};

#[derive(Debug, Args)]
pub struct ListFilesArgs {
    #[arg(short = 'e', long = "extension")]
    pub extensions: Option<String>,
}

#[derive(Debug, Subcommand)]
pub enum FileCommands {
    D { filename: String }, // Delete file
    C { filename: String }, // Create file
    L(ListFilesArgs),       // List files in current directory
    F { filename: String }, // Find file in directory
}

impl FileCommands {
    fn list_files(args: &ListFilesArgs) {
        let entries: Vec<_> = fs::read_dir(".")
            .expect("Failed to read directory")
            .collect();

        println!("Total number of files in the directory: {}", entries.len());

        for entry in entries {
            let entry = entry.expect("Failed to read entry");
            let filename = entry.file_name();
            let filename_str = filename.to_string_lossy();

            if let Some(ref ext) = args.extensions {
                if !filename_str.ends_with(ext) {
                    continue;
                }
            }
            println!("📄 {}", filename_str.blue());
        }
    }

    fn create_file(filename: &str) {
        let path: &Path = Path::new(filename);

        if path.exists() {
            println!("The file already exists");
            return;
        }
        fs::File::create(path).expect("Failed to create file");
        println!("File created sucessfully");
    }

    fn delete_file(filename: &str) {
        let path: &Path = Path::new(filename);

        if !path.exists() {
            println!("The file dont does not exists");
            return;
        }

        fs::remove_file(path).expect("The file could not be deleted");
        println!("File deleted successfully");
    }

    fn find_file(filename: &str) {
        let path: &Path = Path::new(filename);

        if !path.exists() {
            println!("The file you were looking for was not found");
            return;
        }

        println!("File found in: {}", path.display());
        return;
    }
}

impl CommandHandler for FileCommands {
    async fn execute(&self) {
        match self {
            FileCommands::L(args) => Self::list_files(args),
            FileCommands::C { filename } => Self::create_file(filename),
            FileCommands::D { filename } => Self::delete_file(filename),
            FileCommands::F { filename } => Self::find_file(filename),
        }
    }
}
