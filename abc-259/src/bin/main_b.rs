use proconio::{input, fastout};

use std::time::Instant;
use std::f64::consts::PI;

#[fastout]
fn main() {
    input! {
        a: f64,
        b: f64,
        d: f64,
    }

    let start = Instant::now();

    let rad = d / 360. * 2. * PI;

    let x = a * rad.cos() - b * rad.sin();
    let y = a * rad.sin() + b * rad.cos();

    println!("{} {}", x, y);
    let duration = start.elapsed();
    println!("Duration: {:?}", duration);
}
