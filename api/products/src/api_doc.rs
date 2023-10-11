use utoipa::OpenApi;

// internal
use crate::instructions;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Products API",
        version = "1.0.0",
        description = "An API to manage products"
    ),
    paths(
        instructions::create
    ),
    components(
        schemas(
            instructions::CreateResponse,
            instructions::CreateInput
        )
    )
)]
pub struct ApiDoc;
