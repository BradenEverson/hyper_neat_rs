use crate::neat::net::{ann::ANN, edge::Edge, node::NodeId};

pub struct ProblemSet {
    population: Vec<NeatNet>,
    fitness: Box<dyn Fn(ANN) -> f32>
}

pub struct NeatNet {
    genome: ANN,
    nodes: Vec<NodeId>,
    edges: Vec<Edge>
}

impl Default for NeatNet {
    fn default() -> Self {
        NeatNet::empty()
    }
}

impl NeatNet {
    pub fn empty() -> Self {
        NeatNet { genome: ANN::new(), nodes: vec![], edges: vec![] }
    }



}