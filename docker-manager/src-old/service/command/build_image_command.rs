use crate::entrypoint::docker_image::route::request::create_image_request::CreateImageRequest;

#[derive(Debug)]
pub struct BuildImageCommand {
    pub path: String,
    pub tag: String,
    pub language: String,
}

impl BuildImageCommand {

    pub fn new(request: CreateImageRequest) -> Self {
        BuildImageCommand {
            path: request.path,
            tag: request.tag,
            language: request.language,
        }
    }
}