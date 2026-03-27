#![allow(unused_variables)]

use std::io::{Read, Write};
use ark_bn254::{Bn254, Fr};
use ark_groth16::{Groth16, Proof};
use ark_serialize::{CanonicalSerialize, CanonicalDeserialize, Compress, SerializationError, Validate, Valid};
use crate::se_de::{CanonicalDeserializeBE, CanonicalSerializeBE};

/*
pub trait RecoverSecret {
    fn recover_secret(&self) -> Fr; // Should be IdSecret
}
*/

/*
pub trait RecoverSecret {
    type ProofValues_1;
    type ProofValues_2;
    type Error;

    fn recover_secret(proof_values_1: &Self::ProofValues_1, proof_values_2: &Self::ProofValues_2) -> Result<Fr, Self::Error>;
}
*/

pub trait RecoverSecret2<Rhs = Self> {
    type Error;

    fn recover_secret(&self, other: &Rhs) -> Result<Fr, Self::Error>;
}

/*
pub trait SerializeZk {
    fn serialize(&self, endian = LE/BE/MIX) -> Vec<u8>;
}
*/

pub enum Endianess {
    LE, // Little
    BE, // Big
    NE, // Native
}

pub trait RLNZkProof {

    type Witness: CanonicalSerialize + CanonicalDeserialize + CanonicalSerializeBE + CanonicalDeserializeBE;

    type Proof: CanonicalSerialize + CanonicalDeserialize;
    type Values: RecoverSecret2 + CanonicalSerialize + CanonicalDeserialize + TryFrom<Self::Witness> + CanonicalSerializeBE + CanonicalDeserializeBE;

    fn verify(&self) -> Result<bool, String>;

    fn generate_proof() -> Self::Proof;
    fn generate_proof_and_values(&self, wtiness: Self::Witness) -> (Self::Proof, Self::Values);

    fn proof() -> Self::Proof;

    fn values() -> Self::Values;

    // For Waku / ...
    fn to_bytes(&self, values_endian: Endianess) -> Vec<u8>;
    fn from_bytes(&self, values_endian: Endianess) -> Vec<u8>;
}

// impl

#[derive(Debug)]
pub(crate) struct Graph {}

#[derive(Debug)]
pub(crate) struct Zkey {}

#[derive(Debug)]
pub struct MyZkProof {
    graph: Graph,
    zkey: Zkey,
}

impl MyZkProof {
    pub fn new() -> Self {
        Self {
            graph: Graph {},
            zkey: Zkey {},
        }
    }
}

impl RLNZkProof for MyZkProof {
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


#[derive(Debug)]
pub struct ProofValuesSingle {
    root: Fr,
    x: Fr,
    external_nullifier: Fr,
    y: Fr,
    nullifier: Fr
}

impl Valid for ProofValuesSingle {
    fn check(&self) -> Result<(), SerializationError> {
        todo!()
    }
}

impl CanonicalSerialize for ProofValuesSingle {
    fn serialize_with_mode<W: Write>(&self, writer: W, compress: Compress) -> Result<(), SerializationError> {
        todo!()
    }

    fn serialized_size(&self, compress: Compress) -> usize {
        todo!()
    }
}

impl CanonicalDeserialize for ProofValuesSingle {
    fn deserialize_with_mode<R: Read>(reader: R, compress: Compress, validate: Validate) -> Result<Self, SerializationError> {
        todo!()
    }
}

impl TryFrom<WitnessSingle> for ProofValuesSingle {
    type Error = String;

    fn try_from(value: WitnessSingle) -> Result<Self, Self::Error> {

        Ok(Self {
            root: Default::default(),
            x: Default::default(),
            external_nullifier: Default::default(),
            y: Default::default(),
            nullifier: Default::default(),
        })
    }
}

/*
impl RecoverSecret for ProofValuesSingle {
    type ProofValues_1 = Self;
    type ProofValues_2 = Self;
    type Error = String;

    fn recover_secret(proof_values_1: &Self::ProofValues_1, proof_values_2: &Self::ProofValues_2) -> Result<Fr, Self::Error> {
        todo!()
    }
}
*/

impl RecoverSecret2 for ProofValuesSingle {
    type Error = String;

