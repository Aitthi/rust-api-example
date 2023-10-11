use utoipa::OpenApi;

// internal
use crate::resource;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Users API",
        version = "1.0.0",
        description = "An API to manage users"
    ),
    paths(
        resource::login
    ),
    components(
        schemas(
            resource::LoginResponse,
            resource::LoginInput
        )
    )
)]
pub struct ApiDoc;
