use slotmap::new_key_type;

use super::{activation::Activation, edge::Edge};

pub struct Node {
    activation: Option<Activation>,
    edges: Vec<Edge>
}

new_key_type! {
    pub struct NodeId;
}