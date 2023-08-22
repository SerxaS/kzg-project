use core::fmt;
use halo2::{
    arithmetic::Field,
    halo2curves::{bn256::Fr, ff::PrimeField},
};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Polynomial {
    pub(crate) coeff: Vec<Fr>,
}

impl Polynomial {
    pub fn new(coeff: Vec<Fr>) -> Self {
        Self { coeff }
    }

    //Evaluates polynomial in the field.
    pub fn eval(&self, val: Fr) -> Evaluation {
        let mut eval = Fr::zero();

        for (i, j) in self.coeff.iter().enumerate() {
            eval += (Evaluation::new(*j).mul(&pow(&Evaluation::new(val), i.try_into().unwrap())))
                .evaluation
        }
        Evaluation::new(eval)
    }

    //Calculate required roots of unity.
    pub fn rou(&self) -> Evaluation {
        let mut len = self.coeff.len();
        let mut rou = Evaluation::new(<Fr as PrimeField>::ROOT_OF_UNITY);
        let mut counter = 0;

        while len / 2 >= 1 {
            len = len / 2;
            counter += 1;
        }

        for _ in 0..(28 - counter) {
            rou.evaluation = rou.evaluation.square();
        }
        rou
    }

    //Polynomial Long Division.
    pub fn div_poly(&mut self, den: Vec<Fr>) -> (Self, Self) {
        if den.len() > self.coeff.len() {
            return (Self::new(vec![Fr::zero()]), self.clone());
        }

        let diff = self.coeff.len() - den.len();
        let mut q = Polynomial::new(vec![Fr::zero(); diff + 1]);

        for i in (0..q.coeff.len()).rev() {
            let n_idx = self.coeff.len() - 1 - diff + i;
            let inv_d = den[den.len() - 1].invert().unwrap();
            q.coeff[i] = self.coeff[n_idx].mul(&inv_d);

            for j in 0..den.len() {
                self.coeff[n_idx - j] -= q.coeff[i].mul(&den[den.len() - j - 1]);
            }
        }

        for i in (1..self.coeff.len()).rev() {
            if self.coeff[i] == 0.into() {
                self.coeff.pop();
            } else {
                break;
            }
        }
        (q, self.clone())
    }

    //Polynomial Multiplication.
    pub fn mul_poly(&self, rhs: Vec<Fr>) -> Self {
        let p_len = self.coeff.len() + rhs.len() - 1;
        let mut p_res = Polynomial::new(vec![Fr::zero(); p_len]);

        for i in 0..self.coeff.len() {
            for j in 0..rhs.len() {
                p_res.coeff[i + j] += self.coeff[i] * rhs[j];
            }
        }
        p_res
    }
}

//Shows polynomial in the string form.
impl Display for Polynomial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let result: Vec<String> = self
            .coeff
            .iter()
            .enumerate()
            .map(|(i, c)| format!("({:?})x^{}", c, i))
            .collect();
        write!(f, "{}", result.join(" + "))
    }
}

pub struct Evaluation {
    pub(crate) evaluation: Fr,
}

impl Evaluation {
    pub fn new(evaluation: Fr) -> Self {
        Self { evaluation }
    }

    pub fn add(&self, rhs: Self) -> Self {
        Evaluation::new(self.evaluation + rhs.evaluation)
    }

    pub fn sub(&self, rhs: Self) -> Self {
        Evaluation::new(self.evaluation - rhs.evaluation)
    }

    pub fn mul(&self, rhs: &Self) -> Self {
        Evaluation::new(self.evaluation * rhs.evaluation)
    }

    pub fn div(&self, divider: Self) -> Self {
        Evaluation::new(self.evaluation * divider.evaluation.invert().unwrap())
    }
}

//Calculate exponent of a number as field element.
pub fn pow(base: &Evaluation, exp: usize) -> Evaluation {
    let mut mul = Fr::one();

    for _ in 0..exp {
        mul *= base.evaluation
    }
    Evaluation { evaluation: mul }
}

pub struct PolynomialU32 {
    pub(crate) coeff: Vec<u32>,
}

impl PolynomialU32 {
    pub fn new(coeff: Vec<u32>) -> Self {
        Self { coeff }
    }

    //Evaluates polynomial.
    pub fn eval(&self, val: u32) -> u32 {
        let mut eval = 0;

        for (i, j) in self.coeff.iter().enumerate() {
            eval += j * (val.pow(i.try_into().unwrap()));
        }
        eval
    }

    //Converts polynomial to the corresponding field elements.
    pub fn p_to_fr(&self) -> Polynomial {
        let mut vec = Polynomial::new(Vec::new());

        for i in self.coeff.iter() {
            let p_fr = i;
            let p_fr = Fr::from_u128((*p_fr).into());
            vec.coeff.push(p_fr);
        }
        vec
    }
}

//Shows polynomial in the string form.
impl Display for PolynomialU32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let result: Vec<String> = self
            .coeff
            .iter()
            .enumerate()
            .map(|(i, c)| format!("({:?})x^{}", c, i))
            .collect();
        write!(f, "{}", result.join(" + "))
    }
}
