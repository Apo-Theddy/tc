use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum DirCommands {
    D,                     // Delete directory
    C { dirname: String }, // Create directory
    L,                     // List directories
    F { dirname: String }, // Find directory
}
