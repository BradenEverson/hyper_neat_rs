use std::default;

use slotmap::SlotMap;

use super::node::{Node, NodeId};

pub struct ANN {
    species: u32,
    nodes: SlotMap<NodeId, Node>,
    inputs: Vec<NodeId>,
    outputs: Vec<NodeId>
}

impl Default for ANN {
    fn default() -> Self {
        Self::new()
    }
}

impl ANN {
    pub fn new() -> Self {
        let nodes: SlotMap<NodeId, Node> = SlotMap::with_key();
        ANN { species: 0, nodes, inputs: vec![], outputs: vec![] }
    }
}