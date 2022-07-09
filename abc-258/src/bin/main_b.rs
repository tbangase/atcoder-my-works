use proconio::{input, fastout, marker::*};

use std::time::Instant;

#[derive(Debug)]
struct Route {
    n: usize,
    x: usize,
    y: usize,
    direction: Direction
}

impl Route {
    pub fn new(n:usize, x: usize, y: usize) -> Vec<Self> {
        vec![
            Route { n, x , y , direction: Direction::Up },
            Route { n, x , y , direction: Direction::UpRight },
            Route { n, x , y , direction: Direction::Right },
            Route { n, x , y , direction: Direction::DownRight },
            Route { n, x , y , direction: Direction::Down },
            Route { n, x , y , direction: Direction::DownLeft },
            Route { n, x , y , direction: Direction::Left },
            Route { n, x , y , direction: Direction::UpLeft },
        ]
    }

    pub fn next(&mut self) {
        match self.direction {
            Direction::Up => self.up(),
            Direction::UpRight => self.up().right(),
            Direction::Right => self.right(),
            Direction::DownRight => self.down().right(),
            Direction::Down => self.down(),
            Direction::DownLeft => self.down().left(),
            Direction::Left => self.left(),
            Direction::UpLeft => self.up().left(),
        };
    }

    pub fn up(&mut self) -> &mut Self {
        if self.y == 0 {
            self.y = self.n - 1
        } else {
            self.y -= 1
        }
        self
    }

    pub fn down(&mut self) -> &mut Self {
        if self.y == self.n - 1 {
            self.y = 0 
        } else {
            self.y += 1
        }
        self
    }

    pub fn right(&mut self) -> &mut Self {
        if self.x == self.n - 1 {
            self.x = 0 
        } else {
            self.x += 1
        }
        self
    }

    pub fn left(&mut self) -> &mut Self {
        if self.x == 0 {
            self.x = self.n - 1 
        } else {
            self.x -= 1
        }
        self
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [Chars; n],
    }

    let start = Instant::now();

    let a = a.iter().map(|v|{
        v.iter().map(|v| {
            v.to_digit(10).unwrap() as usize
        }).collect::<Vec<usize>>()
    }).collect::<Vec<Vec<usize>>>();

    // Choose start position
    let mut max = 0;
    let mut routes = vec![];
    for (i, row) in a.iter().enumerate() {
        for (j, a_i_j) in row.iter().enumerate() {
            if *a_i_j > max {
                routes.clear();
                routes.extend(Route::new(n, i, j));
                max = *a_i_j;
            } else if *a_i_j == max {
                routes.extend(Route::new(n, i, j))
            }
        }
    }

    let mut sequence = vec![0; n];
    sequence[0] = max;

    for target in sequence.iter_mut() {
        let mut max = 0;


        for route in routes.iter() {
            let val = a[route.x][route.y];
            if max < val {
                max = val;
            }
        }

        routes.retain(|route| {
            a[route.x][route.y] >= max
        });
        *target = max;
        for route in routes.iter_mut() {
            route.next();
        }
    }

    for i in sequence {
        print!("{}", i);
    }
    println!("");

    let duration = start.elapsed();
    println!("Duration: {:?}", duration);
}