    fn recover_secret(&self, other: &Self) -> Result<Fr, Self::Error> {
        // Dummy impl to make the example in main.rs works
        Ok(Fr::from(999))
    }
}

#[derive(Debug)]
pub struct ProofValuesMulti {}

impl Valid for ProofValuesMulti {
    fn check(&self) -> Result<(), SerializationError> {
        todo!()
    }
}

impl CanonicalSerialize for ProofValuesMulti {
    fn serialize_with_mode<W: Write>(&self, writer: W, compress: Compress) -> Result<(), SerializationError> {
        todo!()
    }

    fn serialized_size(&self, compress: Compress) -> usize {
        todo!()
    }
}

impl CanonicalDeserialize for ProofValuesMulti {
    fn deserialize_with_mode<R: Read>(reader: R, compress: Compress, validate: Validate) -> Result<Self, SerializationError> {
        todo!()
    }
}

impl RecoverSecret2 for ProofValuesMulti {
    type Error = String;

    fn recover_secret(&self, other: &Self) -> Result<Fr, Self::Error> {
        todo!()
    }
}

impl RecoverSecret2<ProofValuesSingle> for ProofValuesMulti {
    type Error = String;
    fn recover_secret(&self, other: &ProofValuesSingle) -> Result<Fr, Self::Error> {
        todo!()
    }
}

#[derive(Debug)]
pub enum ProofValues {
    Single(ProofValuesSingle),
    Multi(ProofValuesMulti),
}

impl RecoverSecret2 for ProofValues {
    type Error = String;

    fn recover_secret(&self, other: &Self) -> Result<Fr, Self::Error> {
        match (self, other) {
            (ProofValues::Single(s1), ProofValues::Single(s2)) => s1.recover_secret(s2),
            (ProofValues::Single(s1), ProofValues::Multi(s2)) => s2.recover_secret(s1),
            (ProofValues::Multi(s1), ProofValues::Single(s2)) => s1.recover_secret(s2),
            (ProofValues::Multi(s1), ProofValues::Multi(s2)) => s1.recover_secret(s2),
        }
    }
}

impl CanonicalSerialize for ProofValues {
    fn serialize_with_mode<W: Write>(&self, writer: W, compress: Compress) -> Result<(), SerializationError> {
        todo!()
    }

    fn serialized_size(&self, compress: Compress) -> usize {
        todo!()
    }
}

impl Valid for ProofValues {
    fn check(&self) -> Result<(), SerializationError> {
        todo!()
    }
}

impl CanonicalDeserialize for ProofValues {
    fn deserialize_with_mode<R: Read>(reader: R, compress: Compress, validate: Validate) -> Result<Self, SerializationError> {
        todo!()
    }
}

impl CanonicalSerializeBE for ProofValues {
    fn serialize_be<W: Write>(&self, writer: W) -> Result<(), SerializationError> {
        todo!()
    }

    fn serialized_size_be(&self) -> usize {
        todo!()
    }
}

impl CanonicalDeserializeBE for ProofValues {
    fn deserialize_be<R: Read>(reader: R) -> Result<Self, SerializationError> {
        todo!()
    }
}

impl TryFrom<Witness> for ProofValues {
    type Error = String;
    fn try_from(value: Witness) -> Result<Self, Self::Error> {
        match value {
            Witness::Single(ws) => Ok(ProofValues::Single(ProofValuesSingle::try_from(ws)?)),
            Witness::Multi(wm) => unimplemented!()
        }
    }
}

#[derive(CanonicalSerialize, CanonicalDeserialize, Debug, Clone)]
pub struct WitnessSingle {
    pub identity_secret: Fr, // Should be IdSecret
    pub user_message_limit: Fr,
    pub path_elements: Vec<Fr>,
    pub identity_path_index: Vec<u8>,
    pub x: Fr,
    pub external_nullifier: Fr,
    pub message_id: Fr,
}

impl CanonicalSerializeBE for WitnessSingle {
    fn serialize_be<W: Write>(&self, writer: W) -> Result<(), SerializationError> {
        todo!()
    }

