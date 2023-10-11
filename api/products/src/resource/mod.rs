// mod resource;
pub mod create;

// external
use axum::{body::Body, http::Response, Json};
use serde_json::Value;

#[utoipa::path(
    post,
    path = "/create",
    request_body = CreateInput,
    responses(
        (
            status = http::StatusCode::CREATED, 
            description = "Successfully created product",  
            body = CreateResponse
        )
    ),
)]
pub async fn create(input: Json<Value>) -> Response<Body> {
    create::create(input).await
}