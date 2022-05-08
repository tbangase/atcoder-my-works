use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        (n, a, b): (usize, usize, usize),
    }

    for i in 0..(n * a) {
        let tile_row_odd = (i / a) % 2 == 1;
        for j in 0..(n * b) {
            let tile_col_odd = (j / b) % 2 == 1;
            if tile_row_odd ^ tile_col_odd {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}
