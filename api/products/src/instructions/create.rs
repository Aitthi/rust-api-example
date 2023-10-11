use axum::{
    body::Body,
    http::{Response, StatusCode},
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, Debug, ToSchema)]
pub struct CreateInput {
    pub name: String,
    pub description: String,
    pub price: f64,
    pub quantity: i32,
}

#[derive(Serialize, ToSchema)]
pub struct CreateResponse {
    pub status: u16,
    pub message: String,
    pub data: Value,
}

pub async fn create(mut input: Json<Value>) -> Response<Body> {
    let input: CreateInput = match serde_json::from_value(input.take()) {
        Ok(r) => r,
        Err(err) => {
            return utils::response::json(
                serde_json::json!(CreateResponse {
                    status: StatusCode::BAD_REQUEST.into(),
                    message: format!("Failed to parse input: {}", err),
                    data: serde_json::json!(null)
                }),
                StatusCode::BAD_REQUEST,
            )
        }
    };

    utils::response::json(
        serde_json::json!(CreateResponse {
            status: StatusCode::CREATED.into(),
            message: "Successfully created product".to_string(),
            data: serde_json::json!(input)
        }),
        StatusCode::CREATED,
    )
}
