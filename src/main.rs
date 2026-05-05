mod models;
mod graph_utils;

use graph_utils::{create_mock_data, build};


fn main() {
    
    let data = create_mock_data();

    let campus_map = build(data);
    
    println!("Searching for path from A to B...");
    
    if let Some(path) = campus_map.find_path("A".to_string(), "B".to_string()) {
        println!("Path found! Steps:");
        for node in path {
            println!(" -> {} (at {}, {})", node.name, node.coordinates.x, node.coordinates.y);
        }
    } else {
        println!("No path found. Check your IDs!");
    }
}