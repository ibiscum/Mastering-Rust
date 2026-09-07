use std::string::FromUtf8Error;

fn bytestring_to_string_with_match(str: Vec<u8>) -> Result<String, FromUtf8Error> {
    let ret = match String::from_utf8(str) {
        Ok(str) => str.to_uppercase(),
        Err(err) => return Err(err)
    };

    println!("Conversion succeeded: {}", ret);
    Ok(ret)
}

fn bytestring_to_string_with_try(str: Vec<u8>) -> Result<String, FromUtf8Error> {
    let ret = String::from_utf8(str)?;
    println!("Conversion succeeded: {}", ret);
    Ok(ret)
}

#[allow(dead_code)]
fn bytestring_to_string_with_qmark(str: Vec<u8>) -> Result<String, FromUtf8Error> {
    let ret = String::from_utf8(str)?;
    println!("Conversion succeeded: {}", ret);
    Ok(ret)
}

fn main() {
    let faulty_bytestring = vec!(130, 131, 132, 133);
    let ok_bytestring = vec!(80, 82, 84, 85, 86);

    let s1_faulty = bytestring_to_string_with_match(faulty_bytestring.clone());
    let s1_ok = bytestring_to_string_with_match(ok_bytestring.clone());

    let s2_faulty = bytestring_to_string_with_try(faulty_bytestring.clone());
    let s2_ok = bytestring_to_string_with_try(ok_bytestring.clone());

    println!("Read the string: {:?}", s1_faulty);
    println!("Read the string: {:?}", s1_ok);
    println!("Read the string: {:?}", s2_faulty);
    println!("Read the string: {:?}", s2_ok);

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bytestring_to_string_with_match_uppercases_valid_utf8() {
        let bytes = vec!(80, 82, 84, 85, 86);
        assert_eq!(bytestring_to_string_with_match(bytes).unwrap(), "PRTUV");
    }

    #[test]
    fn bytestring_to_string_with_match_returns_err_for_invalid_utf8() {
        let bytes = vec!(130, 131, 132, 133);
        assert!(bytestring_to_string_with_match(bytes).is_err());
    }

    #[test]
    fn bytestring_to_string_with_try_uppercases_valid_utf8() {
        let bytes = vec!(80, 82, 84, 85, 86);
        assert_eq!(bytestring_to_string_with_try(bytes).unwrap(), "PRTUV");
    }

    #[test]
    fn bytestring_to_string_with_try_returns_err_for_invalid_utf8() {
        let bytes = vec!(130, 131, 132, 133);
        assert!(bytestring_to_string_with_try(bytes).is_err());
    }

    #[test]
    fn bytestring_to_string_with_qmark_uppercases_valid_utf8() {
        let bytes = vec!(80, 82, 84, 85, 86);
        assert_eq!(bytestring_to_string_with_qmark(bytes).unwrap(), "PRTUV");
    }

    #[test]
    fn bytestring_to_string_with_qmark_returns_err_for_invalid_utf8() {
        let bytes = vec!(130, 131, 132, 133);
        assert!(bytestring_to_string_with_qmark(bytes).is_err());
    }
}
