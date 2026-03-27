#[allow(unused_variables)]

use std::str::FromStr;
use ark_bn254::Fr;
use crate::error::ZerokitMerkleTreeError;
use crate::hash::{FrOf, Hasher};

// Traits

pub trait ZerokitMerkleProof {
    type Index;
    // type Hasher: Hasher;

    fn length(&self) -> usize;
    fn leaf_index(&self) -> usize;
    fn get_path_elements(&self) -> Vec<Fr>;
    fn get_path_index(&self) -> Vec<Self::Index>;
    fn compute_root_from(&self, leaf: &Fr) -> Fr;
}

pub trait ZerokitMerkleTree {
    type Proof: ZerokitMerkleProof;
    // type Hasher: Hasher;
    type Config: Default + FromStr;

    fn default(depth: usize) -> Result<Self, ZerokitMerkleTreeError>
    where
        Self: Sized;
    fn new(
        depth: usize,
        default_leaf: Fr,
        config: Self::Config,
    ) -> Result<Self, ZerokitMerkleTreeError>
    where
        Self: Sized;
    fn depth(&self) -> usize;

    // And more functions here...
}

// Impl

#[derive(Debug)]
pub struct MyTree {}

impl ZerokitMerkleTree for MyTree {
    type Proof = MyTreeProof;
    // type Hasher = ();
    type Config = String;

    fn default(depth: usize) -> Result<Self, ZerokitMerkleTreeError>
    where
        Self: Sized
    {
        todo!()
    }

    fn new(depth: usize, default_leaf: Fr, config: Self::Config) -> Result<Self, ZerokitMerkleTreeError>
    where
        Self: Sized
    {
        todo!()
    }

    fn depth(&self) -> usize {
        20
    }
}

#[derive(Debug)]
pub struct MyTreeProof {}

impl ZerokitMerkleProof for MyTreeProof {
    type Index = u8;
    // type Hasher = ();

    fn length(&self) -> usize {
        todo!()
    }

    fn leaf_index(&self) -> usize {
        todo!()
    }

    fn get_path_elements(&self) -> Vec<Fr> {
        todo!()
    }

    fn get_path_index(&self) -> Vec<Self::Index> {
        todo!()
    }

    fn compute_root_from(&self, leaf: &Fr) -> Fr {
        todo!()
    }
}
