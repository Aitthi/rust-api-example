use axum::{
    body::Body,
    http::{Response, StatusCode},
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, Debug, ToSchema)]
pub struct LoginInput {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, ToSchema)]
pub struct LoginResponse {
    pub status: u16,
    pub message: String,
    pub data: Value,
}

pub async fn login(mut input: Json<Value>) -> Response<Body> {
    let input: LoginInput = match serde_json::from_value(input.take()) {
        Ok(r) => r,
        Err(err) => {
            return utils::response::json(
                serde_json::json!(LoginResponse {
                    status: StatusCode::BAD_REQUEST.into(),
                    message: format!("Failed to parse input: {}", err),
                    data: serde_json::json!(null)
                }),
                StatusCode::BAD_REQUEST,
            )
        }
    };

    utils::response::json(
        serde_json::json!(LoginResponse {
            status: StatusCode::CREATED.into(),
            message: "Successfully signed in".to_string(),
            data: serde_json::json!(input)
        }),
        StatusCode::CREATED,
    )
}
