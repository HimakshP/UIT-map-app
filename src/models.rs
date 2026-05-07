use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Coordinates {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) floor: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Connection {
    pub(crate) node_id: String,
    pub(crate) distance: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CampusNode {
    pub(crate) node_id: String,
    pub(crate) name: String,
    
    pub(crate) node_type: String,
    pub(crate) connections: Vec<Connection>,
    pub(crate) coordinates: Coordinates,
}
