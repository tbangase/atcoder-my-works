use proconio::{input, fastout, marker::*};

use std::thread;
use std::time::{Instant, Duration};
use std::io::{stdout, Write, Result};

fn main() -> Result<(), Box<dyn std::error::Error>>{
    input! {
        n: usize,
        pets: [(usize, usize, usize); n],
        m: usize,
        people: [(usize, usize); m],
    }

    let start = Instant::now();

    for i in 0..10 {
        println!("Printing... {i}");
        thread::sleep(Duration::from_secs(1));
        stdout().flush();
    }

    let duration = start.elapsed();
    println!("Duration: {:?}", duration);
}
