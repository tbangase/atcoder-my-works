use proconio::input;
use proconio::fastout;
use std::collections::BTreeMap;

#[fastout]
fn main() {
    input! {
    n: usize, k : usize, 
    pn : [usize; n],
  }

  let mut bt = BTreeMap::<usize, Vec<usize>>::new();
  let mut ans = vec![-1; n + 1];
  for i in 0..n {
    let p = pn[i];
    let turn = (i + 1) as isize;

    let target = if let Some((&num, _)) = &bt.range(p..).next() {
      num
    } else {
      p
    };

    if let Some(mut v) = bt.remove(&target) {
      v.push(p);
      if v.len() == k {
        for val in v {
          ans[val] = turn;
        }
      } else {
        bt.insert(p, v);
      }
    } else {
      if k == 1 {
        ans[p] = turn;
      } else {
        bt.insert(p, vec![p]);
      }
    }
  }

  for num in &ans[1..] {
    println!("{}", num)
  }
}
