use std::path::PathBuf;
use std::str::FromStr;
use axum::http::StatusCode;
use docker_api::Docker;
use docker_api::opts::ImageBuildOpts;
use crate::service::command::build_image_command::BuildImageCommand;
use futures::StreamExt;


pub struct DockerImageService {
    pub docker: Docker,
}

#[derive(Parser)]
pub struct Opts {
    #[command(subcommand)]
    subcmd: Cmd,
}

#[derive(Parser)]
enum Cmd {
    /// Build an image.
    Build {
        /// A path to the directory containing Dockerfile for the image.
        path: PathBuf,
        #[arg(default_value = "latest")]
        tag: String,
        language: String,
    },
    /// Delete an image.
    Delete {
        image: String,
        #[arg(short, long)]
        force: bool,
        #[arg(long)]
        noprune: bool,
    },
    /// Pull an image from image registry.
    Pull {
        /// The name or id of the image to pull.
        image: String,
        /// Username in case authentication is required.
        username: Option<String>,
        /// Password in case authentication is required.
        password: Option<String>,
    },
    Tag {
        /// Repository containing the image to tag.
        repo: String,
        /// The name or id of the image to tag.
        image: String,
        tag: String,
    },
}

impl DockerImageService {
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
        let options = ImageBuildOpts::builder(image.path + &*image.language + ".Dockerfile").tag(image.tag).build();
        let images = self.docker.images();
        let mut stream = images.build(&options);
        while let Some(build_result) = stream.next().await {
            match build_result {
                Ok(output) => {
                    println!("{output:?}");
                    return Ok(());
                }
                Err(e) => {
                    eprintln!("Error: {e}");
                    return Err(StatusCode::INTERNAL_SERVER_ERROR);
                }
            }
        }
        return Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}