#![allow(non_snake_case)]
use std::{fmt, io::BufRead, str::FromStr};

struct InputScanner<R: BufRead> {
    reader: R,
    buf: Vec<u8>, // Should never be empty
    pos: usize,   // Should never be out of bounds as long as the input ends with '\n'
}

impl<R: BufRead> InputScanner<R> {
    fn new(reader: R, capacity: usize) -> Self {
        InputScanner {
            reader,
            buf: Vec::with_capacity(capacity),
            pos: 0,
        }
    }

    #[inline]
    fn read<T: FromStr>(&mut self) -> T
    where
        T::Err: fmt::Debug,
    {
        if self.buf.is_empty() {
            self._read_next_line();
        }
        let mut start = None;
        loop {
            match (self.buf[self.pos], start.is_some()) {
                (b' ', true) | (b'\n', true) => break,
                (_, true) | (b' ', false) => self.pos += 1,
                (b'\n', false) => self._read_next_line(),
                (_, false) => start = Some(self.pos),
            }
        }
        let target = &self.buf[start.unwrap()..self.pos];
        unsafe { std::str::from_utf8_unchecked(target) }
            .parse()
            .unwrap()
    }
    #[allow(dead_code)]
    fn read_line(&mut self) -> String {
        self._read_next_line();
        let res = unsafe { std::str::from_utf8_unchecked(&self.buf) }
            .trim_end_matches('\n')
            .to_string();
        self.buf.clear();
        res
    }

    #[inline]
    fn _read_next_line(&mut self) {
        self.pos = 0;
        self.buf.clear();
        if self.reader.read_until(b'\n', &mut self.buf).unwrap() == 0 {
            panic!("Reached EOF");
        }
    }
}
macro_rules! join {
    ($iter:expr) => {
        $iter.map(|x| x.to_string()).collect::<Vec<_>>().join(" ")
    };
    ($iter:expr, sep: expr) => {
        $iter.map(|x| x.to_string()).collect::<Vec<_>>().join(sep)
    };
}

fn main() {
    use std::io::{stdout, BufWriter, Write};
    let out = stdout();
    let mut out = BufWriter::new(out.lock());
    let stdin = std::io::stdin();
    let mut scan = InputScanner::new(stdin.lock(), 100);
    let N: usize = scan.read();
    let Q: usize = scan.read();
    let mut train: Vec<_> = (0..N).map(|i| (i, i)).collect();
    for _ in 0..Q {
        match scan.read::<usize>() {
            1 => {
                let x = scan.read::<usize>() - 1;
                let y = scan.read::<usize>() - 1;
                train[x].1 = y;
                train[y].0 = x;
            }
            2 => {
                let x = scan.read::<usize>() - 1;
                let y = scan.read::<usize>() - 1;
                train[x].1 = x;
                train[y].0 = y;
            }
            3 => {
                let mut idx = scan.read::<usize>() - 1;
                while train[idx].0 != idx {
                    idx = train[idx].0;
                }
                idx = train[idx].0;
                let mut ans = vec![];
                while train[idx].1 != idx {
                    ans.push(idx + 1);
                    idx = train[idx].1;
                }
                ans.push(idx + 1);
                writeln!(out, "{} {}", ans.len(), join!(ans.iter())).unwrap();
            }
            _ => unreachable!(),
        }
    }
}
