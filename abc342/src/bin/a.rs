use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
    }

    let mut map = std::collections::HashMap::new();

    s.chars().enumerate().for_each(|(i, c)| {
        map.entry(c)
            .and_modify(|(e, _i)| *e = false)
            .or_insert((true, i));
    });

    let res = map
        .iter()
        .filter_map(|(_, (e, i))| if *e { Some(*i) } else { None })
        .collect::<Vec<_>>();

    println!("{:?}", res.first().unwrap() + 1);
}
