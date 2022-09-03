use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        (height, width): (usize, usize),
        g: [String; height],
    }

    let mut g = Game::new(height, width, g);
    let mut status;

    while {
        status = g.next();
        match status {
            Status::KeepGoing => true,
            Status::Finished => false,
            Status::Looping => false,
        }
    } {}

    match status {
        Status::Finished => {
            println!(
                "{} {}",
                g.current_position().0 + 1,
                g.current_position().1 + 1
            );
        }
        Status::Looping => {
            println!("-1");
        }
        _ => panic!("Unreachable Error"),
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

enum Status {
    KeepGoing,
    Finished,
    Looping,
}

struct Game {
    height: usize,
    width: usize,
    indication_map: Vec<Vec<Direction>>,
    current_position: (usize, usize),
    visited: HashSet<(usize, usize)>,
}

impl Game {
    pub fn new(height: usize, width: usize, g: Vec<String>) -> Self {
        let map = g
            .iter()
            .map(|row| {
                row.chars()
                    .map(|c| match c {
                        'U' => Direction::Up,
                        'D' => Direction::Down,
                        'L' => Direction::Left,
                        'R' => Direction::Right,
                        _ => panic!("Invalid character: {}", c),
                    })
                    .collect()
            })
            .collect();
        let mut visited = HashSet::new();
        visited.insert((0, 0));
        Self {
            height,
            width,
            indication_map: map,
            current_position: (0, 0),
            visited,
        }
    }

    pub fn next(&mut self) -> Status {
        let (r, c) = self.current_position;
        let next_position = match self.indication_map[r][c] {
            Direction::Up => {
                if r == 0 {
                    return Status::Finished;
                }
                (r - 1, c)
            }
            Direction::Down => {
                if r == self.height - 1 {
                    return Status::Finished;
                }
                (r + 1, c)
            }
            Direction::Left => {
                if c == 0 {
                    return Status::Finished;
                }
                (r, c - 1)
            }
            Direction::Right => {
                if c == self.width - 1 {
                    return Status::Finished;
                }
                (r, c + 1)
            }
        };
        self.current_position = next_position;

        if self.visited.get(&next_position).is_some() {
            return Status::Looping;
        }
        self.visited.insert(next_position);

        Status::KeepGoing
    }

    pub fn current_position(&self) -> (usize, usize) {
        self.current_position
    }
}
