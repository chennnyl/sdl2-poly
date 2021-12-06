use sdl2::render::{Canvas, RenderTarget};
use sdl2::rect::{Point};
use crate::util::math;

pub fn draw_circle<T: RenderTarget>(x: i32, y: i32, r: i32, canvas: &mut Canvas<T>) -> Result<(), String> {
    let mut points: Vec<Point> = Vec::new();
    for p in 0..=(r as f64 * std::f64::consts::TAU) as i32 {
        let angle = p as f64/r as f64;
        let px = x + (r as f64 * angle.cos()) as i32;
        let py = y + (r as f64 * angle.sin()) as i32;
        points.push(Point::new(px, py));
    }
    canvas.draw_points(&points[..])
}

/// Draw a degree `n` polynomial of the form `ax^n + bx^(n-1) + ...`
pub fn draw_polynomial<T: RenderTarget>(coefficients: Vec<f64>, canvas: &mut Canvas<T>) -> Result<(), String> {
    draw_pure_function(math::poly_closure(coefficients), canvas)
}

pub fn draw_filled_polynomial<T: RenderTarget>(coefficients: Vec<f64>, canvas: &mut Canvas<T>) -> Result<(), String> {
    draw_filled_pure_function(math::poly_closure(coefficients), canvas)
}

pub fn draw_filled_pure_function<T: RenderTarget>(func: Box<math::PureFunc>, canvas: &mut Canvas<T>) -> Result<(), String> {
    let y_positions = (0..canvas.viewport().width()).map(|x| func(x as f64) as i32).collect::<Vec<i32>>();
    canvas.draw_lines(
        &y_positions.iter().zip(0..canvas.viewport().width() as i32).map(
            |(y, x)| vec![Point::new(x, *y), Point::new(x, canvas.viewport().height() as i32)]
        ).flatten().collect::<Vec<Point>>()[..]
    )
}

pub fn draw_pure_function<T: RenderTarget>(func: Box<math::PureFunc>, canvas: &mut Canvas<T>) -> Result<(), String> {
    let mut points: Vec<Point> = Vec::new();
    for x in 0..canvas.viewport().width() {
        let x: f64 = x as f64;
        points.push(Point::new(x as i32, func(x) as i32));
    }

    canvas.draw_lines(&points[..])
}

pub fn draw_parametric_function<T: RenderTarget>(func: Box<math::ParametricFunc>, domain: math::FunctionDomain, time_step: f64, canvas: &mut Canvas<T>) -> Result<(), String> {
    let mut points: Vec<Point> = Vec::new();

    let t0 = domain.start_end(time_step).0;
    for t in 0..domain.num_points(time_step) {
        let t: f64 = t as f64*time_step + t0;
        let (x, y) = func(t);
        let x: i32 = x as i32;
        let y: i32 = y as i32;
        points.push(
            Point::new(x, y)
        )
    }

    canvas.draw_lines(&points[..])
    
}