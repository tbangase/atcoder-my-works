use proconio::{input, fastout, marker::*};

#[fastout]
fn main() {
    input! {
        _n: usize,
        t: Chars,
    }

    let mut direction = Direction::new();
    let mut position: (i32, i32) = (0,0);

    for command in t {
        match command {
            'S' => {
                match direction {
                    Direction::East => position.0 += 1,
                    Direction::South => position.1 -= 1,
                    Direction::West => position.0 -= 1,
                    Direction::North => position.1 += 1,
                }
            },
            'R' => {
                direction.next()
            },
            _ => return 
        }
    }

    println!("{} {}", position.0, position.1);
}

enum Direction {
    North,
    East,
    South,
    West
}

impl Direction {
    pub fn new() -> Self {
        Direction::East
    }

    pub fn next(&mut self) {
        match self {
            &mut Direction::East => *self = Direction::South,
            &mut Direction::South => *self = Direction::West,
            &mut Direction::West => *self = Direction::North,
            &mut Direction::North => *self = Direction::East,
        }
    }
}
