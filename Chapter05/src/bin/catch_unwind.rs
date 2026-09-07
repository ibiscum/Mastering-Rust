use std::panic;

fn main() {
    panic::catch_unwind(|| {
        panic!("Panicing!");

    }).ok();
    println!("Survived that panic.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catch_unwind_catches_panic() {
        let result = panic::catch_unwind(|| {
            panic!("Panicing!");
        });
        assert!(result.is_err());
    }
}
