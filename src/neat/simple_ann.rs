use std::{collections::HashMap, fmt::Display};

use super::net::{ann::ANN, error::{AnnError, Result}};

#[derive(Clone)]
pub struct SimpleANN {
    pub(crate)dims: Vec<usize>,
    pub(crate)nodes: Vec<usize>,
    //From, To, Weight
    pub(crate)edges: Vec<(usize, usize, f32)>
}

impl Display for SimpleANN {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut curr_prev = 0;
        let mut curr_top = 0;
        for i in 0..self.dims.len() {
            write!(f, "\n----Layer #{}----\n", i)?;
            curr_top += self.dims[i];
            let mut edged = vec![];
            for (from, to, weight) in self.edges.iter()
            .filter(|(from, _, _)| *from < curr_top && *from >= curr_prev) {
                write!(f,"Node {} --({:.2})--> Node {}\n", from, weight, to)?;
                edged.push(*from);
            }
            for i in self.nodes.iter()
                .filter(|node| !edged.contains(*node) && *node < &curr_top && *node >= &curr_prev) {
                    write!(f, "Node {} -X-\n", i)?;
            }
            curr_prev += self.dims[i];
        }
        Ok(())
    }
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

    pub fn forward<F: Into<f32> + Copy>(&self, inputs: &[F]) -> Result<Vec<f32>> {
        if inputs.len() != self.dims[0] {
            Err(AnnError::MismatchedInputSizeError(inputs.len(), self.dims[0]))
        } else {
            let mut state_table = vec![f32::NAN; self.nodes.len()];
            let mut res = vec![];

            for (node, i) in inputs.iter().enumerate() {
                state_table.insert(node, (*i).into());
            }

            for (from, to, weight) in self.edges.iter() {
                if !state_table[*from].is_nan() {
                    let prev = state_table[*to];
                    state_table.insert(*to, prev + (state_table[*from] * weight));
                } else {
                    println!("Error at {} to {}", from, to);
                    return Err(AnnError::UninitializedNodeVisitError);
                }
                
            }

            for i in (self.nodes.len() - self.dims[self.dims.len() - 1])..self.nodes.len() {
                res.push(state_table[i]);
            }

            Ok(res)
        }
    }

}