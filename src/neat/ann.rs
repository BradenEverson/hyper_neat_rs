use slotmap::SlotMap;

use super::node::{Node, NodeId};

pub struct ANN {
    nodes: SlotMap<NodeId, Node>,
}

