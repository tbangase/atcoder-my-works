use proconio::{fastout, input};

enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
}

impl From<String> for Weekday {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Monday" => Weekday::Monday,
            "Tuesday" => Weekday::Tuesday,
            "Wednesday" => Weekday::Wednesday,
            "Thursday" => Weekday::Thursday,
            "Friday" => Weekday::Friday,
            _ => panic!("Invalid input"),
        }
    }
}

#[fastout]
fn main() {
    input! {
        s: String,
    }

    let weekday = Weekday::from(s);

    println!("{}", 5 - weekday as usize);
}
