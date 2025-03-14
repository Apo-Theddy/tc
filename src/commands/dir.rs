use clap::Subcommand;
use colored::Colorize;
use std::{fs, path::Path};

use super::CommandHandler;

#[derive(Debug, Subcommand)]
pub enum DirCommands {
    D { dirname: String }, // Delete directory
    C { dirname: String }, // Create directory
    L,                     // List directories
    F { dirname: String }, // Find directory
}

impl DirCommands {
    fn list_dirs() {
        let entries = fs::read_dir(".").expect("Failed to read directory");

        for entry in entries {
            let entry = entry.expect("Failed to read entry");
            let metadata = entry.metadata().expect("Failed to get metadata");

            if metadata.is_dir() {
                println!("📁 {}", entry.file_name().to_string_lossy().green());
            }
        }
    }

    fn create_dir(dirname: &str) {
        let path: &Path = Path::new(dirname);
        if !path.exists() {
            fs::create_dir(path).expect("Failed to create directory");
        }
        println!("Directory created successfully");
    }

    fn delete_dir(dirname: &str) {
        let path: &Path = Path::new(dirname);
        if path.exists() {
            fs::remove_dir(path).expect("Failed to delete directory");
        }
        println!("Directory deleted successfully");
    }

    fn find_dir(dirname: &str) {
        let path: &Path = Path::new(dirname);
        if path.exists() {
            println!("Directory found successfully at {}", path.display());
        } else {
            println!("Directory not found");
        }
    }
}

impl CommandHandler for DirCommands {
    async fn execute(&self) {
        match self {
            DirCommands::L => Self::list_dirs(),
            DirCommands::C { dirname } => Self::create_dir(dirname),
            DirCommands::D { dirname } => Self::delete_dir(dirname),
            DirCommands::F { dirname } => Self::find_dir(dirname),
        }
    }
}
