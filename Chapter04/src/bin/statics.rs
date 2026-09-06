use std::sync::atomic::{AtomicU32, Ordering};

static MEEP: AtomicU32 = AtomicU32::new(4);
static FUUP: u8 = 9;

fn main() {
    println!("Meep is {}", MEEP.load(Ordering::SeqCst));
    MEEP.store(42, Ordering::SeqCst);
    println!("Meep is now {}", MEEP.load(Ordering::SeqCst));

    println!("Fuup is {}", FUUP);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constant_static_has_expected_value() {
        assert_eq!(FUUP, 9);
    }

    #[test]
    fn mutable_static_starts_at_expected_value() {
        assert_eq!(MEEP.load(Ordering::SeqCst), 4);
    }
}