    fn serialized_size_be(&self) -> usize {
        todo!()
    }
}

impl CanonicalDeserializeBE for WitnessSingle {
    fn deserialize_be<R: Read>(reader: R) -> Result<Self, SerializationError> {
        todo!()
    }
}

#[derive(CanonicalSerialize, CanonicalDeserialize, Debug, Clone)]
pub struct WitnessMulti {
    identity_secret: Fr, // Should be IdSecret
    user_message_limit: Fr,
    path_elements: Vec<Fr>,
    identity_path_index: Vec<u8>,
    x: Fr,
    external_nullifier: Fr,
    message_ids: Vec<Fr>,
}

impl CanonicalSerializeBE for WitnessMulti {
    fn serialize_be<W: Write>(&self, writer: W) -> Result<(), SerializationError> {
        todo!()
    }

    fn serialized_size_be(&self) -> usize {
        todo!()
    }
}

impl CanonicalDeserializeBE for WitnessMulti {
    fn deserialize_be<R: Read>(reader: R) -> Result<Self, SerializationError> {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub enum Witness {
    Single(WitnessSingle),
    Multi(WitnessMulti),
}

impl Valid for Witness {
    fn check(&self) -> Result<(), SerializationError> {
        match self {
            Witness::Single(w) => w.check(),
            Witness::Multi(w) => w.check(),
        }
    }
}

// 2. Implement Serialize
impl CanonicalSerialize for Witness {
    fn serialize_with_mode<W: Write>(
        &self,
        mut writer: W,
        compress: Compress,
    ) -> Result<(), SerializationError> {
        match self {
            Witness::Single(w) => {
                // Write tag 0 for Single
                0u8.serialize_with_mode(&mut writer, compress)?;
                w.serialize_with_mode(writer, compress)
            }
            Witness::Multi(w) => {
                // Write tag 1 for Multi
                1u8.serialize_with_mode(&mut writer, compress)?;
                w.serialize_with_mode(writer, compress)
            }
        }
    }

    fn serialized_size(&self, compress: Compress) -> usize {
        // 1 byte for the tag + the size of the inner struct
        1 + match self {
            Witness::Single(w) => w.serialized_size(compress),
            Witness::Multi(w) => w.serialized_size(compress),
        }
    }
}

// 3. Implement Deserialize
impl CanonicalDeserialize for Witness {
    fn deserialize_with_mode<R: Read>(
        mut reader: R,
        compress: Compress,
        validate: Validate,
    ) -> Result<Self, SerializationError> {
        // Read the first byte to determine the variant
        let tag = u8::deserialize_with_mode(&mut reader, compress, validate)?;

        match tag {
            0 => {
                let inner = WitnessSingle::deserialize_with_mode(reader, compress, validate)?;
                Ok(Witness::Single(inner))
            }
            1 => {
                let inner = WitnessMulti::deserialize_with_mode(reader, compress, validate)?;
                Ok(Witness::Multi(inner))
            }
            _ => Err(SerializationError::InvalidData), // Fails if tag is not 0 or 1
        }
    }
}

impl CanonicalSerializeBE for Witness {
    fn serialize_be<W: Write>(&self, writer: W) -> Result<(), SerializationError> {
        todo!()
    }

    fn serialized_size_be(&self) -> usize {
        todo!()
    }
}

impl CanonicalDeserializeBE for Witness {
    fn deserialize_be<R: Read>(reader: R) -> Result<Self, SerializationError> {
        todo!()
    }
}



