use crate::models::{CampusNode, Connection, Coordinates};
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, UnGraph, node_index};
use std::collections::HashMap;

pub struct CampusMap {
    graf: UnGraph<CampusNode, f32>,
    index_map: HashMap<String, NodeIndex>,
}

pub fn build() {
    let mut g = CampusMap {
        graf: UnGraph::new_undirected(),
        index_map: HashMap::new(),
    };

    for _node_index in g.graf.node_indices() {
        g.graf.add_node(CampusNode {
            node_id: Connection {
                node_id: "hi".to_string(),
                distance: 1.0,
            },
            name: "String".to_string(),
            node_type: "String".to_string(),
            connections: [].to_vec(),
            coordinates: Coordinates {
                x: 7.0,
                y: 7.0,
                floor: "1".to_string(),
            },
        });
    }

    for node_index in g.index_map{

    }
}
