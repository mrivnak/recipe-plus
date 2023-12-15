use axum::{
    routing::get,
    Router,
};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let api_router = Router::new().route("/status", get(|| async { "OK" }));

    // run it with hyper on localhost:8000
    axum::serve(TcpListener::bind("0.0.0.0:8000").await.expect("Unable to bind to port"), api_router.into_make_service())
        .await
        .unwrap();
}
