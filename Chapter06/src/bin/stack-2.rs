// Demonstrates that returning a reference to a local variable is not allowed.
// The reference would outlive the value it points to, so we return an owned u8.
fn f1() -> u8 {
    let x = 4;
    x
}

fn main() {
    let f1s_x = f1();
    println!("f1 returned {}", f1s_x);
}
