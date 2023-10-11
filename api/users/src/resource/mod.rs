// mod resource;
pub mod login;

// external
use axum::{body::Body, http::Response, Json};
use serde_json::Value;

#[utoipa::path(
    post,
    path = "/login",
    request_body = LoginInput,
    responses(
        (
            status = http::StatusCode::OK, 
            description = "Successfully signed in",
            body = LoginResponse
        )
    ),
)]
pub async fn login(input: Json<Value>) -> Response<Body> {
    login::login(input).await
}