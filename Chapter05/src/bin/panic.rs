fn checked_divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("division by zero");
    }
    a / b
}

fn compute() {
    let _ = checked_divide(10, 0);
}

fn main() {
    compute();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checked_divide_works_for_non_zero_divisor() {
        assert_eq!(checked_divide(10, 2), 5);
    }

    #[test]
    #[should_panic(expected = "division by zero")]
    fn checked_divide_panics_on_zero() {
        checked_divide(10, 0);
    }
}
