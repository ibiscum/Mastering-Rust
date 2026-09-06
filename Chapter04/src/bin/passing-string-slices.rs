fn hello_for(to_whom: &str) -> String {
    format!("Hey {}!", to_whom)
}

fn say_hello(to_whom: &str) {
    println!("{}", hello_for(to_whom))
}

fn main() {
    let string_slice: &'static str = "you";
    let string: String = string_slice.into();
    say_hello(string_slice);
    say_hello(&string);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_for_formats_correctly() {
        assert_eq!(hello_for("you"), "Hey you!");
    }
}
