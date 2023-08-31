///We can evaluate a polynomial at any number of points and prove it,
///using still only one group element.
use crate::kzg_tools::{polynomial::Polynomial, trusted_setup::TrustedSetup};
use halo2::halo2curves::{
    bn256::{pairing, Fr, G1Affine, G2Affine, G1, G2},
    group::Curve,
};

struct MultiProof {
    polynomial_commitment: G1Affine,
    quotient_commitment: G1Affine,
    interpolation_polynomial_commitment: G1Affine,
    zero_polynomial_commitment: G2Affine,
}

///Say we want to prove the evaluation of k points:
///(z_0, y_0), (z_1, y_1),..., (z_k-1, y_k-1).
///Using an interpolation polynomial "I(X)", we can find a polynomial
///of degree less than k, that goes through all these points.
///Using Lagrange interpolation, we can compute this interpolation polynomial.
///Since our original polynomial p(X) is passing through all these points,
///the polynomial p(X) - I(X) will be zero at each z_0, z_1, ... , z_k-1.
///In other words, this polynomial will be divisible by:
///(X - z_0), (X - z_1), ... , (X - z_k-1).
///Using the zero polynomial "Z(X)", we can again establish a similar relationship that we did before in "kzg":  
fn prover(p_committed: Polynomial, trusted_setup: TrustedSetup, k_points: u32) -> MultiProof {
    let eval_of_z_points = Polynomial::eval_of_z_points(&p_committed, k_points);
    let interpolation_polynomial =
        Polynomial::lagrange(&p_committed, eval_of_z_points.clone(), k_points);
    let mut zero_polynomial = Polynomial::new(vec![Fr::one()]);

    for i in 0..eval_of_z_points.0.coeff.len() {
        let zero_polynomial_values =
            Polynomial::new(vec![eval_of_z_points.0.coeff[i].neg(), Fr::one()]);
        zero_polynomial = zero_polynomial_values.mul_poly(zero_polynomial.coeff);
    }

    let mut zero_polynomial_commitment = G2::generator() * Fr::zero();

    for i in 0..zero_polynomial.coeff.len() {
        zero_polynomial_commitment += trusted_setup.s_g2[i] * zero_polynomial.coeff[i];
    }

    let quotient_polynomial = (p_committed.sub_poly(interpolation_polynomial.coeff.clone()))
        .div_poly(zero_polynomial.coeff)
        .0;

    let mut quotient_commitment = G1::generator() * Fr::zero();

    for i in 0..quotient_polynomial.coeff.len() {
        quotient_commitment += trusted_setup.s_g1[i] * quotient_polynomial.coeff[i];
    }

    let mut polynomial_commitment = G1::generator() * Fr::zero();

    for i in 0..p_committed.coeff.len() {
        polynomial_commitment += trusted_setup.s_g1[i] * p_committed.coeff[i];
    }

    let mut interpolation_polynomial_commitment = G1::generator() * Fr::zero();

    for i in 0..interpolation_polynomial.coeff.len() {
        interpolation_polynomial_commitment +=
            trusted_setup.s_g1[i] * interpolation_polynomial.coeff[i];
    }

    let polynomial_commitment_aff = polynomial_commitment.to_affine();
    let quotient_commitment_aff = quotient_commitment.to_affine();
    let zero_polynomial_commitment_aff = zero_polynomial_commitment.to_affine();
    let interpolation_polynomial_aff = interpolation_polynomial_commitment.to_affine();
    let proof = MultiProof {
        polynomial_commitment: polynomial_commitment_aff,
        quotient_commitment: quotient_commitment_aff,
        zero_polynomial_commitment: zero_polynomial_commitment_aff,
        interpolation_polynomial_commitment: interpolation_polynomial_aff,
    };
    proof
}

///The equation the verifier needs to check is: e(pi, [Z(s)]_2) = e(C - [I(s)]_1, H)
fn verifier(proof: MultiProof) -> bool {
    let poly_com_sub_int =
        (proof.polynomial_commitment - proof.interpolation_polynomial_commitment).to_affine();
    let h_aff = G2::generator().to_affine();
    let pair_left = pairing(
        &proof.quotient_commitment,
        &proof.zero_polynomial_commitment,
    );
    let pair_right = pairing(&poly_com_sub_int, &h_aff);
    pair_left == pair_right
}

#[cfg(test)]
mod tests {
    use crate::kzg_multiproof::{prover, verifier};
    use crate::kzg_tools::{polynomial::Polynomial, trusted_setup::trusted_setup};

    #[test]
    fn kzg_multiproof_test() {
        let p_committed = Polynomial::create_polynomial(7);
        let trusted_setup = trusted_setup(p_committed.clone());
        let prover = prover(p_committed, trusted_setup.clone(), 3);
        let verifier = verifier(prover);
        assert!(verifier);
    }
}
