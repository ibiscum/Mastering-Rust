use std::io::Read;
use std::path::Path;
use std::fs::File;

fn read_and_print(path: &Path) {
    let file = File::open(path);
    let mut s = String::new();
    match file {
        Ok(mut f) => {
            f.read_to_string(&mut s).ok();
            println!("Read the string: {}", s);
        }
        Err(e) => println!("Could not open {:?}: {}", path, e),
    }
}

fn main() {
    let path = Path::new("data.txt");
    read_and_print(path);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_file_yields_error() {
        let path = Path::new("definitely-missing-file.txt");
        assert!(File::open(&path).is_err());
    }
}
