use proconio::{input, fastout, marker::*};

#[fastout]
fn main() {
    input! {
        (h, w): (usize, usize),
        c: [Chars; h],
    }

    let mut map = vec![vec![0; w]; h];

    for i in 0..h {
        for j in 0..w {
            if i == 0 && j == 0 {
                map[i][j] = 1;
            } else if i == 0 {
                if map[i][j - 1] != 0 && c[i][j] != '#' {
                    map[i][j] = map[i][j - 1] + 1;
                }
            } else if j == 0 {
                if map[i - 1][j] != 0 && c[i][j] != '#' {
                    map[i][j] = map[i - 1][j] + 1;
                }
            } else {
                if (map[i][j - 1] != 0 || map[i - 1][j] != 0)
                    && c[i][j] != '#' {
                    map[i][j] = i + j + 1;
                }
            }
        }
    }

    let mut max = 0;

    for i in 0..h {
        for j in 0..w {
            if max < map[i][j] {
                max = map[i][j];
            }
        }
    }

    println!("{}", max);
}
