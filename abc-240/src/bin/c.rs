use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        (n, x): (usize, isize),
        choises: [(isize, isize); n],
    }

    let (mut min, mut max) = (0, 0);
    let mut diff = vec![];
    let mut condition = false;

    for choise in choises {
        if choise.0 < choise.1 {
            min += choise.0;
            max += choise.1;
        } else {
            min += choise.1;
            max += choise.0;
        }
        diff.push((choise.1 - choise.0).abs());
    }
    diff.sort();
    diff.reverse();

    if x < min || x > max {
        condition = false;
    }

    // Diffのコレクションからどうやって調べるか...
    if (x - min).abs() < (x - max).abs() {
        // Min から調べていく
    } else {
        // Max から調べていく

    }

    println!("{}", max);
    println!("{}", min);
    println!("{:?}", diff);

    match condition {
        true => println!("Yes"),
        false => println!("No"),
    }
}

