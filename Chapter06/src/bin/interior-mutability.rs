use std::cell::Cell;

#[derive(Debug)]
pub struct Point {
    x: u8,
    y: u8,
    cached_sum: Cell<Option<u8>>
}

impl Point {
    pub fn new(x: u8, y: u8) -> Self {
        Point { x, y, cached_sum: Cell::new(None) }
    }

    pub fn sum(&self) -> u8 {
        match self.cached_sum.get() {
            Some(sum) => {
                println!("Got from cache: {}", sum);
                sum
            },
            None => {
                let new_sum = self.x + self.y;
                self.cached_sum.set(Some(new_sum));
                println!("Set cache: {}", new_sum);
                new_sum
            }
        }
    }
}

fn main() {
    let p = Point::new(8, 9);

    println!("Summed result: {}", p.sum());
    println!("Summed result: {}", p.sum());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_sum_computes_and_caches() {
        let p = Point::new(8, 9);
        assert_eq!(p.sum(), 17);
        assert_eq!(p.sum(), 17);
    }

    #[test]
    fn point_sum_zero() {
        let p = Point::new(0, 0);
        assert_eq!(p.sum(), 0);
    }
}
