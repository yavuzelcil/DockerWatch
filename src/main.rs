mod cli;
mod docker;
use clap::Parser;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = cli::Cli::parse();
    let docker_client = docker::DockerClient::new();

    match &cli.command {
        cli::Command::List { list_command } => match list_command {
            cli::ListCommands::Containers { all } => {
                println!("Listing Docker containers...");
                match docker_client.list_containers(*all).await {
                    Ok(containers) => {
                        for container in containers {
                            println!(
                                "-> ID: {}, Image: {}, Status: {}",
                                container.id.unwrap_or_default(),
                                container.image.unwrap_or_default(),
                                container.status.unwrap_or_default()
                            );
                        }
                    }
                    Err(e) => {
                        eprintln!("Error listing containers: {}", e);
                    }
                }
            }
            cli::ListCommands::Images => {
                println!("Listing all Docker images...");
                match docker_client.list_images().await {
                    Ok(images) => {
                        for image in images {
                            println!(
                                "-> ID: {}, RepoTags: {:?}",
                                image.id,
                                image.repo_tags
                            );
                        }
                    }
                    Err(e) => {
                        eprintln!("Error listing images: {}", e);
                    }
                }
            }
        },
    }
    
    Ok(())
}
