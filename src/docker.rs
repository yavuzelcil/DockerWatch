use bollard::Docker;
use bollard::query_parameters::{ListContainersOptions, ListImagesOptions, StartContainerOptions, StopContainerOptions};
use bollard::errors::Error;
use bollard::models::{ContainerSummary, ImageSummary};

pub struct DockerClient {
    docker: Docker,
}

impl DockerClient {
    pub fn new() -> Self {
        let docker = Docker::connect_with_unix(
            "unix:///Users/yavuzelcil/.docker/run/docker.sock",
            120,
            bollard::API_DEFAULT_VERSION,
        ).expect("Failed to connect to Docker");
        
        Self { docker }
    }
    
    pub async fn list_containers(&self, all: bool) -> Result<Vec<ContainerSummary>, Error> {
        let options = Some(ListContainersOptions {
            all,
            ..Default::default()
        });
        let containers = self.docker.list_containers(options).await?;
        Ok(containers)
    }
    
    pub async fn list_images(&self) -> Result<Vec<ImageSummary>, Error> {
        let options = ListImagesOptions {
            all: true,
            ..Default::default()
        };
        let images = self.docker.list_images(Some(options)).await?;
        Ok(images)
    }

    pub async fn start_container(&self, container_id: &str) -> Result<(), Error> {
        // Use empty StartContainerOptions
        let options = StartContainerOptions::default();
        self.docker.start_container(container_id, Some(options)).await
    }

    pub async fn stop_container(&self, container_id: &str) -> Result<(), Error> {
        // Use default StopContainerOptions (10 seconds timeout)
        let options = StopContainerOptions::default();
        self.docker.stop_container(container_id, Some(options)).await
    }
}


/*use bollard::query_parameters::ListImagesOptions;

pub async fn list_containers(all: bool) -> Result<(), Box<dyn std::error::Error>> {
    let docker = Docker::connect_with_socket_defaults()?;
    
    if all {
        
        let options = ListImagesOptions {
            all: true,
            ..Default::default()
        };
        let images = docker.list_images(Some(options)).await?;
        
        for image in images {
            println!("-> {:?}", image);
        }
    } else {
        println!("Listing only running Docker containers...");
    }
    
    Ok(())
}

pub fn list_images() {
    // Placeholder implementation for listing Docker images
    println!("Listing all Docker images...");
    // Actual Docker API calls would go here
}


*/