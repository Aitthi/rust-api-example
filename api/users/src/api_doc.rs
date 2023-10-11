use utoipa::{OpenApi, Modify};

// internal
use crate::resource;

// const
pub const BASE_PATH: &str = "/api/users";
pub const BASE_DESCRIPTION: &str = "Base path for users API";

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
            resource::LoginResponse,
            resource::LoginInput
        )
    )
)]
pub struct ApiDoc;

struct ServerBase;
impl Modify for ServerBase {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let mut binding = Box::new(&mut openapi.paths);
        let mut tag = utoipa::openapi::Tag::default();
        tag.name = BASE_PATH.to_string();
        tag.description = Some(BASE_DESCRIPTION.to_string());

        openapi.tags = Some(vec![tag]);
        for (_, item) in binding.as_mut().paths.iter_mut() {
            item.servers = Some(vec![
                utoipa::openapi::Server::new(BASE_PATH)
            ]);
        }

    }
}
