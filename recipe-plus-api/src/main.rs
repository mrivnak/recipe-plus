use crate::responses::GetRecipesResponse;
use axum::extract::State;
use axum::http::StatusCode;
use axum::{routing::get, Json, Router};
use deadpool_diesel::sqlite::{Manager, Pool};
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use tokio::net::TcpListener;

mod models;
mod responses;
mod schema;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = Manager::new(db_url, deadpool_diesel::Runtime::Tokio1);
    let pool: Pool = Pool::builder(manager).build().unwrap();

    let api_router = Router::new()
        .route("/status", get(|| async { "OK" }))
        .route("/recipes", get(get_recipes))
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

async fn get_recipes(
    State(pool): State<Pool>,
) -> Result<Json<GetRecipesResponse>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;
    let res = conn
        .interact(|conn| {
            schema::recipes::table
                .select(models::Recipe::as_select())
                .load::<models::Recipe>(conn)
        })
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;
    Ok(Json(GetRecipesResponse { recipes: res }))
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
