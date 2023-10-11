use utoipa::OpenApi;

// internal
use crate::resource;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Products API",
        version = "1.0.0",
        description = "An API to manage products"
    ),
    paths(
        resource::create
    ),
    components(
        schemas(
            resource::CreateResponse,
            resource::CreateInput
        )
    )
)]
pub struct ApiDoc;
