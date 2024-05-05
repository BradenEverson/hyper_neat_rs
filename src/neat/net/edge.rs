use super::node::NodeId;

#[derive(Debug, Clone)]
pub struct Edge {
    pub(crate) weight: f32,
    pub(crate) to: NodeId,
    pub(crate) from: NodeId,
}

impl Edge {
    pub fn new(to: NodeId, from: NodeId) -> Self {
        Edge {
            to,
            from,
            weight: 1f32
        }
    }
    pub fn update_weight<F: Into<f32>>(&mut self, new_weight: F) {
        self.weight = new_weight.into()
    }
}
