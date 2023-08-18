use crate::kzg_tools::{
    barycentric::barycentric,
    polynomial::{div_poly, Polynomial},
    trusted_setup::TrustedSetup,
};
use halo2::{
    arithmetic::Field,
    halo2curves::{
        bn256::{pairing, Fr, G1Affine, G1, G2},
        group::Curve,
        pairing::Engine,
    },
};
use rand::thread_rng;

struct Proof {
    polynomial_commitment: G1Affine,
    quotient_commitment: G1Affine,
    y: Fr,
}

fn prover(p_committed: Polynomial, z: Fr, trusted_setup: TrustedSetup) -> Proof {
    let y = Polynomial::eval_fr(&p_committed, z);
    let p_fr = Polynomial::p_to_fr(&p_committed);
    let mut num: Vec<Fr> = Vec::new();
    let num_sub = p_fr[0] - (y);
    num.push(num_sub);

    for i in p_fr.iter().skip(1) {
        num.push(*i);
    }

    let mut den: Vec<Fr> = Vec::new();
    den.push(z.neg());
    den.push(Fr::one());

    let q_x = div_poly(num, den).0;
    let mut pi = G1::generator() * Fr::zero();

    for i in 0..q_x.len() {
        pi += trusted_setup.s_g1[i] * q_x[i];
    }

    let mut c = G1::generator() * Fr::zero();

    for i in 0..p_fr.len() {
        c += trusted_setup.s_g1[i] * p_fr[i];
    }

    let c_aff = c.to_affine();
    let pi_aff = pi.to_affine();

    let proof = Proof {
        polynomial_commitment: c_aff,
        quotient_commitment: pi_aff,
        y,
    };
    proof
}

fn verifier(proof: Proof, z: Fr, trusted_setup: TrustedSetup) -> bool {
    let y_g1_aff = (G1::generator() * proof.y).to_affine();
    let c_y = (proof.polynomial_commitment - y_g1_aff).to_affine();
    let z_g2 = G2::generator() * z;
    let s_z = trusted_setup.s_g2 - z_g2;
    let s_z_aff = s_z.to_affine();
    let h = G2::generator();
    let h_aff = h.to_affine();
    let pair_left = pairing(&proof.quotient_commitment, &s_z_aff);
    let pair_right = pairing(&c_y, &h_aff);

    pair_left == pair_right
}

#[cfg(test)]
mod tests {
    use crate::kzg::{prover, verifier, Fr};
    use crate::kzg_tools::{
        barycentric::barycentric,
        fft::fft,
        polynomial::{self, div_poly, mul_poly},
        trusted_setup::trusted_setup,
    };
    use halo2::arithmetic::Field;
    use rand::thread_rng;

    #[test]
    fn kzg_test() {
        let p_committed = polynomial::Polynomial::new(vec![1, 2, 3, 4, 5, 6, 7, 8]);
        let rou = p_committed.clone().rou();
        let p_fr = &p_committed.p_to_fr();

        //Evaluate a polynomial
        println!("{:?}\n", &p_committed.eval(16));

        //Evaluate a polynomial without FFT
        println!("{:?}\n", &p_committed.eval_fr(rou));

        //Evaluate a polynomial using FFT
        println!("{:?}\n", fft(p_fr.clone(), rou));

        let p_numerator = polynomial::Polynomial::new(vec![3, 2, 1, 6, 7, 9]);
        let p_denominator = polynomial::Polynomial::new(vec![2, 1, 5, 6]);
        println!("{} ÷ {}\n", p_numerator, p_denominator);

        //Divide two polynomials.
        //Numerator = 3x^0 + 2x^1 + 1x^2 denominator = 2x^0 + 1x^1 quotient = 0 + 1x and remainder = 3
        println!(
            "{:?}\n",
            div_poly(p_numerator.p_to_fr(), p_denominator.p_to_fr())
        );

        //Multiplying two polynomials.
        //First polynomial = 3x^0 + 2x^1 + 1x^2 second polynomial = 2x^0 + 1x^1 result = 6 + 7x + 4x^2 + 1x^3
        println!(
            "{:?}\n",
            mul_poly(p_numerator.p_to_fr(), p_denominator.p_to_fr())
        );

        //Evaluate polynomial using barycentric
        println!("{:?}\n", barycentric(p_fr.clone(), rou, 16.into()));

        //KZG commitment
        let rng = thread_rng();
        let z = Fr::random(rng.clone());
        let trusted_setup = trusted_setup(p_fr.to_vec());
        let prover = prover(p_committed, z, trusted_setup.clone());
        let verifier = verifier(prover, z, trusted_setup);
        assert!(verifier);
    }
}
