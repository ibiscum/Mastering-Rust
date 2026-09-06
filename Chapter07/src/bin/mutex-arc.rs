use std::sync::Mutex;
use std::thread;
use std::sync::Arc;
use std::time;

const THREADS: u64 = 1_000_000;
const START_NUMBER: u64 = 1;

pub fn expected_final_value() -> u64 {
    START_NUMBER + THREADS - 1
}

fn main() {
    let one_millisecond = time::Duration::from_millis(1);
    let one_second = time::Duration::from_millis(1000);

    let mutexed_number = Arc::new(Mutex::new(START_NUMBER));
    let mutexed_number_2 = mutexed_number.clone();

    thread::spawn(move || {
        for _ in 1..THREADS {
            let mutexed_number_clone = mutexed_number.clone();
            thread::spawn(move || {
                thread::sleep(one_millisecond);
                let mut number = mutexed_number_clone.lock().unwrap();
                *number += 1;
            });
        }
    });

    loop {
        thread::sleep(one_second);
        let number = mutexed_number_2.lock().unwrap();
        if *number != expected_final_value() {
            println!("Not there yet, number is {}", *number);
        } else {
            println!("Got there! Number is {}", *number);
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expected_final_value_is_correct() {
        assert_eq!(expected_final_value(), 1_000_000);
    }

    #[test]
    fn shared_mutex_increments_to_expected() {
        let counter = Arc::new(Mutex::new(0u64));
        let mut handles = Vec::new();
        for _ in 0..100 {
            let c = counter.clone();
            handles.push(thread::spawn(move || {
                let mut n = c.lock().unwrap();
                *n += 1;
            }));
        }
        for h in handles {
            let _ = h.join();
        }
        assert_eq!(*counter.lock().unwrap(), 100);
    }
}
