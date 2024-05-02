use super::node::NodeId;

#[derive(Debug, Clone)]
pub struct Edge {
    pub(crate) weight: f32,
    pub(crate) to: NodeId,
}

impl Edge {
    pub fn new(to: NodeId) -> Self {
        Edge {
            to,
            weight: 1f32
        }
    }
}
