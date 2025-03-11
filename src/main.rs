mod commands;

use clap::{Parser, Subcommand};
use colored::*;
use std::fs;
use std::process::Command;

use commands::{DirCommands, FileCommands};

#[derive(Debug, Parser)]
enum Commands {
    #[command(subcommand)]
    F(FileCommands),

    #[command(subcommand)]
    D(DirCommands),
}

fn main() {
    let args = Commands::parse();

    match args {
        Commands::F(file_cmd) => match file_cmd {
            FileCommands::L => {
                #[cfg(target_os = "windows")]
                Command::new("dir")
                    .spawn()
                    .expect("Failed to execute command");

                #[cfg(target_os = "linux")]
                Command::new("ls")
                    .spawn()
                    .expect("Failed to execute command");

                #[cfg(target_os = "macos")]
                let output = Command::new("ls")
                    .output()
                    .expect("Failed to execute command");

                let output_str = String::from_utf8_lossy(&output.stdout);
                for line in output_str.lines() {
                    let path = line.trim();
                    if let Ok(metadata) = fs::metadata(path) {
                        if metadata.is_file() {
                            println!("📄 {}", line.blue());
                        } else if metadata.is_dir() {
                            println!("📂 {}", line.green());
                        }
                    }
                }
            }
            FileCommands::C { filename } => {}
            FileCommands::D { filename } => {}
            FileCommands::F { filename } => {}
        },
        Commands::D(dir_cmd) => match dir_cmd {
            DirCommands::L => {}
            DirCommands::C { dirname } => {}
            DirCommands::D => {}
            DirCommands::F { dirname } => {}
        },
    }
}
