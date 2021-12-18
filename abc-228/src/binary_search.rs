use std::cmp::Ordering;

pub trait BinarySearch<T> {
    fn lower_bound(&self, x:&T) -> usize;
    fn upper_bound(&self, x:&T) -> usize;
}

impl<T: Ord> BinarySearch<T> for [T] {
    fn lower_bound(&self, x:&T) -> usize {
        let mut left = 0;
        let mut right = self.len();

        while low != high {
            let mid = (left + right) / 2;
            match self[mid].cmp(x) {
                Ordering::Less => {
                    left = mid + 1;
                },
                Ordering::Equal | Ordering::Greater => {
                    right = mid;
                }
            }
        }
        left
    }
        
    fn upper_bound(&self, x:&T) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less | Ordering::Equal=> {
                    low = mid + 1;
                },
                Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }
}
