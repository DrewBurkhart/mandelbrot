use std::str::FromStr;

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

/// Parse the given string as a coordinate pair
///
/// If `s` does not parse, return None
/// Otherwise, return `Some<(x, y)>`
fn parse_pair<T: FromStr>(s: &str, seperator: char) -> Option<(T, T)> {
    match s.find(seperator) {
        None => None,
        Some(index) => match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None,
        },
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10,", ','), None);
    assert_eq!(parse_pair::<i32>(",10", ','), None);
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("0.5x", 'x'), None);
    assert_eq!(parse_pair::<i32>("0.5x1.5", 'x'), None);
}

fn main() {
    println!("Hello, world!");
}
