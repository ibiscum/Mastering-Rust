use std::sync::Mutex;
use std::thread;
use std::sync::Arc;

/// Demonstrates sharing a `Mutex` between threads using `Arc`. Despite the
/// original filename, `Rc` cannot be used here because `Rc` is not `Send`.
pub fn shared_mutex_add_one(value: i32) -> i32 {
    let mutexed_number = Arc::new(Mutex::new(value));
    let mutexed_number_clone = mutexed_number.clone();

    let handle = thread::spawn(move || {
        let number = mutexed_number_clone.lock().unwrap();
        *number + 1
    });

    handle.join().unwrap()
}

fn main() {
    println!(
        "1 Arc/Mutexed number plus one equals {}",
        shared_mutex_add_one(5)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shared_mutex_adds_one() {
        assert_eq!(shared_mutex_add_one(5), 6);
    }

    #[test]
    fn shared_mutex_adds_one_to_zero() {
        assert_eq!(shared_mutex_add_one(0), 1);
    }
}
