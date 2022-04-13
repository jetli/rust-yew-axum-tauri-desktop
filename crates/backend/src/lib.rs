use std::net::SocketAddr;

use axum::{http::Method, response::IntoResponse, routing::get, Router};
use tower_http::cors::{Any, CorsLayer};

pub async fn app() {
    let app = Router::new().route("/", get(handler)).layer(
        CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(vec![Method::GET, Method::POST]),
    );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> impl IntoResponse {
    "Hello, from backend!"
}
