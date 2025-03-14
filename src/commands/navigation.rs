use super::cache::Cache;
use super::CommandHandler;
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum NavigationCommands {
    T { dirpath: String },
}

impl NavigationCommands {
    fn to(dirpath: &str) {
        Cache::save_to_cache(dirpath);
    }
}

impl CommandHandler for NavigationCommands {
    async fn execute(&self) {
        match self {
            NavigationCommands::T { dirpath } => Self::to(dirpath),
        }
    }
}
