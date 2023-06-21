use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct CreateImageRequest {
    pub path: String,
    pub tag: String,
    pub language: String,
}