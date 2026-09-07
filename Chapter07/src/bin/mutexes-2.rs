use std::sync::Mutex;

/// Demonstrates that a `Mutex` can be locked multiple times in sequence once
/// the previous guard has been dropped.
fn main() {
    let mutexed_number = Mutex::new(5);

    {
        let number = mutexed_number.lock().unwrap();
        println!("1 Mutexed number plus one equals {}", *number + 1);
    }
    let number = mutexed_number.lock().unwrap();
    println!("2 Mutexed number plus one equals {}", *number + 1);
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    #[test]
    fn mutex_can_be_relocked_after_guard_drops() {
        let mutexed_number = Mutex::new(5);

        {
            let number = mutexed_number.lock().unwrap();
            assert_eq!(*number + 1, 6);
        }

        let number = mutexed_number.lock().unwrap();
        assert_eq!(*number + 1, 6);
    }
}
