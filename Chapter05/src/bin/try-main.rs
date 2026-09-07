fn run() -> Result<(), ()> {
    let empty_ok_value = Ok(());
    empty_ok_value?;
    Ok(())
}

fn main() {
    run().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_returns_ok() {
        assert!(run().is_ok());
    }
}
