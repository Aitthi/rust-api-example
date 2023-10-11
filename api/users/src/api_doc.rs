use utoipa::OpenApi;

// internal
use crate::instructions;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Users API",
        version = "1.0.0",
        description = "An API to manage users"
    ),
    paths(
        instructions::login
    ),
    components(
        schemas(
            instructions::LoginResponse,
            instructions::LoginInput
        )
    )
)]
pub struct ApiDoc;
