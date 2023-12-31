use crate::errors::{diesel_error, internal_error};
use crate::models::{CreateRecipe, Ingredient, Recipe};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{routing::get, Json, Router};
use deadpool_diesel::sqlite::Pool;
use diesel::prelude::*;

pub fn router() -> Router<Pool> {
    Router::new()
        .route("/", get(get_recipes).post(create_recipe))
        .route(
            "/:id",
            get(get_recipe).put(update_recipe).delete(delete_recipe),
        )
        .route("/:id/ingredients", get(get_ingredients))
}

async fn get_recipes(State(pool): State<Pool>) -> Result<Json<Vec<Recipe>>, (StatusCode, String)> {
    use crate::schema::recipes::dsl::*;

    let conn = pool.get().await.map_err(internal_error)?;
    let res = conn
        .interact(|conn| recipes.select(Recipe::as_select()).load::<Recipe>(conn))
        .await
        .map_err(internal_error)?
        .map_err(diesel_error)?;
    Ok(Json(res))
}

async fn get_recipe(
    Path(r_id): Path<i32>,
    State(pool): State<Pool>,
) -> Result<Json<Recipe>, (StatusCode, String)> {
    use crate::schema::recipes::dsl::*;

    let conn = pool.get().await.map_err(internal_error)?;
    let res = conn
        .interact(move |conn| recipes.find(r_id).select(Recipe::as_select()).first(conn))
        .await
        .map_err(internal_error)?
        .map_err(diesel_error)?;
    Ok(Json(res))
}

async fn create_recipe(
    State(pool): State<Pool>,
    Json(recipe): Json<CreateRecipe>,
) -> Result<(StatusCode, Json<Recipe>), (StatusCode, String)> {
    use crate::schema::recipes::dsl::*;

    let conn = pool.get().await.map_err(internal_error)?;
    let new_recipe = conn
        .interact(move |conn| {
            diesel::insert_into(recipes)
                .values(&recipe)
                .returning(Recipe::as_select())
                .get_result(conn)
        })
        .await
        .map_err(internal_error)?
        .map_err(diesel_error)?;
    Ok((StatusCode::CREATED, Json(new_recipe)))
}

async fn update_recipe(
    Path(r_id): Path<i32>,
    State(pool): State<Pool>,
    Json(recipe): Json<CreateRecipe>,
) -> Result<Json<Recipe>, (StatusCode, String)> {
    use crate::schema::recipes::dsl::*;

    let conn = pool.get().await.map_err(internal_error)?;
    let new_recipe = conn
        .interact(move |conn| {
            diesel::update(recipes.find(r_id))
                .set(&recipe)
                .returning(Recipe::as_select())
                .get_result(conn)
        })
        .await
        .map_err(internal_error)?
        .map_err(diesel_error)?;
    Ok(Json(new_recipe))
}

async fn delete_recipe(
    Path(r_id): Path<i32>,
    State(pool): State<Pool>,
) -> Result<StatusCode, (StatusCode, String)> {
    use crate::schema::recipes::dsl::*;

    let conn = pool.get().await.map_err(internal_error)?;
    let _ = conn
        .interact(move |conn| diesel::delete(recipes.find(r_id)).execute(conn))
        .await
        .map_err(internal_error)?
        .map_err(diesel_error)?;
    Ok(StatusCode::NO_CONTENT)
}

async fn get_ingredients(
    Path(r_id): Path<i32>,
    State(pool): State<Pool>,
) -> Result<Json<Vec<Ingredient>>, (StatusCode, String)> {
    use crate::schema::ingredients::dsl::*;

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
        .map_err(diesel_error)?;
    Ok(Json(res))
}
