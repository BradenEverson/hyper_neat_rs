use std::collections::HashMap;

use super::net::{ann::ANN, node::NodeId};

pub struct SimpleANN {
    dims: Vec<u8>,
    nodes: Vec<u8>,
    edges: Vec<(u8, u8, f32)>
}

impl From<ANN> for SimpleANN {
    fn from(value: ANN) -> Self {
        let mut node_mappings = HashMap::new();

        let inputs = value.inputs();
        let outputs = value.outputs();
        let inner = value.inner();

        let dims = vec![inputs.len() as u8, inner.len() as u8, outputs.len() as u8];
        let mut nodes = vec![];
        let mut edges = vec![];

        for (i, node) in inputs.iter().enumerate() {
            nodes.push(i as u8);
            node_mappings.insert(node, i);
        }
        for (i, node) in inner.iter().enumerate() {
            nodes.push(i as u8);
            node_mappings.insert(node, i);
        }
        for (i, node) in outputs.iter().enumerate() {
            nodes.push(i as u8);
            node_mappings.insert(node, i);
        }

        for edge in value.edges() {
            let from = node_mappings[&edge.from];
            let to = node_mappings[&edge.to];

            let weight = edge.weight;

            edges.push((from as u8, to as u8, weight));
        }

        SimpleANN::new(&dims, &nodes, &edges)
    }
}

impl Into<ANN> for SimpleANN {
    fn into(self) -> ANN {
        let inputs = self.dims[0];
        let outputs = self.dims[self.dims.len() - 1];

        ANN::new()
            .with_inputs(inputs as usize)
            .and_outputs(outputs as usize)
    }
}

impl SimpleANN {
    pub fn new(dims: &[u8], nodes: &[u8], edges: &[(u8, u8, f32)]) -> Self {
        SimpleANN { dims: dims.into(), nodes: nodes.into(), edges: edges.into() }
    }
}