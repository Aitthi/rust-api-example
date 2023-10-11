use utoipa::{OpenApi, Modify};

// internal
use crate::resource::{self, login};

// const
pub const BASE_PATH: &str = "/api/users";

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
    modifiers(&ServerBase),
    components(
        schemas(
            login::LoginResponse,
            login::LoginInput
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
