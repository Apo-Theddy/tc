use clap::Subcommand;

use super::CommandHandler;

#[derive(Debug, Subcommand)]
pub enum DirCommands {
    D,                     // Delete directory
    C { dirname: String }, // Create directory
    L,                     // List directories
    F { dirname: String }, // Find directory
}

impl CommandHandler for DirCommands {
    fn execute(&self) {
        match self {
            DirCommands::L => {}
            DirCommands::C { dirname } => {}
            DirCommands::D => {}
            DirCommands::F { dirname } => {}
        }
    }
}
