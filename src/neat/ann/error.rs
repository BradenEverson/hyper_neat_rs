use super::node::Node;


#[derive(thiserror::Error, Debug)]
pub enum AnnError {
    #[error("Node {0} cannot be connected to node {1}")]
    InvalidConnectionError(Node, Node),
    #[error("{0} cannot be connected to itself")]
    RecursiveConnectionError(Node),
    #[error("Node does not exist in ANN")]
    InvalidNodeIDError
}

pub type Result<T> = std::result::Result<T, AnnError>;
