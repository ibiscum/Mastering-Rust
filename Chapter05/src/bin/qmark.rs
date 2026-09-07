use std::io::Read;
use std::path::Path;
use std::fs::File;

fn read_file_to_string(path: &Path) -> Result<String, Box<dyn std::error::Error>> {
    let mut s = String::new();
    File::open(path)?.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    let path = Path::new("data.txt");
    match read_file_to_string(path) {
        Ok(contents) => println!("Read the string: {}", contents),
        Err(e) => println!("Could not read {:?}: {}", path, e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_file_reports_error() {
        let path = Path::new("definitely-missing-file.txt");
        assert!(read_file_to_string(path).is_err());
    }
}
