use slotmap::new_key_type;

use super::{activation::Activation, edge::Edge};

pub struct Node {
    activation: Option<Activation>,
    edges: Vec<Edge>
}

impl Default for Node {
    fn default() -> Self {
        Self::empty()
    }
}

impl Node {
    pub fn empty() -> Self {
        Node { activation: None, edges: vec![] }
    }
}

new_key_type! {
    pub struct NodeId;
}