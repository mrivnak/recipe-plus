use axum::{routing::get, Router};
use deadpool_diesel::sqlite::{Manager, Pool};
use dotenvy::dotenv;
use std::env;
use tokio::net::TcpListener;

mod controllers;
mod errors;
mod models;
mod schema;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = Manager::new(db_url, deadpool_diesel::Runtime::Tokio1);
    let pool: Pool = Pool::builder(manager).build().unwrap();

    let api_router = Router::new()
        .route("/status", get(|| async { "OK" }))
        .nest("/recipes", controllers::recipe::router())
        .with_state(pool);

    axum::serve(
        TcpListener::bind("0.0.0.0:8000")
            .await
            .expect("Unable to bind to port"),
        api_router.into_make_service(),
    )
    .await
    .unwrap();
}
