use crate::models::Recipe;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct GetRecipesResponse {
    pub recipes: Vec<Recipe>,
}
