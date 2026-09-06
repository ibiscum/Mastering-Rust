fn select_first<T>(p1: T, _: T) -> T {
    p1
}

fn main() {
    let x = 1;
    let y = 2;

    let a = "meep";
    let b = "moop";

    println!("Selected first: {}", select_first(x, y));
    println!("Selected first: {}", select_first(a, b));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn select_first_returns_first_argument() {
        assert_eq!(select_first(1, 2), 1);
        assert_eq!(select_first("meep", "moop"), "meep");
    }
}
