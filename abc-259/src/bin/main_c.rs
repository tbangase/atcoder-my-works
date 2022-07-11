use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        s: String,
        t: String
    }

    let mut s_iter = s.chars();
    let mut t_iter = t.chars();

    let mut prev_s = ' ';

    let mut res = true;
    let mut cont = 0;

    'a: loop {
        if let Some(tc) = t_iter.next() {
            if let Some(sc) = s_iter.next() {
                if tc != sc {
                    if cont >= 2 {
                        while let Some(tc_ext) = t_iter.next() {
                            if tc_ext == sc {
                                break
                            }
                            if tc_ext != tc {
                                if tc_ext == sc {
                                    break
                                } else {
                                    res = false;
                                    break 'a
                                }
                            }
                        }
                    } else {
                        res = false;
                        break
                    }
                }
                if prev_s == sc {
                    cont += 1;
                } else {
                    cont = 1;
                }
                prev_s = sc;
            } else {
                if cont < 2 {
                    res = false;
                    break
                }
                while let Some(tc) = t_iter.next() {
                    if tc != prev_s {
                        res = false;
                        break 'a
                    }
                }
            }
        } else {
            if let Some(_sc) = s_iter.next() {
                res = false;
                break;
            } else {
                break;
            }
        }
    }
    
    match res {
        true => println!("Yes"),
        false => println!("No"),
    }
}
