use common::{
    axum::{self, body::Body, response::Response, Router},
    http::StatusCode,
    tokio::{self, signal},
    utoipa::OpenApi,
};
use tower::{make::Shared, ServiceBuilder};
use tower_http::cors::{Any, CorsLayer};
use utoipa_swagger_ui::SwaggerUi;

// state is shut down
static mut IS_SHUTDOWN: bool = false;

#[tokio::main]
async fn main() {
    let conf = config::get_config();
    // init database
    database::init().await;
    // address to listen on
    let addr = &conf.runtime.addr;
    let routes = Router::new()
        .route("/health_check", common::axum::routing::get(health_check))
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
    let Ok(listener) = tokio::net::TcpListener::bind(addr).await else {
        eprintln!("Failed to bind to address");
        return;
    };

    // allow cors
    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_origin(Any);

    // build the service
    let service = ServiceBuilder::new().layer(cors).service(routes.clone());
    let service = Shared::new(service);
    let server = axum::serve(listener, service);
    // shutdown signal
    tokio::spawn(async move {
        if let Err(e) = server.await {
            eprintln!("Server error: {}", e);
        }
    });
    tokio::select! {
        _ = signal::ctrl_c() => {
            shutdown().await;
        }
    }
    println!("✅ Shutdown complete");
}

async fn shutdown() {
    println!("\n🛑 Shutting down...");
    unsafe {
        IS_SHUTDOWN = true;
    }
    let conf = config::get_config();
    // countdown
    for i in (0..conf.shutdown_signal).rev() {
        println!("Shutting down in {} seconds", i);
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}

async fn health_check() -> Response<Body> {
    if unsafe { IS_SHUTDOWN } {
        Response::builder()
            .status(StatusCode::SERVICE_UNAVAILABLE)
            .body(Body::from("Service is shutting down"))
            .unwrap()
    } else {
        Response::builder()
            .status(StatusCode::OK)
            .body(Body::from("OK"))
            .unwrap()
    }
}
