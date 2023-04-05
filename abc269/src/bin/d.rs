use im_rc::HashMap;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        blacks: [(i32, i32); n],
    }

    let mut searched = blacks
        .iter()
        .map(|block| (block.clone(), false))
        .collect::<HashMap<_, _>>();

    let mut connects = 0;

    for block in blacks.iter() {
        if Some(&false) == searched.get(block) {
            connects += 1;
            search(&mut searched, block);
        }
    }

    println!("{:?}", connects);
}

fn search(searched: &mut HashMap<(i32, i32), bool>, block: &(i32, i32)) {
    for candidate in candidates(block) {
        let next = searched.get_mut(&candidate).and_then(|v| {
            if *v == false {
                *v = true;
                return Some(());
            }
            None
        });

        if next.is_some() {
            search(searched, &candidate);
        }
    }
}

fn candidates(&(x, y): &(i32, i32)) -> Vec<(i32, i32)> {
    vec![
        ((x - 1), (y - 1)),
        (x, (y - 1)),
        ((x - 1), y),
        ((x + 1), y),
        (x, (y + 1)),
        ((x + 1), (y + 1)),
    ]
}
