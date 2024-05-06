use std::collections::HashMap;

use super::net::{ann::ANN, error::{AnnError, Result}};

pub struct SimpleANN {
    dims: Vec<usize>,
    nodes: Vec<usize>,
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

    pub fn forward(&self, inputs: &[f32]) -> Result<Vec<f32>> {
        if inputs.len() != self.dims[0] {
            Err(AnnError::MismatchedInputSizeError(inputs.len(), self.dims[0]))
        } else {
            Ok(vec![])
        }
    }

}