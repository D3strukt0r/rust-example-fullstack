use axum::{extract::Path, response::IntoResponse, routing::get, Router, Json};
use serde::{Deserialize, Serialize};
use axum::routing::post;
use model::ShoppingListItem;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/:name", get(hello_name))
        .route("/your-route", post(workshop_echo))
        .route("/items", get(get_items))
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn hello_world() -> impl IntoResponse {
    "Hello World"
}

async fn hello_name(Path(name): Path<String>) -> impl IntoResponse {
    format!("Hello {name}")
}

#[derive(Serialize, Deserialize)]
struct Workshop {
    attendees_count: i32,
    people_like_it: bool,
}

async fn workshop_echo(Json(workshop): Json<Workshop>) -> impl IntoResponse {
    Json(workshop)
}

async fn get_items() -> impl IntoResponse {
    let items = vec!["milk", "eggs", "potatoes", "dogfood"];

    let uuid: &str = "a28e2805-196b-4cdb-ba5c-a1ac18ea264a";
    let result: Vec<ShoppingListItem> = items
        .iter()
        .map(|item| ShoppingListItem {
            title: item.to_string(),
            posted_by: "Roland".to_string(),
            uuid: uuid.to_string(),
        })
        .collect();

    Json(result)
}
