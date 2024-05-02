use std::fmt::Display;

use slotmap::new_key_type;

use super::{activation::Activation, edge::Edge};

#[derive(Debug, Clone)]
pub struct Node {
    pub(crate) ty: NodeType,
    pub(crate) activation: Option<Activation>,
    pub(crate) edges: Vec<Edge>
}

#[derive(Debug, Clone, PartialEq)]
pub enum NodeType {
    Input,
    Output,
    Inner
}

impl Display for NodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Input => "Input",
            Self::Inner => "Inner",
            Self::Output => "Output"
        })
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} Node with {} edges)", self.ty, self.edges.len())
    }
}

impl Default for Node {
    fn default() -> Self {
        Self::inner()
    }
}

impl Node {
    pub fn inner() -> Self {
        Node { activation: None, edges: vec![], ty: NodeType::Inner }
    }
    pub fn input() -> Self {
        Node { activation: None, edges: vec![], ty: NodeType::Input }
    }
    pub fn output() -> Self {
        Node { activation: None, edges: vec![], ty: NodeType::Output }
    }

    pub fn with_activation(mut self, activation: Activation) -> Self {
        self.activation = Some(activation);
        self
    }
}

new_key_type! {
    pub struct NodeId;
}
