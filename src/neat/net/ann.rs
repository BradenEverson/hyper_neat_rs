use std::collections::{HashMap, HashSet};

use slotmap::SlotMap;

use super::{edge::Edge, error::{AnnError, Result}, node::{Node, NodeId, NodeType}};

pub struct ANN {
    pub(super) species: u32,
    pub(super) nodes: SlotMap<NodeId, Node>,
    pub(super) inputs: Vec<NodeId>,
    pub(super) outputs: Vec<NodeId>
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


    pub fn forward<F: Copy + Into<f32>>(&self, inputs: &[F]) -> Result<Vec<f32>> {
        let mut node_vals: HashMap<&NodeId, f32> = HashMap::new();
        let mut to_visit: Vec<&NodeId> = vec![];
        let mut visited: HashSet<&NodeId> = HashSet::new();

        if inputs.len() != self.inputs.len() {
            Err(AnnError::MismatchedInputSizeError(inputs.len(), self.inputs.len()))
        } else {
            for i in 0..inputs.len() {
                to_visit.push(&self.inputs[i]);
                node_vals.insert(&self.inputs[i], inputs[i].into());
            }

            while let Some(node) = to_visit.pop() {
                if !visited.contains(node) {                
                    for edge in self.get(*node).unwrap().edges.iter() {

                        let node_val = node_vals.get(node).unwrap_or(&0f32).clone();

                        if let Some(val) = node_vals.get_mut(&edge.to) {
                            *val += node_val * edge.weight;
                        } else {
                            node_vals.insert(&edge.to, node_val * edge.weight);
                        }

                        to_visit.push(&edge.to);
                    }
                    visited.insert(node);
                }
            }

            let mut res = vec![];

            for output in self.outputs.iter() {
                res.push(*node_vals.get(output).unwrap_or(&0f32));
            }

            Ok(res)
        }
    }

    pub(crate) fn insert(&mut self, node: Node) -> NodeId {
        self.nodes.insert(node)
    }


    pub(crate) fn connect(&mut self, from: NodeId, to: NodeId) -> Result<()> {
        //Ensure from and to both exist in Slotmap
        self.get(to)?;

        if self.get(from)?.ty == NodeType::Output {
            Err(AnnError::InvalidConnectionError(self.get(from)?.clone(), self.get(to)?.clone()))
        } else if from == to {
            Err(AnnError::RecursiveConnectionError(self.get(to)?.clone()))
        } else {
            let from_node = self.get_mut(from)?;

            let new_edge = Edge::new(from, to);

            from_node.edges.push(new_edge);

            Ok(())
        }
    }
    
    fn get(&self, id: NodeId) -> Result<&Node> {
        match self.nodes.get(id) {
            Some(node) => Ok(node),
            None => Err(AnnError::InvalidNodeIDError)            
        }
    }
    
    fn get_mut(&mut self, id: NodeId) -> Result<&mut Node> {
        match self.nodes.get_mut(id) {
            Some(node) => Ok(node),
            None => Err(AnnError::InvalidNodeIDError)            
        }
    }
}
