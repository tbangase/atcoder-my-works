use proconio::input;

fn main() {
    input! {
        n: usize,
        (a, b): (i64, i64),
        mut list: [i64; n],
    }

    list.sort();

    let mut high = *list.iter().max().unwrap();
    let mut low = *list.iter().min().unwrap();

    while low != high {
        let border = (low + high) / 2;
        let mut expect_plus_minus = 0;
        for &val in list.iter() {
            if border > val {
                expect_plus_minus += ((border as f64 - val as f64) / a as f64).ceil() as i64;
            } else {
                expect_plus_minus -= (val - border) / b;
            }
        }
        if expect_plus_minus <= 0 {
            low = border + 1;
        } else {
            high = border;
        }
    }

    if low != *list.iter().min().unwrap() {
        println!("{}", low - 1);
    } else {
        println!("{}", low);
    }
}
