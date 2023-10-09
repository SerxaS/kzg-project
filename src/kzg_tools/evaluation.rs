use halo2::{arithmetic::Field, halo2curves::bn256::Fr};

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
