use std::io::Read;
use std::path::Path;
use std::fs::File;

fn main() {
    let path = Path::new("data.txt");
    let mut file = File::open(&path)
        .expect("Error while opening data.txt");

    let mut s = String::new();
    file.read_to_string(&mut s)
        .expect("Error while reading file contents");

    println!("Read the string: {}", s);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Error while opening data.txt")]
    fn unwrapping_missing_file_panics() {
        let path = Path::new("definitely-missing-file.txt");
        let _file = File::open(&path).expect("Error while opening data.txt");
    }
}
