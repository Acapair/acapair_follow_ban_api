use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use tower_http::cors::CorsLayer;

use crate::{db::db_operations, utils::database_config, AppState};

pub async fn routing(State(state): State<AppState>) -> Router {
    Router::new()
        .route("/", get(alive))
        .route("/create/:username", get(create))
        .route("/delete/:username", get(delete))
        .route("/search-username/:username", get(search_username))
        .route("/search-id/:id", get(search_id))
        .route(
            "/change-username/:username/:updated_username",
            get(change_username),
        )
        .route("/follow/:follower/:followed", get(follow))
        .route("/unfollow/:follower/:followed", get(unfollow))
        .route("/ban/:victim/:judge", get(ban))
        .route("/unban/:victim/:judge", get(unban))
        .layer(CorsLayer::permissive())
        .with_state(state.clone())
}

async fn alive() -> impl IntoResponse {
    let ping = match db_operations::connect(&database_config().await).await {
        Some(_) => "Alive",
        None => "Dead",
    };
    let alive_json = serde_json::json!({
        "server_status":"Alive",
        "database_status":ping,
    });
    (StatusCode::OK, Json(alive_json))
}

async fn create(Path(username): Path<String>, State(state): State<AppState>) -> impl IntoResponse {
    match db_operations::create(&username, &state.db).await {
        Some(channel) => {
            let create = serde_json::json!({
                "channel":channel,
            });
            (StatusCode::CREATED, Json(create))
        }
        None => (StatusCode::NOT_ACCEPTABLE, Json(serde_json::json!(""))),
    }
}
async fn delete(Path(username): Path<String>, State(state): State<AppState>) -> impl IntoResponse {
    match db_operations::delete(&username, &state.db).await {
        Some(channel) => {
            let delete = serde_json::json!({
                "channel":channel,
            });
            (StatusCode::NO_CONTENT, Json(delete))
        }
        None => (StatusCode::NOT_ACCEPTABLE, Json(serde_json::json!(""))),
    }
}
async fn search_username(
    Path(username): Path<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    match db_operations::search_username(&username, &state.db).await {
        Some(channel) => {
            let search_username = serde_json::json!({
                "channel":channel,
            });
            (StatusCode::OK, Json(search_username))
        }
        None => (StatusCode::NOT_ACCEPTABLE, Json(serde_json::json!(""))),
    }
}
async fn search_id(Path(id): Path<String>, State(state): State<AppState>) -> impl IntoResponse {
    match db_operations::search_id(&id, &state.db).await {
        Some(channel) => {
            let search_id = serde_json::json!({
                "channel":channel,
            });
            (StatusCode::OK, Json(search_id))
        }
        None => (StatusCode::NOT_ACCEPTABLE, Json(serde_json::json!(""))),
    }
}
async fn change_username(
    Path((username, updated_username)): Path<(String, String)>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    match db_operations::change_username(&updated_username, &username, &state.db).await {
        Some(channel) => {
            let change_username = serde_json::json!({
                "channel":channel,
            });
            (StatusCode::OK, Json(change_username))
        }
        None => (StatusCode::NOT_ACCEPTABLE, Json(serde_json::json!(""))),
    }
}
async fn follow(
    Path((follower, followed)): Path<(String, String)>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    match db_operations::follow(&follower, &followed, &state.db).await {
        Some(channel) => {
            let follow = serde_json::json!({
                "channel":channel,
            });
            (StatusCode::OK, Json(follow))
        }
        None => (StatusCode::NOT_ACCEPTABLE, Json(serde_json::json!(""))),
    }
}
async fn unfollow(
    Path((follower, followed)): Path<(String, String)>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    match db_operations::unfollow(&follower, &followed, &state.db).await {
        Some(channel) => {
            let unfollow = serde_json::json!({
                "channel":channel,
            });
            (StatusCode::OK, Json(unfollow))
        }
        None => (StatusCode::NOT_ACCEPTABLE, Json(serde_json::json!(""))),
    }
}
async fn ban(
    Path((victim, judge)): Path<(String, String)>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    match db_operations::ban(&victim, &judge, &state.db).await {
        Some(channel) => {
            let ban = serde_json::json!({
                "channel":channel,
            });
            (StatusCode::OK, Json(ban))
        }
        None => (StatusCode::NOT_ACCEPTABLE, Json(serde_json::json!(""))),
    }
}
async fn unban(
    Path((victim, judge)): Path<(String, String)>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    match db_operations::unban(&victim, &judge, &state.db).await {
        Some(channel) => {
            let unban = serde_json::json!({
                "channel":channel,
            });
            (StatusCode::OK, Json(unban))
        }
        None => (StatusCode::NOT_ACCEPTABLE, Json(serde_json::json!(""))),
    }
}
