fn main() {
    let mut integer_array_1 = [1, 2, 3];
    let integer_array_2: [u64; 3] = [2, 3, 4];
    let integer_array_3: [u64; 32] = [0; 32];
    let integer_array_4: [i32; 16438] = [-5; 16438];
    integer_array_1[1] = 255;

    println!("integer_array_1: {:?}", integer_array_1);
    println!("integer_array_2: {:?}", integer_array_2);
    println!("integer_array_3: {:?}", integer_array_3);
    println!("integer_array_4 length: {}", integer_array_4.len());
    println!("integer_array_1[0]: {}", integer_array_1[0]);
    println!("integer_array_1[2]: {}", integer_array_1[2]);
}

#[cfg(test)]
mod tests {
    #[test]
    fn arrays_behave_as_expected() {
        let mut integer_array_1 = [1, 2, 3];
        let integer_array_2: [u64; 3] = [2, 3, 4];
        integer_array_1[1] = 255;
        assert_eq!(integer_array_1, [1, 255, 3]);
        assert_eq!(integer_array_2, [2, 3, 4]);
        assert_eq!(integer_array_2.len(), 3);
    }
}
