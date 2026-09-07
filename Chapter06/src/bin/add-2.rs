pub fn add_one(a: u8) -> u8 {
   let g = a.wrapping_add(255);
   g
}

fn main() {
    let x = 1;
    let z = add_one(x);
    let _ = z;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_one_wraps() {
        assert_eq!(add_one(1), 0);
    }

    #[test]
    fn add_one_zero() {
        assert_eq!(add_one(0), 255);
    }
}
