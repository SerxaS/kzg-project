use crate::kzg_tools::evaluation::Evaluation;
use core::fmt;
use halo2::{
    arithmetic::Field,
    halo2curves::{bn256::Fr, ff::PrimeField},
};
use rand::thread_rng;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Polynomial {
    pub(crate) coeff: Vec<Fr>,
}

impl Polynomial {
    pub fn new(coeff: Vec<Fr>) -> Self {
        Self { coeff }
    }

    /// Takes degree of polynomial and creates a polynomial with random coefficients.
    pub fn create_polynomial(degree: u32) -> Self {
        let rng = thread_rng();
        let random_coeff =
            Polynomial::new((0..degree + 1).map(|_| Fr::random(rng.clone())).collect());
        random_coeff
    }

    /// Evaluates polynomial in the field.
    pub fn eval(&self, val: Fr) -> Evaluation {
        let mut eval = Fr::zero();

        for (i, j) in self.coeff.iter().enumerate() {
            eval += (Evaluation::new(*j).mul(&pow(val, i.try_into().unwrap()))).evaluation
        }
        Evaluation::new(eval)
    }

    /// Polynomial addition.
    pub fn add_poly(&self, rhs: Vec<Fr>) -> Self {
        let mut len = 0;

        if self.coeff.len() >= rhs.len() {
            len += self.coeff.len();
        } else {
            len += rhs.len();
        }
        let mut sum_poly = Polynomial::new(vec![Fr::zero(); len]);

        for (i, _) in self.coeff.iter().enumerate() {
            sum_poly.coeff[i] += self.coeff[i];
        }

        for (i, _) in rhs.iter().enumerate() {
            sum_poly.coeff[i] += rhs[i];
        }
        sum_poly
    }

    /// Polynomial subtraction.
    pub fn sub_poly(&self, rhs: Vec<Fr>) -> Self {
        let mut len = 0;

        if self.coeff.len() >= rhs.len() {
            len += self.coeff.len();
        } else {
            len += rhs.len();
        }

        let mut sub_poly = Polynomial::new(vec![Fr::zero(); len]);

        for (i, _) in self.coeff.iter().enumerate() {
            sub_poly.coeff[i] += self.coeff[i];
        }

        for (i, _) in rhs.iter().enumerate() {
            sub_poly.coeff[i] -= rhs[i];
        }
        sub_poly
    }

    /// Polynomial long division.
    pub fn div_poly(&mut self, den: Vec<Fr>) -> (Self, Self) {
        if den.len() > self.coeff.len() {
            return (Self::new(vec![Fr::zero()]), self.clone());
        }
        let diff = self.coeff.len() - den.len();
        let mut quotient = Polynomial::new(vec![Fr::zero(); diff + 1]);

        for i in (0..quotient.coeff.len()).rev() {
            let n_idx = self.coeff.len() - 1 - diff + i;
            let inv_d = den[den.len() - 1].invert().unwrap();
            quotient.coeff[i] = self.coeff[n_idx].mul(&inv_d);

            for j in 0..den.len() {
                self.coeff[n_idx - j] -= quotient.coeff[i].mul(&den[den.len() - j - 1]);
            }
        }

        for i in (1..self.coeff.len()).rev() {
            if self.coeff[i] == 0.into() {
                self.coeff.pop();
            } else {
                break;
            }
        }
        let remainder = self.clone();
        (quotient, remainder)
    }

    /// Polynomial multiplication.
    pub fn mul_poly(&self, rhs: Vec<Fr>) -> Self {
        let p_len = self.coeff.len() + rhs.len() - 1;
        let mut mul_poly = Polynomial::new(vec![Fr::zero(); p_len]);

        for i in 0..self.coeff.len() {
            for j in 0..rhs.len() {
                mul_poly.coeff[i + j] += self.coeff[i] * rhs[j];
            }
        }
        mul_poly
    }

    /// Calculate required roots of unity.
    pub fn rou(&self) -> Evaluation {
        let mut len = self.coeff.len();
        let mut rou = Evaluation::new(PrimeField::ROOT_OF_UNITY);
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

    /// Evaluate committed polynomial at determined number of points(signatures).
    pub fn eval_of_z_points(&self, k_points: u32) -> (Self, Self) {
        let rng = thread_rng();
        let mut z_points = Polynomial::new(Vec::new());
        let mut y_points = Polynomial::new(Vec::new());

        for _ in 0..k_points {
            let random_num = Fr::random(rng.clone());
            let eval = Polynomial::eval(&self, random_num);
            y_points.coeff.push(eval.evaluation);
            z_points.coeff.push(random_num);
        }
        (z_points, y_points)
    }

    /// Lagrange Interpolation Method.  
    pub fn lagrange(&self, z_points: Vec<Fr>, y_points: Vec<Fr>) -> Self {
        let mut interpolate_polynomial = Polynomial::new(Vec::new());

        for i in 0..z_points.len() {
            let numerator = Polynomial::new(vec![Fr::one()]);
            let denominator = Polynomial::new(vec![Fr::one()]);
            let mut term = Polynomial::new(vec![y_points[i]]);

           for j in 0..z_points.len() {
                if j != i {
                    let mut dividend = numerator
                        .mul_poly(Polynomial::new(vec![z_points[j].neg(), Fr::one()]).coeff);
                    let divider = denominator
                        .mul_poly(Polynomial::new(vec![z_points[i] - z_points[j]]).coeff);
                    term = term.mul_poly((dividend.div_poly(divider.coeff).0).coeff);
                }
            }
            interpolate_polynomial = term.add_poly(interpolate_polynomial.coeff);
        }
        interpolate_polynomial
    }
}

/// Shows polynomial in the string form.
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

/// Calculate exponent of a number as field element.
pub fn pow(base: Fr, exp: usize) -> Evaluation {
    let mut mul = Fr::one();

    for _ in 0..exp {
        mul *= base
    }
    Evaluation { evaluation: mul }
}
