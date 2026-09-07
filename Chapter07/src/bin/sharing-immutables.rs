use std::thread;
use std::time;

pub fn worker_count() -> usize {
    9
}

fn main() {
    let num = Arc::new(Mutex::new(4));

    for _ in 1..10 {
        let num = num.clone();
        thread::spawn(move || {
            let mut n = num.lock().unwrap();
            *n += 1;
            println!("String is {}", *n);
        });
    }

    thread::sleep(time::Duration::from_secs(1));
    println!("In main thread: num is now {}", *num.lock().unwrap());
}

use std::sync::{Arc, Mutex};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn worker_count_is_nine() {
        assert_eq!(worker_count(), 9);
    }

    #[test]
    fn shared_counter_reaches_expected() {
        let num = Arc::new(Mutex::new(4));
        let mut handles = Vec::new();
        for _ in 0..worker_count() {
            let num = num.clone();
            handles.push(thread::spawn(move || {
                let mut n = num.lock().unwrap();
                *n += 1;
            }));
        }
        for h in handles {
            let _ = h.join();
        }
        assert_eq!(*num.lock().unwrap(), 4 + worker_count() as i32);
    }
}
