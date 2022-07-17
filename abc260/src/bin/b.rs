use std::collections::HashSet;
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        (x, y, z): (usize, usize, usize),
        math_list: [usize; n],
        en_list: [usize; n],
    }

    let sum_list = math_list.iter().zip(&en_list).map(|(&a, &b)| a + b).collect::<Vec<usize>>();

    let mut passed_set = HashSet::new();

    let mut pass_count = 0;

    let sort_func = |(a_i, a): &(usize, &usize), (b_i, b): &(usize, &usize)| {
        let first_cmp = a.cmp(b);
        if std::cmp::Ordering::Equal == first_cmp {
            a_i.cmp(b_i).reverse()
        } else {
            first_cmp
        }
    };

    for (i, _) in math_list.iter().enumerate()
        .sorted_by(sort_func).rev() {
        if pass_count >= x {
            break;
        }

        passed_set.insert(i);
        pass_count += 1;
    };

    pass_count = 0;

    for (i, _) in en_list.iter().enumerate()
        .sorted_by(sort_func).rev() {
        if pass_count >= y {
            break;
        }

        match passed_set.get(&i) {
            Some(_) => continue,
            None => {
                passed_set.insert(i);
                pass_count += 1;
            }
        };
    };

    pass_count = 0;

    for (i, _) in sum_list.iter().enumerate()
        .sorted_by(sort_func).rev() {
        if pass_count >= z {
            break;
        }

        match passed_set.get(&i) {
            Some(_) => continue,
            None => {
                passed_set.insert(i);
                pass_count += 1;
            }
        };
    };

    let mut passed_list: Vec<usize> = passed_set.into_iter().collect();
    passed_list.sort();

    for index in passed_list.iter() {
        println!("{}", index + 1);
    }
}
