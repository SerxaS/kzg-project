use super::polynomial::{pow, Polynomial};
use halo2::{
    arithmetic::Field,
    halo2curves::bn256::{Fr, G1, G2},
};
use rand::thread_rng;
use std::ops::Mul;

#[derive(Clone)]
pub struct TrustedSetup {
    pub s_g1: Vec<G1>,
    pub s_g2: Vec<G2>,
}

///Using an MPC setup(Trusted Setup), the secret s is generated, and using this secret value,
///two sets will be distributed publicly, one for [s^i]_1, and one for [s^i]_2 .
///The secret s is then discarded forever.
pub fn trusted_setup(polynomial_degree: u32) -> TrustedSetup {
    let rng = thread_rng();
    let trusted_s = Fr::random(rng.clone());
    let mut s_g1 = Vec::new();
    let mut s_g2 = Vec::new();

    for i in 0..polynomial_degree + 1 {
        let trusted_s_g1 = G1::generator().mul(pow(trusted_s, i.try_into().unwrap()).evaluation);
        s_g1.push(trusted_s_g1);
    }

    for i in 0..polynomial_degree + 1 {
        let trusted_s_g2 = G2::generator().mul(pow(trusted_s, i.try_into().unwrap()).evaluation);
        s_g2.push(trusted_s_g2);
    }
    TrustedSetup { s_g1, s_g2 }
}
