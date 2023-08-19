use super::fft::pow;
use core::fmt;
use halo2::{
    arithmetic::Field,
    halo2curves::{bn256::Fr, ff::PrimeField},
};
use std::fmt::Display;

#[derive(Clone)]
pub(crate) struct Polynomial {
    polynomial: Vec<u32>,
}

impl Polynomial {
    pub fn new(polynomial: Vec<u32>) -> Polynomial {
        Polynomial { polynomial }
    }

    //Evaluates polynomial.
    pub fn eval(&self, val: u32) -> u32 {
        let mut eval = 0;

        for (i, j) in self.polynomial.iter().enumerate() {
            eval += j * (val.pow(i.try_into().unwrap()));
        }
        eval
    }

    //Converts polynomial to the corresponding field elements.
    pub fn p_to_fr(&self) -> Vec<Fr> {
        let mut vec: Vec<Fr> = Vec::new();

        for i in self.polynomial.iter() {
            let p_fr = i;
            let p_fr = Fr::from_u128((*p_fr).into());
            vec.push(p_fr);
        }
        vec
    }

    //Evaluates polynomial in the field.
    pub fn eval_fr(&self, val: Fr) -> Fr {
        let p_fr = self.p_to_fr();
        let mut eval = Fr::zero();

        for (i, j) in p_fr.iter().enumerate() {
            eval += j * pow(val, i.try_into().unwrap()).evaluation
        }
        eval
    }

    //Calculate required roots of unity.
    pub fn rou(&self) -> Fr {
        let mut len = self.polynomial.len();
        let mut rou = <Fr as PrimeField>::ROOT_OF_UNITY;
        let mut counter = 0;

        while len / 2 >= 1 {
            len = len / 2;
            counter += 1;
        }

        for _ in 0..(28 - counter) {
            rou = rou.square();
        }
        rou
    }
}

//Shows polynomial in the string form.
impl Display for Polynomial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let result: Vec<String> = self
            .polynomial
            .iter()
            .enumerate()
            .map(|(i, c)| format!("({:?})x^{}", c, i))
            .collect();
        write!(f, "{}", result.join(" + "))
    }
}

//Polynomial Long Division.
pub fn div_poly(mut n: Vec<Fr>, d: Vec<Fr>) -> (Vec<Fr>, Vec<Fr>) {
    if d.len() > n.len() {
        return (vec![Fr::zero()], n);
    }

    let diff = n.len() - d.len();
    let mut q: Vec<Fr> = vec![Fr::zero(); diff + 1];

    for i in (0..q.len()).rev() {
        let n_idx = n.len() - 1 - diff + i;
        let inv_d = d[d.len() - 1].invert().unwrap();
        q[i] = n[n_idx].mul(&inv_d);

        for j in 0..d.len() {
            n[n_idx - j] -= q[i].mul(&d[d.len() - j - 1]);
        }
    }

    for i in (1..n.len()).rev() {
        if n[i] == 0.into() {
            n.pop();
        } else {
            break;
        }
    }
    (q, n)
}

//Polynomial Multiplication.
pub fn mul_poly(p_left: Vec<Fr>, p_right: Vec<Fr>) -> Vec<Fr> {
    let p_len = p_left.len() + p_right.len() - 1;
    let mut p_res = vec![Fr::zero(); p_len];

    for i in 0..p_left.len() {
        for j in 0..p_right.len() {
            p_res[i + j] += p_left[i] * p_right[j];
        }
    }
    p_res
}
