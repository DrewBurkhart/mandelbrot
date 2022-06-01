/// Try to determine if `c` is in the
/// mandelbrot set usin at most `limit`
/// iterations to decide
///
/// If `c` is not a member, return None
/// Otherwise, return Some(i) where i is the
/// number of iterations it took for c to
/// leave the circle of radius 2 centered
/// on the origin.
fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }

    None
}

fn main() {
    println!("Hello, world!");
}
