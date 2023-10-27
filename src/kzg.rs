/// This scheme is often also called Kate Polynomial Commitment Scheme.
/// As a polynomial commitment scheme, it allows a prover
/// to compute a commitment to a polynomial, with the properties that this
/// commitment can later be opened at any position: The prover shows that
/// the value of the polynomial at a certain position is equal to a claimed value.
/// I made use of "KZG polynomial commitments" from
/// https://dankradfeist.de/ethereum/2020/06/16/kate-polynomial-commitments.html"
use crate::kzg_tools::{
    evaluation::Evaluation, polynomial::Polynomial, trusted_setup::TrustedSetup,
};
use halo2::halo2curves::{
    bn256::{pairing, Fr, G1Affine, G1, G2},
    group::Curve,
};

struct Proof {
    polynomial_commitment: G1Affine,
    quotient_commitment: G1Affine,
    y: Evaluation,
}

/// The prover first needs to commit to the polynomial p(X) and then submit
/// a proof pi(π), along with his claim p(z) = y. The point z will be selected
/// by the verifier, and sent to the prover after the prover sends his commitment
/// C = [p(s)], which is a commitment to the polynomial p(X).
fn prover(polynomial_degree: u32, z: Fr, trusted_setup: TrustedSetup) -> Proof {
    let p_committed = Polynomial::create_polynomial(polynomial_degree);
    let y = Polynomial::eval(&p_committed, z);
    let mut num: Polynomial = Polynomial::new(Vec::new());
    let num_sub = p_committed.coeff[0] - (y.evaluation);
    num.coeff.push(num_sub);

    for i in p_committed.coeff.iter().skip(1) {
        num.coeff.push(*i);
    }
    let mut den = Vec::new();
    den.push(z.neg());
    den.push(Fr::one());
    let q_x = Polynomial::div_poly(&mut num, den).0;
    let mut pi = G1::generator() * Fr::zero();

    for i in 0..q_x.coeff.len() {
        pi += trusted_setup.s_g1[i] * q_x.coeff[i];
    }
    let mut polynomial_commitment = G1::generator() * Fr::zero();

    for i in 0..p_committed.coeff.len() {
        polynomial_commitment += trusted_setup.s_g1[i] * p_committed.coeff[i];
    }
    let polynomial_commitment_aff = polynomial_commitment.to_affine();
    let pi_aff = pi.to_affine();
    let proof = Proof {
        polynomial_commitment: polynomial_commitment_aff,
        quotient_commitment: pi_aff,
        y,
    };
    proof
}

/// The verifier checks the equation: (pi, [s - z]_2) = e(C - [y]_1, H)
/// with pairing and if the equation holds, the verifier accepts the proof.
fn verifier(proof: Proof, z: Fr, trusted_setup: TrustedSetup) -> bool {
    let y_g1_aff = (G1::generator() * proof.y.evaluation).to_affine();
    let c_y = (proof.polynomial_commitment - y_g1_aff).to_affine();
    let z_g2 = G2::generator() * z;
    let s_z = trusted_setup.s_g2[1] - z_g2;
    let s_z_aff = s_z.to_affine();
    let h_aff = G2::generator().to_affine();
    let pair_left = pairing(&proof.quotient_commitment, &s_z_aff);
    let pair_right = pairing(&c_y, &h_aff);
    pair_left == pair_right
}

#[cfg(test)]
mod tests {
    use crate::kzg::{prover, verifier, Fr};
    use crate::kzg_tools::trusted_setup::trusted_setup;
    use halo2::arithmetic::Field;
    use rand::thread_rng;

    #[test]
    fn kzg_test() {
        // Create a polynomial with given degree.
        let polynomial_degree = 7;
        let rng = thread_rng();
        let z = Fr::random(rng.clone());
        let trusted_setup = trusted_setup(polynomial_degree);
        let prover = prover(polynomial_degree, z, trusted_setup.clone());
        let verifier = verifier(prover, z, trusted_setup);
        assert!(verifier);
    }
}
