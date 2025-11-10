use clap::{Parser, Subcommand};

/// CLI structure for the Docker monitoring tool
#[derive(Parser)]
#[command(name = "dockerwatch")]
#[command(about = "A CLI tool to monitor and manage Docker containers", long_about = None)]
pub struct Cli {
    /// Main command for the CLI
    #[command(subcommand)]
    pub command: Command,
}

/// Enum for top-level commands
#[derive(Subcommand)]
pub enum Command {
    /// List resources
    List {
        /// Subcommands for listing resources
        #[command(subcommand)]
        list_command: ListCommands,
    },
}

#[derive(Subcommand)]
pub enum ListCommands {
    /// List all running Docker containers
    Containers {
        #[arg(short, long, default_value_t = false)]
        all: bool, // List all containers including stopped ones
    },
    /// List all Docker images
    Images,
}