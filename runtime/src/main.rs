use common::{
    axum::{Router, Server},
    tokio,
    utoipa::OpenApi,
};
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() {
    // address to listen on
    let addr = "0.0.0.0:1337";
    let routes = Router::new()
        .nest(
            "/api",
            Router::new()
                .merge(products::routes::Routes::new().get_routes())
                .merge(users::routes::Routes::new().get_routes()),
        )
        .merge(SwaggerUi::new("/apidoc").urls(vec![
            (
                "/apidoc/products/openapi.json".into(),
                products::api_doc::ApiDoc::openapi(),
            ),
            (
                "/apidoc/users/openapi.json".into(),
                users::api_doc::ApiDoc::openapi(),
            ),
        ]));

    println!("\n🚀 Listening on http://{}\n", addr);
    if let Err(e) = Server::bind(&addr.parse().unwrap())
        .serve(routes.into_make_service())
        .await
    {
        eprintln!("Server error: {}", e);
    }
}
