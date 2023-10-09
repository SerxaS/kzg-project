use super::{
    evaluation::Evaluation,
    polynomial::{pow, Polynomial},
};
use crate::kzg_tools::fft::fft;
use halo2::{
    arithmetic::Field,
    halo2curves::{bn256::Fr, ff::PrimeField},
};

///Taking a set of evaluations of a polynomial and using that directly to compute
///an evaluation at a different point.
pub(crate) fn barycentric(polynomial: Polynomial, rou: Fr, x: Fr) -> Evaluation {
    let len = polynomial.coeff.len();
    //Evaluate polynomial at (degree + 1) points using FFT Algorithm.
    let eval = fft(polynomial, rou);
    let mut right_res = Polynomial::new(Vec::new());

    for (i, j) in eval.coeff.iter().enumerate() {
        //Right side of equation.
        let w_i = pow(rou, i);
        let y_i_mul_w_i = Evaluation::new(*j).mul(&w_i);
        let divide_res = y_i_mul_w_i.div(Evaluation::new(x).sub(w_i));
        right_res.coeff.push(divide_res.evaluation);
    }
    let mut sum_res = Evaluation::new(Fr::zero());

    for i in right_res.coeff {
        sum_res.evaluation += i;
    }

    //Left side of equation.
    let x_n = pow(x, len).sub(Evaluation::new(Fr::one()));
    let n = Evaluation::new(Fr::from_u128(len.try_into().unwrap()));
    let left_res = x_n.mul(&Evaluation::new(n.evaluation.invert().unwrap()));
    let res = left_res.mul(&Evaluation::new(sum_res.evaluation));
    res
}
