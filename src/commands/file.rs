use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum FileCommands {
    D { filename: String }, // Delete file
    C { filename: String }, // Create file
    L,                      // List files in current directory
    F { filename: String }, // Find file in directory
}
