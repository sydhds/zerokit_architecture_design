use std::fmt::{Debug, Display};
use std::str::FromStr;

pub trait Hasher {
    /// Type of the leaf and tree node
    type Fr: Clone + Copy + Eq + Default + Debug + Display + FromStr + Send + Sync;

    // Note: not sure what's the idea behind this method
    // /// Returns the default tree leaf
    // fn default_leaf() -> Self::Fr;

    /// Utility to compute the hash of an intermediate node
    fn hash(input: &[Self::Fr]) -> Self::Fr;
}
pub type FrOf<H> = <H as Hasher>::Fr;


