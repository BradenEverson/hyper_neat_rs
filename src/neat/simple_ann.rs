use std::{collections::HashMap};

use super::{net::{ann::ANN, error::{AnnError, Result}}, simple_edge::SimpleEdge};

#[derive(Clone)]
pub struct SimpleANN {
    pub(crate)dims: Vec<usize>,
    pub(crate)nodes: Vec<usize>,
    //From, To, Weight
    pub(crate)edges: Vec<SimpleEdge>
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
            nodes.push(i + dims[0]);
            node_mappings.insert(node, i + dims[0]);
        }
        for (i, node) in outputs.iter().enumerate() {
            nodes.push(i + dims[0] + inner.len());
            node_mappings.insert(node, i + dims[0] + inner.len());
        }

        for (i, edge) in value.edges().iter().enumerate() {
            let from = node_mappings[&edge.from];
            let to = node_mappings[&edge.to];

            let weight = edge.weight;

            edges.push((from, to, weight).into());
        }

        SimpleANN::new(&dims, &nodes, &edges)
    }
}


impl SimpleANN {
    pub fn new(dims: &[usize], nodes: &[usize], edges: &[(usize, usize, f32)]) -> Self {
        let mut simple_edges: Vec<SimpleEdge> = vec![];

        for edge in edges.iter() {
            simple_edges.push(SimpleEdge::from(*edge))
        }

        SimpleANN { dims: dims.into(), nodes: nodes.into(), edges: simple_edges }
    }

    pub fn insert(&mut self, edge: (usize, usize, f32)) {
        todo!();
    }

    pub fn forward<F: Into<f32> + Copy>(&self, inputs: &[F]) -> Result<Vec<f32>> {
        if inputs.len() != self.dims[0] {
            Err(AnnError::MismatchedInputSizeError(inputs.len(), self.dims[0]))
        } else {
            let mut state_table = vec![f32::NAN; self.nodes.len()];
            let mut res = vec![];

            for (node, i) in inputs.iter().enumerate() {
                state_table[node] = (*i).into();
            }

            for edge in self.edges.iter() {
                let (_, from, to, weight) = Into::<(usize, usize, usize, f32)>::into(*edge);
                if !state_table[from].is_nan() {
                    let prev = match state_table[to].is_nan() {
                        true => 0f32,
                        false => state_table[to]
                    };
                    state_table[to] = prev + (state_table[from] * weight);
                } else {
                    println!("Error at {} to {}", from, to);
                    return Err(AnnError::UninitializedNodeVisitError);
                }
                
            }
            println!("{:?}", state_table);

            for elem in state_table.iter()
                .take(self.nodes.len()).skip(self.nodes.len() - self.dims[self.dims.len() - 1]) {

                res.push(*elem);
            }

            Ok(res)
        }
    }

}