// @generated automatically by Diesel CLI.

diesel::table! {
    ingredients (id) {
        id -> Integer,
        name -> Text,
        quantity -> Float,
        unit -> Text,
        recipe_id -> Integer,
    }
}

diesel::table! {
    recipes (id) {
        id -> Integer,
        title -> Text,
        description -> Text,
    }
}

diesel::table! {
    steps (id) {
        id -> Integer,
        description -> Text,
        recipe_id -> Integer,
    }
}

diesel::joinable!(ingredients -> recipes (recipe_id));
diesel::joinable!(steps -> recipes (recipe_id));

diesel::allow_tables_to_appear_in_same_query!(ingredients, recipes, steps,);
