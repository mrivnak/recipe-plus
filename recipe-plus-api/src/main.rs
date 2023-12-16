use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{routing::get, Json, Router};
use deadpool_diesel::sqlite::{Manager, Pool};
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use tokio::net::TcpListener;
use crate::errors::internal_error;
use crate::models::{CreateRecipe, Ingredient, Recipe};

mod models;
mod schema;
mod errors;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = Manager::new(db_url, deadpool_diesel::Runtime::Tokio1);
    let pool: Pool = Pool::builder(manager).build().unwrap();

    let api_router = Router::new()
        .route("/status", get(|| async { "OK" }))
        .route("/recipes", get(get_recipes).post(create_recipe))
        .route("/recipes/:id", get(get_recipe))
        .route("/recipes/:id/ingredients", get(get_ingredients))
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
) -> Result<Json<Vec<Recipe>>, (StatusCode, String)> {
    use schema::recipes::dsl::*;

    let conn = pool.get().await.map_err(internal_error)?;
    let res = conn
        .interact(|conn| {
            recipes
                .select(Recipe::as_select())
                .load::<Recipe>(conn)
        })
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;
    Ok(Json(res))
}

async fn get_recipe(
    Path(r_id): Path<i32>,
    State(pool): State<Pool>,
) -> Result<Json<Recipe>, (StatusCode, String)> {
    use schema::recipes::dsl::*;

    let conn = pool.get().await.map_err(internal_error)?;
    let res = conn
        .interact(move |conn| {
            recipes
                .find(r_id)
                .select(Recipe::as_select())
                .first(conn)
        })
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;
    Ok(Json(res))
}

async fn create_recipe(
    State(pool): State<Pool>,
    Json(recipe): Json<CreateRecipe>,
) -> Result<(StatusCode, Json<Recipe>), (StatusCode, String)> {
    use schema::recipes::dsl::*;

    let conn = pool.get().await.map_err(internal_error)?;
    let r_id = conn
        .interact(move |conn| {
            diesel::insert_into(recipes)
                .values(&recipe)
                .returning(Recipe::as_select())
                .get_result(conn)
        })
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;
    Ok((StatusCode::CREATED, Json(r_id)))
}

async fn get_ingredients(
    Path(r_id): Path<i32>,
    State(pool): State<Pool>,
) -> Result<Json<Vec<Ingredient>>, (StatusCode, String)> {
    use schema::ingredients::dsl::*;

    let conn = pool.get().await.map_err(internal_error)?;
    let res = conn
        .interact(move |conn| {
            ingredients
                .select(Ingredient::as_select())
                .filter(recipe_id.eq(r_id))
                .load::<Ingredient>(conn)
        })
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;
    Ok(Json(res))
}
