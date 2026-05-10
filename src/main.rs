mod graph_utils;
mod models;
mod routes;

use graph_utils::{CampusMap, load_data_from_file};
use std::net::SocketAddr;
use std::sync::Arc;

use crate::graph_utils::build;
use crate::routes::create_router;

#[tokio::main]
async fn main() {
    // 1. Initialize the "Brain"
    let raw_data = load_data_from_file("data/campus.json");
    let map = build(raw_data);

    // 2. Wrap in Arc so multiple web requests can read it safely at once
    let shared_state = Arc::new(map);

    // 3. Build the router (defined in routes.rs)
    let app = create_router(shared_state);

    // 4. Start the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Campus Navigation Server running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
