use axum::{routing::post, Router};

// internal
use crate::instructions;

pub struct Routes {
    r: Router,
}

impl Routes {
    pub fn new() -> Self {
        let r = Router::new().nest(
            "/users",
            Router::new().route("/login", post(instructions::login)),
        );

        Self { r }
    }

    pub fn get_routes(&self) -> Router {
        self.r.clone()
    }
}
