use super::polynomial::{pow, Evaluation, Polynomial};
use halo2::{
    arithmetic::Field,
    halo2curves::bn256::{Fr, G1, G2},
};
use rand::thread_rng;
use std::ops::Mul;

#[derive(Clone)]
pub struct TrustedSetup {
    pub s_g1: Vec<G1>,
    pub s_g2: G2,
}

pub fn trusted_setup(polynomial: Polynomial) -> TrustedSetup {
    let rng = thread_rng();
    let trusted_s = Fr::random(rng.clone());
    let mut s_g1 = Vec::new();
    let s_g2 = G2::generator().mul(trusted_s);

    for i in 0..polynomial.coeff.len() {
        let trusted_s_g1 =
            G1::generator().mul(pow(&Evaluation::new(trusted_s), i.try_into().unwrap()).evaluation);
        s_g1.push(trusted_s_g1);
    }

    TrustedSetup { s_g1, s_g2 }
}
