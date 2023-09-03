use axum::{
    routing::{get, post},
    Router,
};
use std::path::Path;
use tokio::fs::create_dir;
mod routes;
mod tasks;
use routes::convert::{convert_image, make_zipfile};
use tasks::maintenance::delete_old_uploads;
#[tokio::main]
async fn main() {
    if !Path::new("uploads").is_dir() {
        create_dir("uploads").await.unwrap();
    }

    let router = Router::new()
        .route("/convert", post(convert_image))
        .route("/convertzip", get(make_zipfile));

    let addr: std::net::SocketAddr = "0.0.0.0:8000".parse().unwrap();

    println!("Starting your service at port 8000...");
    let server = axum::Server::bind(&addr).serve(router.into_make_service());

    tokio::select! {
        _ = server => {},
        _ = delete_old_uploads() => {}
    }

    println!("Looks like something went wrong :( The `tokio::select!` macro shouldn't die");
}
