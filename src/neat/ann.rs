use std::default;

use slotmap::SlotMap;

use super::node::{Node, NodeId};

pub struct ANN {
    species: u32,
    nodes: SlotMap<NodeId, Node>,
    inputs: Vec<NodeId>,
    inner: Vec<NodeId>,
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
        ANN { species: 0, nodes, inputs: vec![], outputs: vec![], inner: vec![] }
    }

    pub fn with_species(&mut self, species: u32) {
        self.species = species
    }

    pub fn with_inputs(&mut self, input_count: usize) {
        self.inputs = Vec::with_capacity(input_count);
        for i in 0..input_count {
            let node = Node::default();
            self.inputs[i] = self.insert(node);
        }
    }

    pub fn and_outputs(&mut self, output_count: usize) {
        self.outputs = Vec::with_capacity(output_count);
        for i in 0..output_count {
            let node = Node::default();
            self.inputs[i] = self.insert(node);
        }
    }

    fn insert(&mut self, node: Node) -> NodeId {
        self.nodes.insert(node)
    }
}