pub fn f2(y: u8) -> u8 {
    let x = 2 + y;
    x
}

pub fn f1(x: u8) -> u8 {
    let z = f2(5);
    z + x
}

fn main() {
    println!("f1(9) is {}", f1(9));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn f2_adds_two() {
        assert_eq!(f2(5), 7);
    }

    #[test]
    fn f1_composes_f2() {
        assert_eq!(f1(9), 16);
    }
}
