use super::node::NodeId;

#[derive(Debug, Clone)]
pub struct Edge {
    pub(crate) weight: f32,
    pub(crate) to: NodeId,
    pub(crate) from: NodeId
}

impl Edge {
    pub fn new(from: NodeId, to: NodeId) -> Self {
        Edge {
            from, 
            to,
            weight: 0f32
        }
    }
}
