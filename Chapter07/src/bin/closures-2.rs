/// Demonstrates how the `move` keyword forces a closure to take ownership of
/// its captured variables, leaving the original outer binding untouched.
fn main() {
    let mut outer_scope_x = 42;

    {
        let mut closure = move || {
            outer_scope_x += 42;
            println!("Outer scope variable is {}", outer_scope_x);
        };

        closure();
    }
    println!("Outer_scope_x {}", outer_scope_x);
}

#[cfg(test)]
mod tests {
    #[test]
    fn move_closure_copies_capture() {
        let mut outer_scope_x = 42;

        {
            let mut closure = move || {
                outer_scope_x += 42;
                outer_scope_x
            };

            assert_eq!(closure(), 84);
        }

        // The original variable is unchanged because the closure took ownership
        // of a copy of the integer.
        assert_eq!(outer_scope_x, 42);
    }
}
