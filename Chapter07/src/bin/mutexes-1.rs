use std::sync::Mutex;

/// Locks a `Mutex<i32>` once and returns the guarded value plus one.
pub fn add_one_to_mutex(value: i32) -> i32 {
    let mutexed_number = Mutex::new(value);
    let number = mutexed_number.lock().unwrap();
    *number + 1
}

fn main() {
    println!("Mutexed number plus one equals {}", add_one_to_mutex(5));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_one_to_positive() {
        assert_eq!(add_one_to_mutex(5), 6);
    }

    #[test]
    fn add_one_to_zero() {
        assert_eq!(add_one_to_mutex(0), 1);
    }

    #[test]
    fn add_one_to_negative() {
        assert_eq!(add_one_to_mutex(-5), -4);
    }
}
