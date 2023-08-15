use halo2::halo2curves::bn256::Fr;

pub struct Evaluation {
    pub(crate) evaluation: Fr,
}

impl Evaluation {
    pub fn new(val: Fr) -> Self {
        Evaluation { evaluation: val }
    }
}

//Calculate exponent of a number as field element.
pub fn pow(base: Fr, exp: u32) -> Evaluation {
    let mut mul = Fr::one();

    for _ in 0..exp {
        mul *= base
    }
    Evaluation { evaluation: mul }
}

//FFT operation.
pub fn fft(p: Vec<Fr>, rou: Fr) -> Vec<Fr> {
    let len = p.len();
    let mut vec = vec![Fr::zero(); len];

    if len == 1 {
        return p;
    } else {
        let mut even = Vec::new();
        let mut odd = Vec::new();

        for (i, j) in p.iter().enumerate() {
            if i % 2 == 0 {
                even.push(*j);
            } else {
                odd.push(*j);
            }
        }

        let even_fft = fft(even, rou.square());
        let odd_fft = fft(odd, rou.square());

        for i in 0..len / 2 {
            let temp_rou = pow(rou, i.try_into().unwrap());
            vec[i] = even_fft[i].add(&temp_rou.evaluation.mul(&odd_fft[i]));
            vec[i + len / 2] = even_fft[i].sub(&temp_rou.evaluation.mul(&odd_fft[i]));
        }
    }
    return vec;
}
