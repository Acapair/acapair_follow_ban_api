use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use tower_http::cors::CorsLayer;

use crate::{db::db_operations, AppState};

pub async fn routing(State(state): State<AppState>) -> Router {
    Router::new()
        .route("/", get(alive))
        .layer(CorsLayer::permissive())
        .with_state(state.clone())
}

async fn alive() -> impl IntoResponse {
    let ping = match db_operations::connect().await {
        Some(_) => "Alive",
        None => "Dead",
    };
    let alive_json = serde_json::json!({
        "server_status":"Alive",
        "database_status":ping,
    });
    println!("{}", alive_json);
    (StatusCode::OK, Json(alive_json))
}
