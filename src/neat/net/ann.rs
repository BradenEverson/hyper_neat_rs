use std::collections::{HashMap, HashSet, LinkedList};

use slotmap::SlotMap;

use super::{edge::Edge, error::{AnnError, Result}, initializer::Initializer, node::{Node, NodeId, NodeType}};

pub struct ANN {
    pub(super) species: u32,
    pub(super) nodes: SlotMap<NodeId, Node>,
    pub(super) inputs: Vec<NodeId>,
    pub(super) outputs: Vec<NodeId>,

    seed: Option<String>
}

impl Default for ANN {
    fn default() -> Self {
        Self::new()
    }
}

impl ANN {
    pub fn new() -> Self {
        let nodes: SlotMap<NodeId, Node> = SlotMap::with_key();
        ANN { species: 0, nodes, inputs: vec![], outputs: vec![], seed: None }
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

    pub fn with_seed(mut self, seed: String) -> Self {
        self.seed = Some(seed);
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

    pub fn edges(&self) -> Vec<Edge> {
        let mut res = vec![];
        for edge in self.nodes.iter().flat_map(|f| f.1.edges.clone()) {
            res.push(edge)
        }

        res
    }
    pub fn nodes(&self) -> Vec<NodeId> {
        let mut res = vec![];
        for node in self.nodes.iter() {
            res.push(node.0)
        }

        res
    }

    pub fn inputs(&self) -> &[NodeId] {
        &self.inputs
    }
    pub fn outputs(&self) -> &[NodeId] {
        &self.outputs
    }
    pub fn inner(&self) -> Vec<NodeId> {
        self.nodes.iter()
            .filter(|(nodeid, _)| !self.inputs.contains(nodeid) && !self.outputs.contains(nodeid))
            .map(|node| node.0).collect::<Vec<_>>()
    }


    pub fn forward<F: Copy + Into<f32>>(&self, inputs: &[F]) -> Result<Vec<f32>> {
        let mut node_vals: HashMap<&NodeId, f32> = HashMap::new();
        let mut to_visit: LinkedList<&NodeId> = LinkedList::new();
        let mut visited: HashSet<&NodeId> = HashSet::new();

        if inputs.len() != self.inputs.len() {
            Err(AnnError::MismatchedInputSizeError(inputs.len(), self.inputs.len()))
        } else {
            for (i, input) in self.inputs.iter().enumerate() {
                to_visit.push_front(input);
                node_vals.insert(input, inputs[i].into());
            }

            while let Some(node) = to_visit.pop_back() {
                if !visited.contains(node) {                
                    for edge in self.get(*node).unwrap().edges.iter() {

                        let mut node_val = *node_vals.get(node).unwrap_or(&0f32);

                        if let Some(act) = &self.get(*node).unwrap().activation {
                            node_val = act.apply(node_val)
                        }

                        if let Some(val) = node_vals.get_mut(&edge.to) {
                            *val += node_val * edge.weight;
                        } else {
                            node_vals.insert(&edge.to, node_val * edge.weight);
                        }

                        to_visit.push_front(&edge.to);
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


    pub fn connect(&mut self, from: NodeId, to: NodeId) -> Result<()> {
        //Ensure from and to both exist in Slotmap
        self.get(to)?;

        if self.get(from)?.ty == NodeType::Output {
            Err(AnnError::InvalidConnectionError(self.get(from)?.clone(), self.get(to)?.clone()))
        } else if from == to {
            Err(AnnError::RecursiveConnectionError(self.get(to)?.clone()))
        } else {
            let from_node = self.get_mut(from)?;

            let new_edge = Edge::new(to, from);

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

    fn num_edges(&self) -> usize {
        let mut res = 0;

        for (_, node) in self.nodes.iter() {
            res += node.edges.len();
        }

        res
    }

    pub fn init(&mut self, initializer: &Initializer) {
        let weights = initializer.gen_range(&self.seed, self.num_edges());
        let all_edges = self.nodes.iter_mut().flat_map(|(_,node)| &mut node.edges).collect::<Vec<&mut Edge>>();

        for (i, edge) in all_edges.into_iter().enumerate() {
            edge.update_weight(weights[i]);
        }
    }
}
