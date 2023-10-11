use axum::{routing::post, Router};

// internal
use crate::instructions;

pub struct Routes {
    r: Router,
}

impl Routes {
    pub fn new() -> Self {
        let r = Router::new().nest(
            "/products",
            Router::new().route("/create", post(instructions::create)),
        );

        Self { r }
    }

    pub fn get_routes(&self) -> Router {
        self.r.clone()
    }
}
