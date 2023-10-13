use common::axum::{routing::post, Router};

// internal
use crate::resource;

pub struct Routes {
    r: Router,
}

impl Routes {
    pub fn new() -> Self {
        let r = Router::new().nest(
            "/users",
            Router::new().route("/login", post(resource::login)),
        );

        Self { r }
    }

    pub fn get_routes(&self) -> Router {
        self.r.clone()
    }
}
