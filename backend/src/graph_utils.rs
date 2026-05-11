use crate::models::{CampusNode, Connection, Coordinates};
use petgraph::algo::astar;
use petgraph::graph::{NodeIndex, UnGraph};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

pub struct CampusMap {
    graf: UnGraph<CampusNode, f32>,
    index_map: HashMap<String, NodeIndex>,
}

pub fn load_data_from_file(path: &str) -> Vec<CampusNode> {
    let file = File::open(path).expect("Failed to open campus.json");
    let reader = BufReader::new(file);
    let nodes: Vec<CampusNode> = serde_json::from_reader(reader)
        .expect("Failed to parse JSON. Check your commas and brackets!");
    nodes
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

impl CampusMap {
    pub fn find_path(&self, start_id: String, end_id: String) -> Option<Vec<CampusNode>> {
        let st = self.index_map.get(&start_id).copied().unwrap_or(0.into());
        let end = self.index_map.get(&end_id).copied().unwrap_or(0.into());

        let path = astar(
            &self.graf,
            st,
            |finish| finish == end,
            |e| *e.weight(),
            |_| 0.0,
        );

        if let Some((_total_cost, actual_path_nodes)) = path {
            // Now 'path' is just the Vec<NodeIndex>
            let mut path_in_vec = Vec::new();

            for idx in actual_path_nodes {
                // Now you can index the graph correctly!
                let node_data = self.graf[idx].clone();
                path_in_vec.push(node_data);
            }

            Some(path_in_vec)
        } else {
            None // No path was found
        }
        // Now you have a Vec<CampusNode> to send to the frontend!
    }
}
