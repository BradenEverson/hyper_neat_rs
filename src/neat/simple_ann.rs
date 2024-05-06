use std::collections::HashMap;

use super::net::{ann::ANN, error::{AnnError, Result}};

pub struct SimpleANN {
    dims: Vec<usize>,
    nodes: Vec<usize>,
    //From, To, Weight
    edges: Vec<(usize, usize, f32)>
}

impl From<ANN> for SimpleANN {
    fn from(value: ANN) -> Self {
        let mut node_mappings = HashMap::new();

        let inputs = value.inputs();
        let outputs = value.outputs();
        let inner = value.inner();

        let dims = vec![inputs.len(), inner.len(), outputs.len()];
        let mut nodes = vec![];
        let mut edges = vec![];

        for (i, node) in inputs.iter().enumerate() {
            nodes.push(i);
            node_mappings.insert(node, i);
        }
        for (i, node) in inner.iter().enumerate() {
            nodes.push(i);
            node_mappings.insert(node, i);
        }
        for (i, node) in outputs.iter().enumerate() {
            nodes.push(i);
            node_mappings.insert(node, i);
        }

        for edge in value.edges() {
            let from = node_mappings[&edge.from];
            let to = node_mappings[&edge.to];

            let weight = edge.weight;

            edges.push((from, to, weight));
        }

        SimpleANN::new(&dims, &nodes, &edges)
    }
}

impl SimpleANN {
    pub fn new(dims: &[usize], nodes: &[usize], edges: &[(usize, usize, f32)]) -> Self {
        SimpleANN { dims: dims.into(), nodes: nodes.into(), edges: edges.into() }
    }

    pub fn forward<F: Into<f32>>(&self, inputs: &[F]) -> Result<Vec<f32>> {
        if inputs.len() != self.dims[0] {
            Err(AnnError::MismatchedInputSizeError(inputs.len(), self.dims[0]))
        } else {
            let mut state_table: HashMap<usize, f32> = HashMap::new();
            let mut res = vec![];

            for (from, to, weight) in self.edges.iter() {
                if let Some(val) = state_table.get(from) {
                    let prev = state_table.get(to).unwrap_or(&0f32);
                    state_table.insert(*to, prev + (val * weight));
                } else {
                    return Err(AnnError::UninitializedNodeVisitError);
                }
                
            }

            for i in (self.nodes.len() - self.dims[self.dims.len() - 1])..self.nodes.len() {
                res.push(*state_table.get(&i).unwrap_or(&0f32))
            }

            Ok(res)
        }
    }

}