#[derive(Debug, Clone)]
struct Polynomial {
    coefficients: Vec<u32>,
}

impl Polynomial {
    fn new(coefficients: Vec<u32>) -> Polynomial {
        Polynomial { coefficients }
    }

    //Evaluates polynomial.
    fn eval(&self, val: u32) -> u32 {
        let mut eval = 0;

        for (i, j) in self.coefficients.iter().enumerate() {
            eval += j * (val.pow(i.try_into().unwrap()));
        }
        return eval;
    }
}
