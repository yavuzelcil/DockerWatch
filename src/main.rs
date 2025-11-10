mod cli;
mod docker;
use clap::Parser;
use cli::{Cli, Command, ListCommands};
use docker::DockerClient;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = cli::Cli::parse();
    let docker_client = docker::DockerClient::new();

    match &cli.command {
        cli::Command::List { list_command } => match list_command {
            cli::ListCommands::Containers { all } => {
                let containers = docker_client.list_containers(*all).await?;
                for container in containers {
                    println!("-> {:?}", container);
                }
            }
            cli::ListCommands::Images => {
                println!("Listing all Docker images...");
                // TODO: Implement list_images
            }
        },
    }
    
    Ok(())
}
