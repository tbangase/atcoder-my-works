use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        point_1: (f64, f64),
        point_2: (f64, f64),
    }

    let x_diff = (point_2.0 - point_1.0).abs();
    let y_diff = (point_2.1 - point_1.1).abs();

    let condition_1 = x_diff == 3.0 && y_diff == 3.0;
    let condition_2 = x_diff == 2.0 && y_diff == 4.0;
    let condition_3 = x_diff == 4.0 && y_diff == 2.0;
    let condition_4 = x_diff == 1.0 && y_diff == 1.0;
    let condition_5 = x_diff == 0.0 && y_diff == 4.0;
    let condition_6 = x_diff == 4.0 && y_diff == 0.0;
    let condition_7 = x_diff == 1.0 && y_diff == 3.0;
    let condition_8 = x_diff == 3.0 && y_diff == 1.0;
    let condition_9 = x_diff == 0.0 && y_diff == 2.0;
    let condition_10= x_diff == 2.0 && y_diff == 0.0;

    match condition_1 || condition_2 || condition_3 || condition_4 
        || condition_5 || condition_6 || condition_7 || condition_8 
        || condition_9 || condition_10 {
        true => println!("Yes"),
        false => println!("No"),
    }
}
