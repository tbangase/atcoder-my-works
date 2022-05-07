use proconio::{input, fastout, marker::*};

use std::collections::HashMap;

#[derive(Debug)]
struct Point {
    x: usize,
    direction: Direction
}

impl Point {
    pub fn x(&self) -> usize {
        self.x
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}

#[derive(Debug)]
enum Direction {
    Left,
    Right
}

#[fastout]
fn main() {
    input! {
        n: usize,
        point: [(usize, usize); n],
        s: Chars,
    }

    let mut points_by_height: HashMap<usize, Vec<Point>> = HashMap::new();

    for (i, (x, y)) in point.iter().enumerate() {
        match points_by_height.get_mut(y) {
            Some(list) => {
                if s[i] == 'R' {
                    list.push(Point { x: *x, direction: Direction::Right });
                } else {
                    list.push(Point { x: *x, direction: Direction::Left });
                }
            },
            None => {
                if s[i] == 'R' {
                    points_by_height.insert(*y, vec![Point { x: *x, direction: Direction::Right }]);
                } else {
                    points_by_height.insert(*y, vec![Point { x: *x, direction: Direction::Left }]);
                }
            }
        }
    }

    for (_height, points) in points_by_height {
        let mut righter = None;
        let mut lefter = None;

        for point in points {
            match point.direction() {
                Direction::Right => {
                    match righter {
                        Some(x) => {
                            if x > point.x() {
                                righter = Some(point.x())
                            }
                        },
                        None => {
                            righter = Some(point.x())
                        }
                    }
                }, 
                Direction::Left => {
                    match lefter {
                        Some(x) => {
                            if x < point.x() {
                                lefter = Some(point.x())
                            }
                        },
                        None => {
                            lefter = Some(point.x())
                        }
                    }
                }
            }

            if check_collision(&righter, &lefter) {
                println!("Yes");
                return;
            }
        } 
    }

    println!("Yes");
}

fn check_collision(righter: &Option<usize>, lefter: &Option<usize>) -> bool {
    if let Some(righter_x) = righter {
        if let Some(lefter_x) = lefter {
            return righter_x < lefter_x
        }
    }
    false
}
