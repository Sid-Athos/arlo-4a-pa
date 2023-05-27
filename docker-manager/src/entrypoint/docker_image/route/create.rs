use axum::http::StatusCode;
use axum::Json;
use crate::entrypoint::docker_image::route::request::create_image_request::CreateImageRequest;
use crate::entrypoint::docker_image::route::response::create_image_response::CreateImageResponse;
use crate::service::command::build_image_command::BuildImageCommand;
use crate::service::docker_image_service::DockerImageService;

#[utoipa::path(
post,
path = "/",
request_body = CreateUserRequest,
responses(
(status = 200, description = "Image created",),
(status = 500, description = "Something went wrong",),
),
security(
("api-key" = [])
),
tag = "docker_image"
)]
pub async fn image_create(Json(image): Json<CreateImageRequest>) -> Result<(), StatusCode> {
    tracing::info!("Received create image request {:?}", image);
    let image_service = DockerImageService::new()?;

    let image = image_service.build_image(BuildImageCommand::new(image)).await;

    image
}