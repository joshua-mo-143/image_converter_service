use axum::{
    routing::{get, post},
    Router,
};

mod routes;
use routes::convert::{convert_image, make_zipfile};

#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/convert", post(convert_image))
        .route("/convertzip", get(make_zipfile));

    let addr: std::net::SocketAddr = "0.0.0.0:8000".parse().unwrap();

    println!("Starting your service at port 8000...");
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
