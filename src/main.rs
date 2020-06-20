#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

use serde::Serialize;

pub mod conversion;
pub mod db_connection;
pub mod handlers;
pub mod models;
pub mod schema;

#[derive(Serialize)]
struct Recipe {
    id: i32,
}

fn main() {
    dotenv::dotenv().ok();
    rocket::ignite()
        .manage(db_connection::init_pool())
        .mount(
            "/",
            routes![
                // Recipes
                handlers::recipes::get_single_recipe,
                handlers::recipes::create_single_recipe,
                handlers::recipes::update_single_recipe,
                handlers::recipes::archive_single_recipe,
                
                // Recipe Ingredients
                handlers::ingredients::get_recipe_ingredients,
                handlers::ingredients::create_recipe_ingredient,
                handlers::ingredients::update_recipe_ingredient,
                handlers::ingredients::archive_recipe_ingredient,

                // Ingredients
                handlers::ingredients::create_ingredient,
                handlers::ingredients::get_single_ingredient,
                handlers::ingredients::update_ingredient,
                handlers::ingredients::archive_ingredient,
            ],
        )
        .launch();
}
