use std::fmt::Debug;

const FOO: u8 = 4;
fn print_slice<T: Debug>(slice: &[T]) {
    println!("{:?}", slice);
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let panic_on_oob = args.iter().any(|a| a == "panic-on-out-of-bounds");

    let array: [u8; 5] = [1, 2, 3, 4, 5];

    print!("Whole array just borrowed: ");
    print_slice(&array);

    print!("Whole array sliced: ");
    print_slice(&array[..]);

    print!("Without the first element: ");
    print_slice(&array[1..]);

    print!("One element from the middle: ");
    print_slice(&array[3..4]);

    print!("First three elements: ");
    print_slice(&array[..3]);

    if panic_on_oob {
        print!("Oops, going too far!: ");
        print_slice(&array[..900]);
    }

    let z = &FOO;
    let g = &FOO;
    println!("{} {}", &z, g);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slices_match_expected() {
        let array: [u8; 5] = [1, 2, 3, 4, 5];
        assert_eq!(&array[..], &[1, 2, 3, 4, 5]);
        assert_eq!(&array[1..], &[2, 3, 4, 5]);
        assert_eq!(&array[3..4], &[4]);
        assert_eq!(&array[..3], &[1, 2, 3]);
    }

    #[test]
    fn const_reference_has_expected_value() {
        assert_eq!(FOO, 4);
        let z = &FOO;
        let g = &FOO;
        assert_eq!(*z, 4);
        assert_eq!(*g, 4);
    }

    #[test]
    #[should_panic(expected = "range end index 900 out of range")]
    fn out_of_bounds_slice_panics() {
        let array: [u8; 5] = [1, 2, 3, 4, 5];
        let _ = &array[..900];
    }
}
