use std::env;
use std::path::PathBuf;
use std::process::Command;

fn bin_path(name: &str) -> String {
    let env_var = format!("CARGO_BIN_EXE_{}", name);
    env::var(&env_var).unwrap_or_else(|_| {
        let manifest = env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
        PathBuf::from(manifest)
            .join("target/debug")
            .join(name)
            .to_string_lossy()
            .to_string()
    })
}

fn run_bin(name: &str) -> (bool, String, String) {
    let output = Command::new(&bin_path(name))
        .output()
        .unwrap_or_else(|e| panic!("failed to execute {}: {}", name, e));
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    (output.status.success(), stdout, stderr)
}

#[test]
fn regression_catch_unwind() {
    let (success, stdout, _) = run_bin("catch_unwind");
    assert!(success);
    assert_eq!(stdout, "Survived that panic.\n");
}

#[test]
fn regression_custom_error_1_panics() {
    let (success, _, stderr) = run_bin("custom-error-1");
    assert!(!success);
    assert!(stderr.contains("Unimplemented!"));
}

#[test]
fn regression_custom_error_2() {
    let expected = "Money_1 is Ok(Money { currency: EUR, amount: 12345 })\n\
                    Money_2 is Err(MoneyError { cause: CurrencyError { description: \"FIM not a valid currency\" } })\n\
                    MoneyError due to CurrencyError: FIM not a valid currency\n";
    let (success, stdout, _) = run_bin("custom-error-2");
    assert!(success);
    assert_eq!(stdout, expected);
}

#[test]
fn regression_mapping() {
    let err = "Err(FromUtf8Error { bytes: [130, 131, 132, 133], error: Utf8Error { valid_up_to: 0, error_len: Some(1) } })";
    let expected = format!(
        "Read the string: {err}\n\
         Read the string: Ok(\"PRTUV\")\n\
         Read the string: {err}\n\
         Read the string: Ok(\"PRTUV\")\n"
    );
    let (success, stdout, _) = run_bin("mapping");
    assert!(success);
    assert_eq!(stdout, expected);
}

#[test]
fn regression_panic_panics() {
    let (success, _, stderr) = run_bin("panic");
    assert!(!success);
    assert!(stderr.contains("Panicking in f3!"));
}

#[test]
fn regression_qmark_returns_error() {
    let (success, _, stderr) = run_bin("qmark");
    assert!(!success);
    assert!(stderr.contains("No such file or directory"));
}

#[test]
fn regression_result_1() {
    let (success, stdout, _) = run_bin("result-1");
    assert!(success);
    assert!(stdout.contains("Error opening file:"));
    assert!(stdout.contains("No such file or directory"));
}

#[test]
fn regression_result_unwrapping_panics() {
    let (success, _, stderr) = run_bin("result-unwrapping");
    assert!(!success);
    assert!(stderr.contains("Error while opening data.txt"));
}

#[test]
fn regression_try() {
    let err = "Err(FromUtf8Error { bytes: [130, 131, 132, 133], error: Utf8Error { valid_up_to: 0, error_len: Some(1) } })";
    let expected = format!(
        "Conversion succeeded: PRTUV\n\
         Conversion succeeded: PRTUV\n\
         Read the string: {err}\n\
         Read the string: Ok(\"PRTUV\")\n\
         Read the string: {err}\n\
         Read the string: Ok(\"PRTUV\")\n"
    );
    let (success, stdout, _) = run_bin("try");
    assert!(success);
    assert_eq!(stdout, expected);
}

#[test]
fn regression_try_main() {
    let (success, stdout, _) = run_bin("try-main");
    assert!(success);
    assert_eq!(stdout, "");
}
