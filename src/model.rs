use crate::neat::net::{ann::ANN, edge::Edge, node::NodeId};

pub struct ProblemSet {
    population: Vec<NeatNet>
}

pub struct NeatNet {
    genome: ANN,
    nodes: Vec<NodeId>,
    edges: Vec<Edge>
}

impl NeatNet {

}