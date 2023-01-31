#![doc = include_str!("../README.md")]

use ark_ff::Field;
use ark_relations::{
    lc,
    r1cs::{ConstraintSynthesizer, ConstraintSystemRef, Result as R1CSResult, Variable},
};

struct AddTwoValueCircuit<F: Field> {
    a: F,
    b: F,
    out: F,
}

impl<F: Field> ConstraintSynthesizer<F> for AddTwoValueCircuit<F> {
    fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> R1CSResult<()> {
        let var_a = cs.new_witness_variable(|| Ok(self.a)).unwrap();
        let var_b = cs.new_witness_variable(|| Ok(self.b)).unwrap();
        let var_out = cs.new_input_variable(|| Ok(self.out)).unwrap();

        // A * B = C
        cs.enforce_constraint(lc!() + var_a + var_b, lc!(), lc!() + var_out)?;

        Ok(())
    }
}

pub fn assign_witness() -> R1CSResult<()> {
    Ok(())
}

pub fn prove() -> bool {
    // trusted setup (Preprocessing)
    //   -> CRS (prover parameter, verifier parameter)

    // circuit
    // a + b * c = d

    // calculate proof
    // prove
    // 1.
    // 2.
    // 3.

    // verify proof

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = prove();
        assert!(result);
    }
}
