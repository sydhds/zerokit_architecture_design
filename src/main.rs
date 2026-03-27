mod error;
mod tree;
mod hash;
mod se_de;
mod zk_proof;
mod qap;
mod partial_zk_proof;

use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::marker::PhantomData;
use std::str::FromStr;
use ark_bn254::Fr;
use ark_serialize::{CanonicalSerialize, Compress, CanonicalDeserialize};
use crate::error::ZerokitMerkleTreeError;
use crate::partial_zk_proof::{MyPartialZkProof, PartialWitness, RLNPartialZkProof};
use crate::tree::{ZerokitMerkleTree, ZerokitMerkleProof, MyTree, MyTreeProof};
use crate::zk_proof::{MyZkProof, RLNZkProof, RecoverSecret2, Witness, WitnessSingle};

/*
pub trait WitnessCompute {
    fn calc_witness(&self, inputs: HashMap<String, Vec<Fr>>) -> Result<Vec<Fr>, String>;
    fn tree_depth(&self) -> usize;
    fn max_out(&self) -> usize;
}
*/



// End traits

/*
#[derive(Debug)]
pub enum TreeState<T> {
    Stateless,
    Active(T)
}
*/

#[derive(Debug)]
pub struct Stateless;

#[derive(Debug)]
pub struct Stateful<T: ZerokitMerkleTree> {
    pub tree: T,
}

#[derive(Debug)]
struct RLN<Mode, ZKP> {
    mode: Mode,
    zk_proof: ZKP,
}

impl<Mode, ZKP: RLNZkProof> RLN<Mode, ZKP> {

    // Common methods for Stateless & Stateful
    fn generate_proof(&self, witness: ZKP::Witness) -> (ZKP::Proof, ZKP::Values) {
        self.zk_proof.generate_proof_and_values(witness)
    }
}

impl<Mode, ZKP: RLNPartialZkProof> RLN<Mode, ZKP> {

    // Partial methods

    fn generate_partial_proof(&self, partial_witness: ZKP::PartialWitness) -> ZKP::PartialProof {
        self.zk_proof.generate_partial_proof(partial_witness)
    }

    fn finish_proof(&self, partial_proof: ZKP::PartialProof, witness: ZKP::Witness) -> ZKP::Proof  {
        self.zk_proof.finish_proof(partial_proof, witness)
    }

}

impl<ZKP> RLN<Stateless, ZKP> {
    fn new_stateless(zk_1: ZKP) -> Self {
        Self {
            mode: Stateless,
            zk_proof: zk_1,
        }
    }
}

impl<T: ZerokitMerkleTree, ZKP> RLN<Stateful<T>, ZKP> {
    fn new(tree: T, zk_1: ZKP) -> Self {
        Self {
            mode: Stateful { tree },
            zk_proof: zk_1,
        }
    }

    // Tree methods
    fn tree_depth(&self) -> usize {
        self.mode.tree.depth()
    }
}


fn main() {
    // println!("Hello, world!");

    // Ser / De test
    /*
    {
        let fr = Fr::from(1);
        let mut fr_ser = Vec::with_capacity(fr.serialized_size(Compress::Yes));
        fr.serialize_compressed(&mut fr_ser).unwrap();

        let fr_2 = vec![Fr::from(42), Fr::from(43)];
        let mut fr_2_ser = Vec::with_capacity(fr_2.serialized_size(Compress::Yes));
        fr_2.serialize_compressed(&mut fr_2_ser).unwrap();
        println!("fr_2_ser: {:?}", fr_2_ser);
        let fr_2_de = <Vec<Fr>>::deserialize_compressed(fr_2_ser.as_slice()).unwrap();
        assert_eq!(fr_2, fr_2_de);
    }
    */

    // Stateful (aka non-stateless)

    let r1: RLN<Stateful<MyTree>, MyZkProof> = RLN::new(MyTree {}, MyZkProof::new() );

    println!("RLN 1:");
    println!("{:#?}", r1);
    println!("tree depth: {:#?}", r1.tree_depth());

    // TODO: create a builder here to show a nice API
    let witness = Witness::Single(WitnessSingle {
        identity_secret: Fr::from(0), // Should be IdSecret
        user_message_limit: Fr::from(10),
        path_elements: vec![],
        identity_path_index: vec![],
        x: Fr::from(42),
        external_nullifier: Fr::from(42),
        message_id: Fr::from(0),
    });

    let (p1, pv1) = r1.generate_proof(witness.clone());
    println!("gen proof: {:?} --- {:?}", p1, pv1);
    let (p2, pv2) = r1.generate_proof(witness.clone());
    let id_secret = pv1.recover_secret(&pv2);
    println!("Recovered secret from pv1 & pv2: {:?}", id_secret);

    println!("{}", "#".repeat(8));

    // Stateful + partial proof support

    let r1_2: RLN<Stateful<MyTree>, MyPartialZkProof> = RLN::new(MyTree {}, MyPartialZkProof::new() );

    println!("RLN 1_2:");
    println!("{:#?}", r1_2);
    println!("tree depth: {:#?}", r1_2.tree_depth());

    // stateless
    let r2: RLN<Stateless, MyZkProof> = RLN::new_stateless( MyZkProof::new() );

    println!("RLN 2:");
    println!("{:#?}", r2);
    println!("gen proof - stateless: {:#?}", r2.generate_proof(witness.clone()));
    println!("{}", "#".repeat(8));

    // stateless + partial proof support
    let r2_2: RLN<Stateless, MyPartialZkProof> = RLN::new_stateless( MyPartialZkProof::new() );

    let partial_witness = PartialWitness {
        identity_secret: Fr::from(0),
        user_message_limit: Fr::from(10),
        path_elements: vec![],
        identity_path_index: vec![],
    };

    println!("RLN 2_2:");
    println!("{:#?}", r2_2);
    println!("gen partial proof - stateless: {:#?}", r2_2.generate_partial_proof(partial_witness));
    println!("{}", "#".repeat(8));
}
