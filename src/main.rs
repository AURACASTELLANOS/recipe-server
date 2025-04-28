use askama::Template;
use axum::{
    response::Html,
    routing::get,
    Router,
    serve, // correcto
};
use rand::seq::SliceRandom;
use serde::Deserialize;
use std::fs;
use tokio::net::TcpListener;


#[derive(Debug, Deserialize)]
struct Recipe {
    title: String,
    ingredients: Vec<String>,
    instructions: String,
}

#[template(path = "recipe.html")]
struct RecipeTemplate {
    title: String,
    ingredients: Vec<String>,
    instructions: String,
}

async fn recipe_handler() -> Html<String> {
    // Leer el archivo recipes.json
    let data = fs::read_to_string("recipes.json").expect("Unable to read recipes.json");
    let recipes: Vec<Recipe> = serde_json::from_str(&data).expect("JSON was not well-formatted");

    //choose random recipe
    let mut rng = rand::thread_rng();
    let recipe = recipes.choose(&mut rng).expect("No recipes found");

    let template = RecipeTemplate {
        title: recipe.title.clone(),
        ingredients: recipe.ingredients.clone(),
        instructions: recipe.instructions.clone(),
    };

    Html(template.render().unwrap())
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(recipe_handler));

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Server running at http://127.0.0.1:3000");

    serve(listener, app).await.unwrap();
}
