use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        _n: usize,
        mut s: String,
        q: usize,
        queries: [(char, char); q],
    }

    let mut to_chars = ('a'..='z').collect::<Vec<_>>();

    queries.iter().for_each(|(from, to)| {
        to_chars = to_chars
            .iter()
            .map(|c| if *c == *from { *to } else { *c })
            .collect::<Vec<_>>();
    });

    s = s
        .chars()
        .map(|c| to_chars[(c as u8 - b'a') as usize])
        .collect::<String>();

    println!("{}", s);
}

// use proconio::{fastout, input};

// #[fastout]
// fn main() {
//     input! {
//         _n: usize,
//         mut s: String,
//         q: usize,
//         cds: [(char, char); q],
//     }

//     for (c, d) in cds {
//         s = s.chars().map(|x| if x == c { d } else { x }).collect();
//     }

//     println!("{}", s);
// }
