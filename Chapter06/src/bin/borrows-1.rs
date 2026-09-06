// Demonstrates that a function cannot return a reference to a local variable
// because the local is dropped when the function returns. We return an owned
// value instead.
fn get_an_owned_value() -> u8 {
    let x = 1;
    x
}

fn main() {
    let value = get_an_owned_value();
    println!("value = {}", value);
}
