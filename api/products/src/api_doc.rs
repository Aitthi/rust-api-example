use utoipa::{
    OpenApi,
    Modify
};

// internal
use crate::resource::{self, create};

// const
pub const BASE_PATH: &str = "/api/products";

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
    modifiers(&ServerBase),
    components(
        schemas(
            create::CreateResponse,
            create::CreateInput
        )
    )
)]
pub struct ApiDoc;

struct ServerBase;
impl Modify for ServerBase {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let paths = openapi.paths.paths.clone();
        openapi.paths.paths.clear();
        for (path, item) in paths.iter() {
            let path = format!("{}{}", BASE_PATH, path);
            openapi.paths.paths.insert(path, item.clone());
        }
    }
}
