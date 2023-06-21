use axum::http::StatusCode;
use docker_api::Docker;
use docker_api::opts::ImageBuildOpts;
use crate::service::command::build_image_command::BuildImageCommand;
use futures::StreamExt;
use crate::entrypoint::docker_image::route::create::image_create;


pub struct DockerImageService {
    pub docker: Docker,
}


impl DockerImageService {
    #[cfg(unix)]
    pub fn new_docker() -> docker_api::Result<Docker> {
        Ok(Docker::unix("/var/run/docker.sock"))
    }

    #[cfg(not(unix))]
    pub fn new_docker() -> docker_api::Result<Docker> {
        Docker::new("tcp://127.0.0.1:8080")
    }

    pub fn new() -> Result<Self, StatusCode> {
        Ok(DockerImageService {
            docker: match Self::new_docker() {
                Ok(docker_api) => {
                    docker_api
                }
                Err(_) => { return Err(StatusCode::INTERNAL_SERVER_ERROR); }
            },
        })
    }

    pub async fn build_image(&self, image: BuildImageCommand) -> Result<(), StatusCode> {
        let options = ImageBuildOpts::builder(format!("{}{}", image.path, &*image.language)).tag(image.tag).build();
        println!("{:?}", options);
        let images = self.docker.images();
        let mut stream = images.build(&options);
        while let Some(build_result) = stream.next().await {
            match build_result {
                Ok(output) => {
                    println!("{output:?}");
                }
                Err(e) => {
                    eprintln!("Error: {e}");
                    return Err(e);
                }
            }
        }
        Ok(())
    }
}