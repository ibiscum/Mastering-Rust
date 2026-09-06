pub fn square(x: u32) -> u32 {
    x * x
}

fn function_without_vars() {
    println!("Entered function without variables");
}

fn main() {
    let square_c1 = |x: u32|          x*x;
    let square_c2 = |x: u32|        { x*x };
    let square_c3 = |x: u32| -> u32 { x*x };

    let closure_without_vars = || println!("Entered closure without variables");

    println!("square of 4 = {}", square(4));
    println!("square of 4 = {}", square_c1(4));
    println!("square of 4 = {}", square_c2(4));
    println!("square of 4 = {}", square_c3(4));

    function_without_vars();
    closure_without_vars();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn square_zero() {
        assert_eq!(square(0), 0);
    }

    #[test]
    fn square_positive() {
        assert_eq!(square(4), 16);
        assert_eq!(square(5), 25);
    }

    #[test]
    fn square_large() {
        assert_eq!(square(1_000), 1_000_000);
    }
}
