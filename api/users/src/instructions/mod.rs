// mod instructions;
pub mod login;

// external
use axum::{body::Body, http::Response, Json};
use serde_json::Value;

// export Schema for API documentation
pub use login::{
    LoginInput,
    LoginResponse
};

#[utoipa::path(
    post,
    path = "/login",
    request_body = LoginInput,
    responses(
        (
            status = http::StatusCode::CREATED, 
            description = "Successfully signed in",
            body = LoginResponse
        )
    ),
)]
pub async fn login(input: Json<Value>) -> Response<Body> {
    login::login(input).await
}