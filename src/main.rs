use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Html,
    routing::{get, post},
    Json, Router,
};
use askama::Template;
use rand::seq::SliceRandom;
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::Mutex;
use utoipa::{OpenApi, ToSchema, path};
use utoipa_swagger_ui::SwaggerUi;

mod model;
use model::Recipe;

type AppState = Arc<Mutex<Vec<Recipe>>>;

#[derive(Template)]
#[template(path = "recipe.html")]
struct RecipeTemplate {
    title: String,
    ingredients: Vec<String>,
    instructions: String,
}

// Handler HTML
async fn recipe_handler(State(state): State<AppState>) -> Html<String> {
    let recipes = state.lock().await;
    let mut rng = rand::thread_rng();
    let recipe = recipes.choose(&mut rng).expect("No recipes found");

    let template = RecipeTemplate {
        title: recipe.title.clone(),
        ingredients: recipe.ingredients.clone(),
        instructions: recipe.instructions.clone(),
    };

    Html(template.render().unwrap())
}

#[utoipa::path(
    get,
    path = "/api/recipes",
    responses((status = 200, description = "Lista de recetas", body = [Recipe]))
)]
async fn list_recipes(State(state): State<AppState>) -> Json<Vec<Recipe>> {
    let recipes = state.lock().await;
    Json(recipes.clone())
}

#[utoipa::path(
    get,
    path = "/api/recipes/{id}",
    params(("id" = u32, Path, description = "ID de la receta")),
    responses(
        (status = 200, description = "Receta encontrada", body = Recipe),
        (status = 404, description = "No encontrada")
    )
)]
async fn get_recipe(Path(id): Path<u32>, State(state): State<AppState>) -> Result<Json<Recipe>, StatusCode> {
    let recipes = state.lock().await;
    recipes
        .iter()
        .find(|r| r.id == id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

#[utoipa::path(
    post,
    path = "/api/recipes",
    request_body = Recipe,
    responses((status = 201, description = "Receta creada", body = Recipe))
)]

async fn create_recipe(
    State(state): State<AppState>,
    Json(new_recipe): Json<Recipe>,
) -> (StatusCode, Json<Recipe>) {
    let mut recipes = state.lock().await;
    recipes.push(new_recipe.clone());
    (StatusCode::CREATED, Json(new_recipe))
}


#[derive(OpenApi)]
#[openapi(
    paths(list_recipes, get_recipe, create_recipe),
    components(schemas(Recipe)),
    tags(
        (name = "Recipes", description = "Recipe API endpoints")
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    let recipes = vec![Recipe {
        id: 1,
        title: "Tacos al Pastor".to_string(),
        ingredients: vec!["Carne de cerdo".to_string(), "Piña".to_string()],
        instructions: "Marinar la carne y cocinarla.".to_string(),
    }];

    let shared_state = Arc::new(Mutex::new(recipes));

    let app = Router::new()
        .route("/", get(recipe_handler))
        .route("/api/recipes", get(list_recipes).post(create_recipe))
        .route("/api/recipes/:id", get(get_recipe))
        .merge(SwaggerUi::new("/docs").url("/api-doc/openapi.json", ApiDoc::openapi()))
        .with_state(shared_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}/", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

