use std::thread;

/// Spawns a thread that takes ownership of a value from the surrounding scope
/// via a `move` closure and returns the string it received.
pub fn spawn_worker(value: String) -> thread::JoinHandle<String> {
    thread::spawn(move || {
        println!("Inside thread with string '{}'", value);
        value
    })
}

fn main() {
    let outside_string = String::from("outside");
    let handle = spawn_worker(outside_string);
    let _ = handle.join();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn worker_returns_input_string() {
        let handle = spawn_worker(String::from("test"));
        let result = handle.join().unwrap();
        assert_eq!(result, "test");
    }
}
