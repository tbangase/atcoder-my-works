use std::vec;

use proconio::fastout;
use proconio::input;

use std::cmp::max;

#[fastout]
fn main() {
    input! {
        (n, m): (usize, usize),
        mut x: [i32; m],  // Vec<(i32, i32, i32)>
    }

    x.sort();
    let mut diff = vec![];

    for i in 1..m {
        diff.push(x[i] - x[i - 1]);
    }
    diff.sort();

    let count = max(0, m as i32 - n as i32) as usize;
    let mut res = 0;
    for i in 0..count {
        res += diff[i];
    }

    println!("{}", res);
}


fn quick_sort<T>(vector: &mut Vec<T>, left: usize, right: usize)
where
    T: PartialOrd + Clone,
{
    if left >= right {
        return;
    }
    // ソート対象のベクトルから3つ取ってきて中央値をPivotとする
    let pivot;
    {
        let pivot_vec = vec![
            vector[left].clone(),
            vector[(left + (right - 1)) / 2].clone(),
            vector[left].clone(),
        ];

        pivot = medium(pivot_vec);
    }
    let mut i = left;
    let mut j = right;

    loop {
        // Pivotより小さい値のインデックスを探す
        while vector[i] > pivot { i += 1 }
        // Pivotより大きい値のインデックスを探す
        while vector[j] < pivot { j -= 1 }

        if i >= j { break; }

        vector.swap(i, j);

        i += 1;
        j -= 1;
    }

    // i is the index of last searched under pivot data
    // In other words, left to i - 1 is over the pivot.
    if i != 0 {
        quick_sort(vector, left, i - 1);
    }
    // same logic as obove, j + 1 to right is under the pivot.
    quick_sort(vector, j + 1, right);
}

fn medium<T: PartialOrd + Clone>(list: Vec<T>) -> T {
    if list[0] > list[1] {
        if list[1] > list[2] {
            let val = list[1].clone();
            return val;
        } else {
            if list[0] > list[2] {
                return list[2].clone();
            } else {
                return list[0].clone();
            }
        }
    } else {
        if list[0] > list[2] {
            return list[0].clone();
        } else {
            if list[1] > list[2] {
                return list[2].clone();
            } else {
                return list[1].clone();
            }
        }
    }
}
