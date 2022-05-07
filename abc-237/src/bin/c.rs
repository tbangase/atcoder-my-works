use proconio::{input, fastout, marker::*};

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    let mut left = 0;
    let mut right = s.len() - 1;

    let mut only_a = true;
    let mut can_be_palindrome = true;
    
    while left < right {
        if s[left] == s[right] {
            if s[left] != 'a' {
                only_a = false;
            }
            left += 1;
            right -= 1;
            continue;
        }

        if only_a == true && s[right] == 'a' {
            right -= 1;
            continue;
        }

        can_be_palindrome = false;
        break;
    }

    match can_be_palindrome {
        true => println!("Yes"),
        false => println!("No"),
    }
}
