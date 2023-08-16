use crate::kzg_tools::{
    barycentric::barycentric, polynomial::div_poly, trusted_setup::trusted_setup,
};
use halo2::{
    arithmetic::Field,
    halo2curves::{
        bn256::{pairing, Fr, G1, G2},
        group::Curve,
    },
};
use rand::thread_rng;

#[allow(dead_code)]
fn kzg(p: Vec<Fr>, rou: Fr) -> String {
    let rng = thread_rng();
    let z = Fr::random(rng.clone());
    let y = barycentric(p.clone(), rou, z);
    let mut num: Vec<Fr> = Vec::new();
    let num_sub = p[0] - (y);
    num.push(num_sub);

    for i in p.iter().skip(1) {
        num.push(*i);
    }

    let mut den: Vec<Fr> = Vec::new();
    den.push(z.neg());
    den.push(Fr::one());

    let q_x = div_poly(num, den).0;
    let trusted_s = trusted_setup(p.clone());
    let trusted_s_g1 = trusted_s.0;
    let mut pi = G1::generator() * Fr::zero();

    for i in 0..q_x.len() {
        pi += trusted_s_g1[i] * q_x[i];
    }

    let trusted_s_g2 = trusted_s.1;
    let z_g2 = G2::generator() * z;
    let s_z = trusted_s_g2 - z_g2;
    let s_z_aff = s_z.to_affine();
    let pi_aff = pi.to_affine();

    let y_g1 = G1::generator() * y;
    let mut c = G1::generator() * Fr::zero();

    for i in 0..p.len() {
        c += trusted_s_g1[i] * p[i];
    }

    let c_y = c - y_g1;
    let c_y_aff = c_y.to_affine();
    let h = G2::generator();
    let h_aff = h.to_affine();

    let pair_left = pairing(&pi_aff, &s_z_aff);
    let pair_right = pairing(&c_y_aff, &h_aff);

    if pair_left == pair_right {
        return "Equation holds, the verifier accepts the proof.".to_owned();
    } else {
        return "Equation does not hold, the verifier rejects the proof.".to_owned();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kzg_tools::{
        fft::fft,
        polynomial::{self, mul_poly},
    };

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
        println!("{:?}\n", kzg(p_fr.to_vec(), rou));
    }
}
