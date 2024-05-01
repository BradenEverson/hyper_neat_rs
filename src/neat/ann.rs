use slotmap::SlotMap;

use super::node::{Node, NodeId};

pub struct ANN {
    species: u32,
    pub nodes: SlotMap<NodeId, Node>,
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

    pub fn with_species(mut self, species: u32) -> Self {
        self.species = species;
        self
    }

    pub fn with_inputs(mut self, input_count: usize) -> Self {
        for _ in 0..input_count {
            let node = Node::default();
            let node_id = self.insert(node);
            self.inputs.push(node_id);
        }

        self
    }

    pub fn and_outputs(mut self, output_count: usize) -> Self {
        for _ in 0..output_count {
            let node = Node::default();
            let node_id = self.insert(node);
            self.outputs.push(node_id);
        }

        self
    }

    fn insert(&mut self, node: Node) -> NodeId {
        self.nodes.insert(node)
    }

    pub fn species_num(&self) -> u32 {
        self.species
    }
}
