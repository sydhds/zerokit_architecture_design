#[derive(thiserror::Error, Debug)]
pub enum ZerokitMerkleTreeError {
    #[error("Invalid index")]
    InvalidIndex,
    // InvalidProof,
    #[error("Leaf index out of bounds")]
    InvalidLeaf,
    #[error("Level exceeds tree depth")]
    InvalidLevel,
    #[error("Subtree index out of bounds")]
    InvalidSubTreeIndex,
    #[error("Start level is != from end level")]
    InvalidStartAndEndLevel,
    #[error("set_range got too many leaves")]
    TooManySet,
    #[error("Unknown error while computing merkle proof")]
    ComputingProofError,
    #[error("Invalid witness length (!= tree depth)")]
    InvalidWitness,
    // #[cfg(feature = "pmtree-ft")]
    // #[error("Pmtree error: {0}")]
    // PmtreeErrorKind(#[from] pmtree::PmtreeErrorKind),
}