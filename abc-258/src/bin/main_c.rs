use proconio::{input, fastout, marker::*};

#[fastout]
fn main() {
    input! {
        (k, q): (usize, usize),
        s: Chars
    }

    let mut offset = 0usize;

    for _ in 0..q {
        input! {
            kind: usize, x: usize
        }

        match kind {
            1 => {
                offset += x;
                offset %= k;
            },
            2 => {
                let mut x = x - 1;
                if x < offset {
                    x += k
                }
                let index = x - offset;

                println!("{}", s.get(index).unwrap());
            },
            _ => ()
        }
    }
}
