use crate::kzg_tools::polynomial::{pow, Polynomial};
use halo2::halo2curves::bn256::Fr;

/// Evaluate polynomial at (degree + 1) points using FFT Algorithm.
pub fn fft(polynomial: Polynomial, rou: Fr) -> Polynomial {
    let len = polynomial.coeff.len();
    let mut fft_values = Polynomial::new(vec![Fr::zero(); len]);

    if len == 1 {
        return polynomial;
    } else {
        let mut even = Polynomial::new(Vec::new());
        let mut odd = Polynomial::new(Vec::new());

        for (i, j) in polynomial.coeff.iter().enumerate() {
            if i % 2 == 0 {
                even.coeff.push(*j);
            } else {
                odd.coeff.push(*j);
            }
        }
        let even_fft = fft(even, rou.square());
        let odd_fft = fft(odd, rou.square());

        for i in 0..len / 2 {
            let temp_rou = pow(rou, i.try_into().unwrap());
            fft_values.coeff[i] =
                even_fft.coeff[i].add(&temp_rou.evaluation.mul(&odd_fft.coeff[i]));
            fft_values.coeff[i + len / 2] =
                even_fft.coeff[i].sub(&temp_rou.evaluation.mul(&odd_fft.coeff[i]));
        }
    }
    fft_values
}
