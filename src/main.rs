mod commands;

use clap::Parser;
use commands::{
    ai::AICommands, navigation::NavigationCommands, CommandHandler, DirCommands, FileCommands,
};
use dotenvy::{self, dotenv};

#[derive(Debug, Parser)]
enum Commands {
    #[command(subcommand)]
    F(FileCommands),

    #[command(subcommand)]
    D(DirCommands),

    #[command(subcommand)]
    AI(AICommands),

    #[command(subcommand)]
    CD(NavigationCommands),
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let args = Commands::parse();

    match args {
        Commands::D(dir_cmd) => dir_cmd.execute().await,
        Commands::F(file_cmd) => file_cmd.execute().await,
        Commands::AI(ai_cmd) => ai_cmd.execute().await,
        Commands::CD(navigation_cmd) => navigation_cmd.execute().await,
    }
}
