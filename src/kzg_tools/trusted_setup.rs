use super::fft::pow;
use halo2::{
    arithmetic::Field,
    halo2curves::bn256::{Fr, G1, G2},
};
use rand::thread_rng;
use std::ops::Mul;

//Trusted Setup  operation.
pub fn trusted_setup(p: Vec<Fr>) -> (Vec<G1>, G2) {
    let rng = thread_rng();
    let trusted_s = Fr::random(rng.clone());
    let mut trusted_p = Vec::new();
    let s_g2 = G2::generator().mul(trusted_s);

    for i in 0..p.len() {
        let s_g1 = G1::generator().mul(pow(trusted_s, i.try_into().unwrap()).evaluation);
        trusted_p.push(s_g1);
    }

    (trusted_p, s_g2)
}
