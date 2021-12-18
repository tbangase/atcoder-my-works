use proconio::{input, fastout};

// use std::time::Instant;
use std::fmt;
use std::cmp::Ordering;

#[derive(Debug, Clone)]
struct Order {
    index: i64,
    start: Position,
    destination: Position,
    distance: i64,
}

impl Order {
    pub fn init() -> Self {
        Order {
            index: -1,
            start: Position { x: 10000, y: 10000 },
            destination: Position { x: 20000, y: 20000 },
            distance: 20000
        }
    }
}

impl PartialOrd for Order {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.distance.partial_cmp(&other.distance)
    }
}


impl PartialEq for Order {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}


#[derive(Debug, Clone, Copy)]
struct Position {
    x: i64,
    y: i64
}

#[allow(dead_code)]
impl Position {
    pub fn distance(&self, pos: Position) -> i64 {
        (self.x - pos.x).abs() + (self.y - pos.y).abs()
    }

    pub fn is_in_the_area(&self, square: i64) -> bool {
        if self.x > 400 + square / 2 || self.x < 400 - square / 2 
        || self.y > 400 + square / 2 || self.y < 400 - square / 2 {
            return false
        }
        true
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.x, self.y)
    }
}

#[fastout]
fn main() {
    let mut order_list: Vec<Order> = vec![];

    for i in 0..1000 {
        input! {
            (a, b, c, d): (i64, i64, i64, i64)
        }
        let start = Position { x: a, y: b };
        let destination = Position { x: c, y: d };

        let area = 600;

        if start.is_in_the_area(area) && destination.is_in_the_area(area) {
            order_list.push(
                Order {
                    index: i,
                    start,
                    destination,
                    distance: start.distance(destination)
                }
            );
        }
    }

    // let start = Instant::now();

    let length = order_list.len();
    quick_sort(&mut order_list, 0, length - 1);

    let mut task_list: Vec<&Order> = vec![];

    // 高橋くん用に50個選ぶ
    let order_list: Vec<&Order> = order_list.iter().rev().collect();
    for i in 0..50 {
        task_list.push(order_list.get(i).unwrap());
    }

    deliver(&mut task_list);

    // let duration = start.elapsed();
    // println!("Duration: {:?}", duration);
}

fn deliver(orders: &mut Vec<&Order>) {
    print!("{}", orders.len());
    for i in 0..orders.len() {
        print!(" {}", orders[i].index + 1);
    }
    print!("\n");

    print!("{} ", orders.len() * 2 + 2);
    print!("{} {} ", 400, 400);
    // for i in 0..orders.len() {
    //     print!("{} {} ", orders[i].start, orders[i].destination);
    // }

    let mut current = Position {
        x: 400, y: 400,
    };
    while !orders.is_empty() {
        // 一番近くの配達を取得する
        let mut nearest = Order::init();
        let mut index = 100;

        for (i, order) in orders.iter().enumerate() {
            let nearest_distance = current.distance(nearest.start);
            let order_distance = current.distance(order.start);
            if order_distance < nearest_distance {
                nearest = (*order).clone();
                index = i;
            }
        }
        print!("{} {} ", nearest.start, nearest.destination);
        current = nearest.destination;
        orders.remove(index);
    }
    print!("{} {}", 400, 400);
    print!("\n");
}

fn quick_sort<T>(vector: &mut Vec<T>, left: usize, right: usize)
where
    T: PartialOrd + Clone,
{
    if left >= right {
        return;
    }
    // ソート対象のベクトルから3つ取ってきて中央値をPivotとする
    let pivot;
    {
        let pivot_vec = vec![
            vector[left].clone(),
            vector[(left + (right - 1)) / 2].clone(),
            vector[left].clone(),
        ];

        pivot = medium(pivot_vec);
    }
    let mut i = left;
    let mut j = right;

    loop {
        // Pivotより小さい値のインデックスを探す
        while vector[i] > pivot { i += 1 }
        // Pivotより大きい値のインデックスを探す
        while vector[j] < pivot { j -= 1 }

        if i >= j { break; }

        vector.swap(i, j);

        i += 1;
        j -= 1;
    }

    // i is the index of last searched under pivot data
    // In other words, left to i - 1 is over the pivot.
    if i != 0 {
        quick_sort(vector, left, i - 1);
    }
    // same logic as obove, j + 1 to right is under the pivot.
    quick_sort(vector, j + 1, right);
}

fn medium<T: PartialOrd + Clone>(list: Vec<T>) -> T {
    if list[0] > list[1] {
        if list[1] > list[2] {
            let val = list[1].clone();
            return val;
        } else {
            if list[0] > list[2] {
                return list[2].clone();
            } else {
                return list[0].clone();
            }
        }
    } else {
        if list[0] > list[2] {
            return list[0].clone();
        } else {
            if list[1] > list[2] {
                return list[2].clone();
            } else {
                return list[1].clone();
            }
        }
    }
}

