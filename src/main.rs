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
        }
        cli::Command::Start { container } => {
            println!("Starting container: {}", container);
            match docker_client.start_container(container).await {
                Ok(_) => {
                    println!("Container {} started successfully.", container);
                }
                Err(e) => {
                    eprintln!("Error starting container {}: {}", container, e);
                }
            }
        }
        cli::Command::Stop { container } => {
            println!("Stopping container: {}", container);
            match docker_client.stop_container(container).await {
                Ok(_) => {
                    println!("Container {} stopped successfully.", container);
                }
                Err(e) => {
                    eprintln!("Error stopping container {}: {}", container, e);
                }
            }
        }
        cli::Command::Pull { image } => {
            println!("Pulling image: {}", image);
            match docker_client.pull_image(image).await {
                Ok(_) => {
                    println!("Image {} pulled successfully.", image);
                }
                Err(e) => {
                    eprintln!("Error pulling image {}: {}", image, e);
                }
            }
        }
    }
    
    Ok(())
}
