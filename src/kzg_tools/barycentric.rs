use super::fft::pow;
use crate::kzg_tools::fft::fft;
use halo2::{
    arithmetic::Field,
    halo2curves::{bn256::Fr, ff::PrimeField},
};

//Barycentric  operation.
pub(crate) fn barycentric(p: Vec<Fr>, rou: Fr, num: Fr) -> Fr {
    let len = p.len();
    let eval = fft(p, rou);
    let mut right_res = Vec::new();

    for (i, j) in eval.iter().enumerate() {
        let pow = pow(rou, i.try_into().unwrap());
        let j_mul_pow = j.mul(&pow.evaluation);
        let num_sub_pow = num.sub(&pow.evaluation);
        let divide_res = j_mul_pow.mul(&num_sub_pow.invert().unwrap());
        right_res.push(divide_res);
    }

    let mut sum_res = Fr::zero();

    for i in right_res {
        sum_res += i;
    }

    let pow_num = pow(num, len.try_into().unwrap()).evaluation.sub(&Fr::one());
    let len_fr = Fr::from_u128(len.try_into().unwrap());
    let left_res = pow_num.mul(&len_fr.invert().unwrap());
    let res = left_res.mul(&sum_res);

    res
}
