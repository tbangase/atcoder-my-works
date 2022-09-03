use proconio::{fastout, input};

const UP: char = '1';

#[fastout]
fn main() {
    input! {
        s: String,
    }

    let mut prev_pin_up = s.chars().nth(6).unwrap_or_default() == UP;
    let mut pin_have_been_up = prev_pin_up;

    let is_pin_1_down = s.chars().nth(0).unwrap_or_default() != UP;
    prev_pin_up = s.chars().nth(3).unwrap_or_default() == UP;
    pin_have_been_up = pin_have_been_up || prev_pin_up;

    let is_pin_up =
        s.chars().nth(1).unwrap_or_default() == UP || s.chars().nth(7).unwrap_or_default() == UP;
    if pin_have_been_up && !prev_pin_up && is_pin_up && is_pin_1_down {
        // is split
        println!("Yes");
        return;
    }
    prev_pin_up = is_pin_up;
    pin_have_been_up = pin_have_been_up || prev_pin_up;

    let is_pin_up =
        s.chars().nth(0).unwrap_or_default() == UP || s.chars().nth(4).unwrap_or_default() == UP;
    if pin_have_been_up && !prev_pin_up && is_pin_up && is_pin_1_down {
        // is split
        println!("Yes");
        return;
    }
    prev_pin_up = is_pin_up;
    pin_have_been_up = pin_have_been_up || prev_pin_up;

    let is_pin_up =
        s.chars().nth(2).unwrap_or_default() == UP || s.chars().nth(8).unwrap_or_default() == UP;
    if pin_have_been_up && !prev_pin_up && is_pin_up && is_pin_1_down {
        // is split
        println!("Yes");
        return;
    }
    prev_pin_up = is_pin_up;
    pin_have_been_up = pin_have_been_up || prev_pin_up;

    let is_pin_up = s.chars().nth(5).unwrap_or_default() == UP;
    if pin_have_been_up && !prev_pin_up && is_pin_up && is_pin_1_down {
        // is split
        println!("Yes");
        return;
    }
    prev_pin_up = is_pin_up;
    pin_have_been_up = pin_have_been_up || prev_pin_up;

    let is_pin_up = s.chars().nth(9).unwrap_or_default() == UP;
    if pin_have_been_up && !prev_pin_up && is_pin_up && is_pin_1_down {
        // is split
        println!("Yes");
        return;
    }

    println!("No");
}
