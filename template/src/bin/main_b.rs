use proconio::{input, fastout, marker::*};

use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    let mut alphabet_set = HashSet::new();
    let mut unique = true;
    let mut has_uppercase = false;
    let mut has_lowercase = false;

    for c in s {
        if alphabet_set.contains(&c) {
            unique = false;
            break
        };
        if c.is_uppercase() {
            has_uppercase = true;
        };
        if c.is_lowercase() {
            has_lowercase = true;
        };
        alphabet_set.insert(c);
    }

    if unique && has_lowercase && has_uppercase {
        println!("Yes")
    } else {
        println!("No")
    }
}
