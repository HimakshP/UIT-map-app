use crate::models::{CampusNode, Connection, Coordinates};
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, UnGraph, node_index};
use std::collections::HashMap;

pub struct CampusMap {
    graf: UnGraph<CampusNode, f32>,
    index_map: HashMap<String, NodeIndex>,
}

pub fn create_mock_data() -> Vec<CampusNode> {
    let node1 = CampusNode {
        node_id: "A".to_string(),
        name: "Main Gate".to_string(),
        node_type: "entrance".to_string(),
        coordinates: Coordinates {
            x: 0.0,
            y: 0.0,
            floor: "1".to_string(),
        },
        connections: vec![Connection {
            node_id: "B".to_string(),
            distance: 10.0,
        }],
    };

    let node2 = CampusNode {
        node_id: "B".to_string(),
        name: "Library".to_string(),
        node_type: "building".to_string(),
        coordinates: Coordinates {
            x: 10.0,
            y: 0.0,
            floor: "1".to_string(),
        },
        connections: vec![Connection {
            node_id: "A".to_string(),
            distance: 10.0,
        }],
    };

    vec![node1, node2]
}

// pub fn build() {
//     let mut g = CampusMap {
//         graf: UnGraph::new_undirected(),
//         index_map: HashMap::new(),
//     };

//     for _node_index in g.graf.node_indices() {
//         g.graf.add_node(CampusNode {
//             node_id: Connection {
//                 node_id: "hi".to_string(),
//                 distance: 1.0,
//             },
//             name: "String".to_string(),
//             node_type: "String".to_string(),
//             connections: [].to_vec(),
//             coordinates: Coordinates {
//                 x: 7.0,
//                 y: 7.0,
//                 floor: "1".to_string(),
//             },
//         });
//     }

//     for node_index in g.index_map{

//     }
// }

pub fn build(nodes: Vec<CampusNode>) -> CampusMap {
    let mut graf = UnGraph::<CampusNode, f32>::new_undirected();
    let mut index_map = HashMap::new();

    for node_data in nodes {
        // we iterate through the nodes one by one
        let id = node_data.node_id.clone(); // get the node_id: String for each node 

        let idx = graf.add_node(node_data); // returns node index of this node 

        index_map.insert(id, idx); // put the pair of id and index in the hashmap
    }

    for (_id, &current_idx) in index_map.iter() {
        // we iterate through node indices in index map

        let node_data = &graf[current_idx]; // create a variable to access data for current node from the graf

        for conn in node_data.connections.clone() {
            // we iterate through connection vector

            if let Some(&neighbor_idx) = index_map.get(&conn.node_id) {
                // create  a variable for a neighbor index and get the value index)for the new node

                graf.add_edge(current_idx, neighbor_idx, conn.distance); // create the edge from current node to neighbor node with distance 
            }
        }
    }

    CampusMap { graf, index_map }
}
