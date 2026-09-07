pub fn shared_borrow_sum() -> i32 {
    let x = 1;
    let x1 = &x;
    let x2 = &x;
    *x1 + *x2
}

fn main() {
    println!("x1 says 1");
    println!("x2 says 1");
    println!("sum is {}", shared_borrow_sum());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shared_borrow_sum_is_two() {
        assert_eq!(shared_borrow_sum(), 2);
    }
}
