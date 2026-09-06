fn main() {
    let mut empty_string = String::new();
    let empty_string_with_capacity = String::with_capacity(50);
    let string_from_bytestring: String = String::from_utf8(vec![82, 85, 83, 84]).expect("Creating String from bytestring failed");

    println!("Length of the empty string is {}", empty_string.len());
    println!("Length of the empty string with capacity is {}", empty_string_with_capacity.len());
    println!("Length of the string from a bytestring is {}", string_from_bytestring.len());

    println!("Bytestring says {}", string_from_bytestring);

    empty_string.push('1');
    println!("1) Empty string now contains {}", empty_string);
    empty_string.push_str("2345");
    println!("2) Empty string now contains {}", empty_string);
    println!("Length of the previously empty string is now {}", empty_string.len());
}

#[cfg(test)]
mod tests {
    #[test]
    fn string_operations_behave_as_expected() {
        let mut s = String::new();
        assert_eq!(s.len(), 0);
        s.push('1');
        s.push_str("2345");
        assert_eq!(s, "12345");
        assert_eq!(s.len(), 5);
        let from_bytes = String::from_utf8(vec![82, 85, 83, 84]).unwrap();
        assert_eq!(from_bytes, "RUST");
        assert_eq!(from_bytes.len(), 4);
    }
}
