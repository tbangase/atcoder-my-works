use proconio::{input, fastout};

use std::collections::{HashMap, HashSet};
// use std::{time::Instant, collections::{HashMap, HashSet}};

#[fastout]
fn main() {
    input! {
        (n, k): (usize, usize),
        s_list: [String; n],
    }

    // let start = Instant::now();

    // DFS
    // 必要なデータ
    // 到達した文字種
    // 登場した文字
    let char_count = HashMap::new();
    let reached_chars = HashSet::new();
    let ans = find(&s_list[..], char_count, reached_chars, 0, k);

    println!("{}", ans);
    // let duration = start.elapsed();
    // println!("Duration: {:?}", duration);
}

fn find(
    s_list: &[String],
    mut char_count: HashMap<char, usize>,
    mut reached_chars: HashSet<char>,
    index: usize,
    x: usize,
) -> usize {
    // println!("index: {}", index);
    // println!("Char Count: {:?}", char_count);
    // println!("Reached Chars: {:?}", reached_chars);
    // println!("Target String: {}\n", s_list[index]);
    // 最後の文字列だったら終わらせる
    if s_list.len() - 1 == index {
        // 採用しない場合と採用する場合に分ける
        let unused_count = reached_chars.len();
        add_string(s_list[index].as_str(), &mut char_count, &mut reached_chars, x);
        // println!("Used Char Count: {:?}", char_count);
        // println!("Used Reached Chars: {:?}\n", reached_chars);
        let used_count = reached_chars.len();
        return used_count.max(unused_count)
    }
    // 採用しない場合
    let unused_result = find(s_list, char_count.clone(), reached_chars.clone(), index + 1, x);

    add_string(s_list[index].as_str(), &mut char_count, &mut reached_chars, x);
    
    // 採用する場合
    let used_result = find(s_list, char_count, reached_chars, index + 1, x);
    // println!("Unused Result: {}", unused_result);
    // println!("Used Result  : {}\n", used_result);
    unused_result.max(used_result)
} 

fn add_string(
    s: &str,
    char_count: &mut HashMap<char, usize>,
    reached_chars: &mut HashSet<char>,
    x: usize,
) {
    for c in s.chars() {
        let count = char_count.entry(c).or_insert(0);
        *count += 1;
        if *count == x {
            reached_chars.insert(c);
        } else {
            reached_chars.remove(&c);
        }
    }
}