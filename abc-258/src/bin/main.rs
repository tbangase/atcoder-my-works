use proconio::{input, fastout};

use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        (n, q, x): (usize, usize, usize),
        w: [usize; n],
    }

    // calc c_i
    let s: usize = w.iter().sum();
    // println!("s: {}", s);

    let w_mirrored: Vec<&usize> = w.iter().chain(&w).collect();

    // ruler method
    let x = x % s;
    let mut containable_counts = vec![];
    let mut r = 0;
    let mut contained = 0;
    // calc containable_counts
    for l in 0..n {
        // println!("for l: {}", l);
        while contained < x {
            // println!("Putting potato: {}", w_mirrored[r]);
            // println!("Current count in container: {}", r);
            // println!("Current container size: {}\n", contained);
            contained += w_mirrored[r];
            r += 1;
        }
        containable_counts.push(r - l);
        let v = *w_mirrored[l];
        if contained < v {
            contained = 0;
        } else {
            contained -= w_mirrored[l];
        }
    }

    let mut appeared = HashMap::new();
    let mut p_tracks = vec![];

    let m;
    let mut current = 0;
    let mut step = 0;

    loop {
        if let Some(&stp) = appeared.get(&current) {
            m = stp;
            break;
        }
        appeared.insert(current, step);
        p_tracks.push(current);
        current = (current + containable_counts[current]) % n;
        step += 1;
    }

    // println!("containable_counts: {:?}", containable_counts);
    // println!("p_tracks: {:?}", p_tracks);
    // println!("start iterator index: {}", m);
    // println!("appeared: {:?}", appeared);
    let m = m as u64;

    for _ in 0..q {
        input! {
            mut k: u64
        }
        let k = k - 1;

        // handle query
        let p_track_count = p_tracks.len() as u64;
        let ans = if k < m as u64 {
            containable_counts[p_tracks[k as usize]]
        } else {
            let index = (m + (k - m) % (p_track_count - m)) as usize;
            containable_counts[p_tracks[index]]
        };

        println!("{}", ans);
    }
}
