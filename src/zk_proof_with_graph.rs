use ark_bn254::{Bn254, Fr};
use ark_groth16::Proof;
use crate::zk_proof::{Graph, MyZkProof, ProofValues, RLNZkProof, Witness, Zkey};

pub trait GraphEval<W> {
    fn eval_witness(&self, witness: &W) -> Result<Fr, String>;
}

pub trait RLNZkProofWithGraph: RLNZkProof {

    fn evaluate_witness(&self, witness: &Self::Witness) -> Result<Self::EvaluatedWitness, String>;

    fn generate_proof_from_witness(&self, witness: Self::Witness) -> (Self::Proof, Self::Values);
}

#[derive(Debug)]
pub struct MyZkProofGraphLess {
    zkey: Zkey,
}

impl MyZkProofGraphLess {
    pub fn new() -> Self {
        Self {
            zkey: Zkey {},
        }
    }
}

impl RLNZkProof for MyZkProofGraphLess {
    type Proof = Proof<Bn254>;
    type Values = ProofValues;
    type Witness = Witness;
    type EvaluatedWitness = Vec<Fr>;

    fn verify(&self) -> Result<bool, String> {
        todo!()
    }



    fn generate_proof_and_values(&self, witness: Self::Witness, evaluated_witness: Self::EvaluatedWitness) -> (Self::Proof, Self::Values) {
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

}
