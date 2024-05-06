use super::node::Node;


#[derive(thiserror::Error, Debug)]
pub enum AnnError {
    #[error("Node {0} cannot be connected to node {1}")]
    InvalidConnectionError(Node, Node),
    #[error("{0} cannot be connected to itself")]
    RecursiveConnectionError(Node),
    #[error("Node does not exist in ANN")]
    InvalidNodeIDError,
    #[error("Inputs provided do not match number of input nodes ({0} vs {1})")]
    MismatchedInputSizeError(usize, usize),
    #[error("Uninitialized node visited before initialization")]
    UninitializedNodeVisitError


}

pub type Result<T> = std::result::Result<T, AnnError>;
