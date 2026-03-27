#![allow(unused_variables)]

use std::io::{Read, Write};
use ark_bn254::{Bn254, Fr};
use ark_groth16::Proof;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, Compress, SerializationError, Valid, Validate};
use crate::zk_proof::{Endianess, Graph, MyZkProof, ProofValues, RLNZkProof, Witness, Zkey};

pub trait RLNPartialZkProof: RLNZkProof {
    type PartialWitness: CanonicalSerialize + CanonicalDeserialize;
    // type RemainingWitness;
    type PartialProof: CanonicalSerialize + CanonicalDeserialize;

    fn generate_partial_proof(&self, witness: Self::PartialWitness) -> Self::PartialProof; // Result
    fn finish_proof(&self, partial: Self::PartialProof, remaining: Self::Witness) -> Self::Proof; // Result
}

#[derive(Debug)]
pub struct MyPartialZkProof {
    graph: crate::zk_proof::Graph,
    zkey: crate::zk_proof::Zkey,
}

impl MyPartialZkProof {
    pub fn new() -> Self {
        Self {
            graph: Graph {},
            zkey: Zkey {},
        }
    }
}

impl RLNZkProof for MyPartialZkProof {
    type Proof = Proof<Bn254>;
    type Values = ProofValues;
    type Witness = Witness;

    fn verify(&self) -> Result<bool, String> {
        todo!()
    }

    fn generate_proof() -> Self::Proof {
        todo!()
    }

    fn generate_proof_and_values(&self, witness: Self::Witness) -> (Self::Proof, Self::Values) {
        let proof_values = ProofValues::try_from(witness).unwrap();
        let proof = {

            /*
            let full_assignment = calc_witness(inputs, graph)?;

            let proof = Groth16::<_, CircomReduction>::create_proof_with_reduction_and_matrices(
                &zkey.0,
                r,
                s,
                &zkey.1,
                zkey.1.num_instance_variables,
                zkey.1.num_constraints,
                full_assignment.as_slice(),
            )?;
            */

            Self::Proof::default()
        };
        (proof, proof_values)
    }

    fn proof() -> Self::Proof {
        todo!()
    }

    fn values() -> Self::Values {
        todo!()
    }

    fn to_bytes(&self, values_endian: Endianess) -> Vec<u8> {
        todo!()
    }

    fn from_bytes(&self, values_endian: Endianess) -> Vec<u8> {
        todo!()
    }
}

impl RLNPartialZkProof for MyPartialZkProof {
    type PartialWitness = PartialWitness;
    type PartialProof = PartialProof;

    fn generate_partial_proof(&self, witness: Self::PartialWitness) -> Self::PartialProof {
        PartialProof {}
    }

    fn finish_proof(&self, partial: Self::PartialProof, remaining: Self::Witness) -> Self::Proof {
        todo!()
    }
}

#[derive(Debug)]
pub struct PartialWitness {
    pub identity_secret: Fr, // IdSecret
    pub user_message_limit: Fr,
    pub path_elements: Vec<Fr>,
    pub identity_path_index: Vec<u8>,
}

impl CanonicalSerialize for PartialWitness {
    fn serialize_with_mode<W: Write>(&self, writer: W, compress: Compress) -> Result<(), SerializationError> {
        todo!()
    }

    fn serialized_size(&self, compress: Compress) -> usize {
        todo!()
    }
}

impl Valid for PartialWitness {
    fn check(&self) -> Result<(), SerializationError> {
        todo!()
    }
}

impl CanonicalDeserialize for PartialWitness {
    fn deserialize_with_mode<R: Read>(reader: R, compress: Compress, validate: Validate) -> Result<Self, SerializationError> {
        todo!()
    }
}

#[derive(Debug)]
pub struct PartialProof;

impl CanonicalSerialize for PartialProof {
    fn serialize_with_mode<W: Write>(&self, writer: W, compress: Compress) -> Result<(), SerializationError> {
        todo!()
    }

    fn serialized_size(&self, compress: Compress) -> usize {
        todo!()
    }
}

impl Valid for PartialProof {
    fn check(&self) -> Result<(), SerializationError> {
        todo!()
    }
}

impl CanonicalDeserialize for PartialProof {
    fn deserialize_with_mode<R: Read>(reader: R, compress: Compress, validate: Validate) -> Result<Self, SerializationError> {
        todo!()
    }
}
