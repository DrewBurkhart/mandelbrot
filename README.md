
![Logo](mandel.png)

# Mandelbrot Plotter

A rust package that plots the Mandelbrot set (or a customizable slide of it) as cartesian coordinates on the complex plane.

## Authors

- [Andrew Burkhart (@drewburkhart)](https://www.github.com/drewburkhart)


## Tech Stack

Rust


## To Run

```bash
cargo build --release
target/release/mandelbrot {filename} {dimensions} {upper_left_bound} {lower_right_bound}
```

#### Example

> Note: I typically run this with `time` to get an understanding of the performance.

```bash
time target/release/mandelbrot mandel.png 4000x3000 -1.20,0.35 -1,0.20
```

## Acknowledgements

Based on knowledge from [Programming Rust v2](https://www.oreilly.com/library/view/programming-rust-2nd/9781492052586/)

