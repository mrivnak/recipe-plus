use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Insertable)]
#[diesel(table_name = crate::schema::recipes)]
pub struct CreateRecipe {
    pub title: String,
    pub description: String,
}


#[derive(Deserialize, Serialize, Queryable, Selectable)]
#[diesel(table_name = crate::schema::recipes)]
pub struct Recipe {
    pub id: i32,
    pub title: String,
    pub description: String,
}

#[derive(Deserialize, Serialize, Queryable, Selectable)]
#[diesel(table_name = crate::schema::ingredients)]
pub struct Ingredient {
    pub id: i32,
    pub name: String,
    pub quantity: f32,
    pub unit: String,
    pub recipe_id: i32,
}

#[derive(Deserialize, Serialize, Queryable, Selectable)]
#[diesel(table_name = crate::schema::steps)]
pub struct Step {
    pub id: i32,
    pub description: String,
    pub recipe_id: i32,
}
