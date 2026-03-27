
/*
pub trait RlnSerialize: Sized {
    fn to_bytes_le(&self) -> Vec<u8>;
    fn to_bytes_be(&self) -> Vec<u8>;
    fn from_bytes_le(input: &[u8]) -> Result<(Self, usize), String>;
    fn from_bytes_be(input: &[u8]) -> Result<(Self, usize), String>;
}
*/

use ark_serialize::SerializationError;
use std::io::{Read, Write};
use ark_bn254::Fr;
use ark_ff::{BigInteger, BigInt, PrimeField};
use num_bigint::BigUint;

pub trait CanonicalSerializeBE {
    fn serialize_be<W: Write>(&self, writer: W) -> Result<(), SerializationError>;
    fn serialized_size_be(&self) -> usize;
}

pub trait CanonicalDeserializeBE: Sized {
    fn deserialize_be<R: Read>(reader: R) -> Result<Self, SerializationError>;
}

impl CanonicalSerializeBE for Fr {
    fn serialize_be<W: Write>(&self, mut writer: W) -> Result<(), SerializationError> {
        let bigint = self.into_bigint();
        let be_bytes = bigint.to_bytes_be();
        writer.write_all(&be_bytes).map_err(SerializationError::IoError)?;
        Ok(())
    }

    fn serialized_size_be(&self) -> usize {
        // Calculate the bytes needed (e.g., 254 bits / 8 = 32 bytes)
        (Fr::MODULUS_BIT_SIZE as usize).div_ceil(8)
    }
}

impl CanonicalDeserializeBE for Fr {
    fn deserialize_be<R: Read>(mut reader: R) -> Result<Self, SerializationError> {
        let size = (Fr::MODULUS_BIT_SIZE as usize).div_ceil(8);
        let mut be_bytes = vec![0u8; size];
        reader.read_exact(&mut be_bytes).map_err(SerializationError::IoError)?;

        let biguint = BigUint::from_bytes_be(&be_bytes);
        let modulus_bytes = Fr::MODULUS.to_bytes_be();
        let modulus_biguint = BigUint::from_bytes_be(&modulus_bytes);

        // Fr::from(BigUint) performs a modulo reduction if the bytes represent a number
        // larger than Fr::MODULUS
        // this can lead to malleability bug (multiple bytes can produce the same Fr)
        if biguint >= modulus_biguint {
            return Err(SerializationError::InvalidData);
        }

        Ok(Fr::from(biguint))
    }
}