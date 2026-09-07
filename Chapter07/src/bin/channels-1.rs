use std::thread;
use std::sync::mpsc;

/// Sends values through an asynchronous channel and collects them into a
/// sorted vector so the output is deterministic regardless of thread scheduling.
pub fn collect_channel_messages() -> Vec<i32> {
    let (tx, rx) = mpsc::channel();
    let tx_clone = tx.clone();
    let tx_clone_2 = tx.clone();
    let _ = tx.send(0);

    thread::spawn(move || {
        let _ = tx_clone.send(1);
    });

    thread::spawn(move || {
        let _ = tx_clone_2.send(2);
    });

    drop(tx);

    let mut received = Vec::new();
    while let Ok(value) = rx.recv() {
        received.push(value);
    }
    received.sort();
    received
}

fn main() {
    for value in collect_channel_messages() {
        println!("Received {} via the channel", value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collects_all_channel_messages() {
        assert_eq!(collect_channel_messages(), vec![0, 1, 2]);
    }
}
