use std::env;
use std::path::PathBuf;
use std::process::Command;

fn run_bin(name: &str) -> String {
    let env_var = format!("CARGO_BIN_EXE_{}", name);
    let path = env::var(&env_var).unwrap_or_else(|_| {
        let manifest = env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
        PathBuf::from(manifest)
            .join("target/debug")
            .join(name)
            .to_string_lossy()
            .to_string()
    });

    let output = Command::new(&path)
        .output()
        .unwrap_or_else(|e| panic!("failed to execute {}: {}", path, e));

    assert!(
        output.status.success(),
        "{} exited with {:?}\nstderr: {}",
        name,
        output.status,
        String::from_utf8_lossy(&output.stderr)
    );

    String::from_utf8(output.stdout)
        .unwrap_or_else(|_| panic!("{} produced invalid UTF-8", name))
}

#[test]
fn regression_array_slicing() {
    let expected = "Whole array just borrowed: [1, 2, 3, 4, 5]\n\
                    Whole array sliced: [1, 2, 3, 4, 5]\n\
                    Without the first element: [2, 3, 4, 5]\n\
                    One element from the middle: [4]\n\
                    First three elements: [1, 2, 3]\n\
                    4 4\n";
    assert_eq!(run_bin("array-slicing"), expected);
}

#[test]
fn regression_array_slicing_out_of_bounds_panics() {
    let env_var = format!("CARGO_BIN_EXE_{}", "array-slicing");
    let path = env::var(&env_var).unwrap_or_else(|_| {
        let manifest = env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
        PathBuf::from(manifest)
            .join("target/debug")
            .join("array-slicing")
            .to_string_lossy()
            .to_string()
    });
    let output = Command::new(&path)
        .arg("panic-on-out-of-bounds")
        .output()
        .unwrap();
    assert!(!output.status.success(), "expected binary to panic");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("range end index 900 out of range"),
        "unexpected stderr: {}",
        stderr
    );
}

#[test]
fn regression_display_trait() {
    assert_eq!(run_bin("display-trait"), "Displaying money: 42 EUR\n");
}

#[test]
fn regression_fixed_array_example() {
    let zeros = "[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, \
                 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]";
    let expected = format!(
        "integer_array_1: [1, 255, 3]\n\
         integer_array_2: [2, 3, 4]\n\
         integer_array_3: {}\n\
         integer_array_4 length: 16438\n\
         integer_array_1[0]: 1\n\
         integer_array_1[2]: 3\n",
        zeros
    );
    assert_eq!(run_bin("fixed-array-example"), expected);
}

#[test]
fn regression_generic_function() {
    assert_eq!(
        run_bin("generic-function"),
        "Selected first: 1\nSelected first: meep\n"
    );
}

#[test]
fn regression_generic_struct() {
    assert_eq!(
        run_bin("generic-struct"),
        "Whole euros: Money { amount: 42, currency: \"EUR\" }\n\
         Floating euros: Money { amount: 24.312, currency: \"EUR\" }\n"
    );
}

#[test]
fn regression_into_impl() {
    assert_eq!(
        run_bin("into-impl"),
        "Money without currency: CurrencylessMoney { amount: 42 }\n"
    );
}

#[test]
fn regression_mutable_string() {
    assert_eq!(
        run_bin("mutable-string"),
        "Length of the empty string is 0\n\
         Length of the empty string with capacity is 0\n\
         Length of the string from a bytestring is 4\n\
         Bytestring says RUST\n\
         1) Empty string now contains 1\n\
         2) Empty string now contains 12345\n\
         Length of the previously empty string is now 5\n"
    );
}

#[test]
fn regression_passing_string_slices() {
    assert_eq!(run_bin("passing-string-slices"), "Hey you!\nHey you!\n");
}

#[test]
fn regression_statics() {
    assert_eq!(run_bin("statics"), "Meep is 4\nMeep is now 42\nFuup is 9\n");
}

#[test]
fn regression_std_trait_impls() {
    assert_eq!(
        run_bin("std-trait-impls"),
        "Summed euros: Money { amount: 84, currency: \"EUR\" }\n"
    );
}

#[test]
fn regression_string_slices() {
    assert_eq!(
        run_bin("string-slices"),
        "Constant string says: This is a constant string\n\
         Another string says: This string is local to the main function\n"
    );
}

#[test]
fn regression_trait_object() {
    assert_eq!(
        run_bin("trait-object"),
        "1: Point { x: 1, y: 3 }\n2: ThreeDimPoint { x: 3, y: 5, z: 9 }\n"
    );
}
