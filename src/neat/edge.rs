use super::node::NodeId;

pub struct Edge {
    weight: f32,
    to: NodeId,
    from: NodeId
}