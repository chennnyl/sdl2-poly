pub type PureFunc = dyn Fn(f64) -> f64;

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