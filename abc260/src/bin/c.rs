use proconio::input;

#[derive(Debug)]
struct Asset {
    pub level: usize,
    pub count: u64,
}

impl Asset {
    pub fn new(level: usize, count: u64) -> Self {
        Asset { level, count }
    }

    pub fn add(&mut self, other: Self) {
        if self.level != other.level {
            panic!("Miss match level.");
        }
        self.count += other.count;
    }
}

fn main() {
    input! {
        n: usize,
        (x, y): (u64, u64)
    }

    let exchange_red = |red: &Asset| {
        (Asset::new(red.level - 1, red.count), Asset::new(red.level, red.count * x))
    };

    let exchange_blue = |blue: &Asset | {
        (Asset::new(blue.level - 1, blue.count), Asset::new(blue.level - 1, blue.count * y))
    };

    let mut red_asset = Asset::new(n, 1);
    let mut blue_asset = Asset::new(n, 0);

    loop {
        // exchange_red
        if blue_asset.level <= 1 || red_asset.level <= 1{
            break;
        }

        let (red_from_red, blue_from_red) = exchange_red(&red_asset);

        red_asset = red_from_red;
        blue_asset.add(blue_from_red);

        // exchange_blue
        let (red_from_blue, blue_from_blue) = exchange_blue(&blue_asset);

        red_asset.add(red_from_blue);
        blue_asset = blue_from_blue;

    }

    println!("{:?}", blue_asset.count);
}
