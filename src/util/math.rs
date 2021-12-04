pub type PureFunc = dyn Fn(f64) -> f64;
pub type ParametricFunc = dyn Fn(f64) -> (f64, f64);

pub enum FunctionDomainType {
    Exclusive(f64),
    Inclusive(f64)
}
pub struct FunctionDomain(pub FunctionDomainType, pub FunctionDomainType);

impl FunctionDomain {
    pub fn build_inclusive(lower: f64, upper: f64) -> Self {
        Self(FunctionDomainType::Inclusive(lower), FunctionDomainType::Inclusive(lower))
    }

    pub fn start_end(&self, time_step: f64) -> (f64, f64) {
        let t0 = match self.0 {
            FunctionDomainType::Exclusive(t) => t + time_step,
            FunctionDomainType::Inclusive(t) => t
        };
        let tf = match self.1 {
            FunctionDomainType::Exclusive(t) => t + time_step,
            FunctionDomainType::Inclusive(t) => t
        };
        (t0, tf)
    }
    pub fn num_points(&self, time_step: f64) -> usize {
        // TODO: fix how this works
        let (t0, tf) = self.start_end(time_step);
        ((tf-t0)/time_step) as usize
    }
    pub fn shift(self, amount: f64) -> Self {
        Self(
            match self.0 { 
                FunctionDomainType::Exclusive(t) => FunctionDomainType::Exclusive(t+amount),
                FunctionDomainType::Inclusive(t) => FunctionDomainType::Inclusive(t+amount)
            },
            match self.1 { 
                FunctionDomainType::Exclusive(t) => FunctionDomainType::Exclusive(t+amount),
                FunctionDomainType::Inclusive(t) => FunctionDomainType::Inclusive(t+amount)
            }
        )
    }
}

pub enum StdDomains {
    ZERO_TWOPI,
}

pub fn differentiate(mut coefficients: Vec<f64>) -> Vec<f64> {
    if coefficients.len() <= 1 {
        return vec![0.0]
    }
    coefficients.pop();
    for (i, coeff) in coefficients.iter_mut().rev().enumerate() {
        *coeff *= (i+1) as f64;
    }

    coefficients
}

pub fn n_differentiate(mut coefficients: Vec<f64>, mut n: usize) -> Vec<f64> {
    while coefficients.len() >= 1 && n > 0 {
        coefficients = differentiate(coefficients);
        n -= 1;
    }
    coefficients
}

pub fn poly_closure(coefficients: Vec<f64>) -> Box<PureFunc> {
    Box::new(move |x: f64| coefficients
        .iter()
        .rev()
        .enumerate()
        .map(|(i,c)| c*x.powi(i as i32))
        .sum()
    )
}

pub fn lissajous(w: f64, h: f64, kx: usize, ky: usize) -> (Box<ParametricFunc>, FunctionDomain) {

    (
        Box::new(move |t: f64| (w/2.0 * (1.0 + (t*kx as f64).cos()), h/2.0 * (1.0 + (t*ky as f64).sin()))),
        FunctionDomain(FunctionDomainType::Inclusive(0.0), FunctionDomainType::Inclusive(std::f64::consts::TAU))
    )
}