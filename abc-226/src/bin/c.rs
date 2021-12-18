use proconio::{input, fastout};

use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut a_matrix: Vec<Vec<usize>> = vec![];
    let mut t_list: Vec<u64> = vec![];

    for _ in 0..n {
        input! {
            (t, k): (u64, usize),
            a: [usize; k],
        }
        a_matrix.push(a);
        t_list.push(t);
    }

    let mut task_list: HashSet<usize> = HashSet::new();

    set_task_list(&mut task_list, &a_matrix, a_matrix.len() - 1);

    // Add time for getting last skill
    let mut cost: u64 = t_list.get(n - 1).unwrap().clone();

    // Collect elements to Vector for iteration.
    let task_list = task_list.into_iter().collect::<Vec<_>>();

    for task in task_list {
        cost += t_list[task];
    }

    println!("{}", cost);
}

fn set_task_list(task_list :&mut HashSet<usize>, a_matrix: &Vec<Vec<usize>>, t: usize) {
    for a in a_matrix[t].clone() {
        if task_list.insert(a - 1) {
            set_task_list(task_list, a_matrix, a - 1)
        }
    }
}
