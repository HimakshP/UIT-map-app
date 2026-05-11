use crate::graph_utils::CampusMap;
use crate::models::CampusNode;
use axum::{
    Json, Router,
    extract::{Query, State},
    routing::get,
};
use serde::Deserialize;
use tower_http::cors::CorsLayer;
use std::sync::Arc;

// This struct maps to the URL parameters: /path?start=A&end=B
#[derive(Deserialize)]
pub struct PathRequest {
    pub start: String,
    pub end: String,
}

pub fn create_router(state: Arc<CampusMap>) -> Router {
    let cors = CorsLayer::permissive();

    Router::new()
        .route("/health", get(|| async { "OK" }))
        .route("/path", get(get_path_handler))
        .with_state(state)
        .layer(cors)
}
pub(crate) async fn get_path_handler(
    State(map): State<Arc<CampusMap>>,
    Query(params): Query<PathRequest>,
) -> Json<Option<Vec<CampusNode>>> {
    // We call your find_path function from the graph_utils
    let path = map.find_path(params.start, params.end);

    // Axum automatically turns the Vec<CampusNode> into a JSON array
    Json(path)
}
