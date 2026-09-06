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

fn run_bin(name: &str) -> String {
    let output = Command::new(&bin_path(name))
        .output()
        .unwrap_or_else(|e| panic!("failed to execute {}: {}", name, e));

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
fn regression_add_2() {
    assert_eq!(run_bin("add-2"), "");
}

#[test]
fn regression_blocks() {
    let expected = "level_0_str is alive: foo\n\
                    Inside level 1 block, number = 9\n\
                    Inside level 2 block, vector = [1, 2, 3]\n\
                    Back in level 1 block, number still = 9\n\
                    Inside another level 2 block, number = 9\n\
                    Back in main, level_0_str still alive: foo\n";
    assert_eq!(run_bin("blocks"), expected);
}

#[test]
fn regression_borrows_1() {
    assert_eq!(run_bin("borrows-1"), "value = 1\n");
}

#[test]
fn regression_borrows_2() {
    assert_eq!(run_bin("borrows-2"), "x1 says 1\nx2 says 1\nsum is 2\n");
}

#[test]
fn regression_drops() {
    let output = run_bin("drops");
    assert!(output.contains("Steve went away"), "unexpected output: {}", output);
    assert!(output.contains("John went away"), "unexpected output: {}", output);
}

#[test]
fn regression_functions() {
    let expected = "The n is 5\n\
                    The s is string\n\
                    n is 5\n\
                    s is string\n";
    assert_eq!(run_bin("functions"), expected);
}

#[test]
fn regression_functions_with_borrows_1() {
    let expected = "The n is 5\n\
                    The s is string\n\
                    n is 5\n\
                    s is string\n";
    assert_eq!(run_bin("functions-with-borrows-1"), expected);
}

#[test]
fn regression_functions_with_borrows_2() {
    let expected = "The n is 5\n\
                    The s is string\n\
                    Foo: Foo\n\
                    n is 10\n\
                    s is string\n";
    assert_eq!(run_bin("functions-with-borrows-2"), expected);
}

#[test]
fn regression_interior_mutability() {
    let expected = "Set cache: 17\n\
                    Summed result: 17\n\
                    Got from cache: 17\n\
                    Summed result: 17\n";
    assert_eq!(run_bin("interior-mutability"), expected);
}

#[test]
fn regression_lifetime_structs() {
    let expected = "num is now 1\n\
                    num is now 2\n";
    assert_eq!(run_bin("lifetime-structs"), expected);
}

#[test]
fn regression_mem_introspection() {
    let output = run_bin("mem-introspection");
    assert!(output.contains("type u8: 1"), "unexpected output: {}", output);
    assert!(output.contains("type f64: 8"), "unexpected output: {}", output);
}

#[test]
fn regression_multiple_cells() {
    assert_eq!(run_bin("multiple-cells"), "x is now 3\n");
}

#[test]
fn regression_multiple_move_types() {
    assert_eq!(run_bin("multiple-move-types"), "Foo { number: 3 } Foo { number: 3 }\n");
}

#[test]
fn regression_multiple_move_types_with_refcell_1() {
    // This binary intentionally panics at runtime to demonstrate borrowing rules.
    let output = Command::new(&bin_path("multiple-move-types-with-refcell-1"))
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("already borrowed") || stderr.contains("BorrowMutError"), "unexpected stderr: {}", stderr);
}

#[test]
fn regression_multiple_move_types_with_refcell_2() {
    assert_eq!(run_bin("multiple-move-types-with-refcell-2"), "");
}

#[test]
fn regression_multiple_owners() {
    let expected = "Number num1 is 1\n\
                    Number num2 is 1\n\
                    String s1 is meep\n\
                    String s2 is meep\n";
    assert_eq!(run_bin("multiple-owners"), expected);
}

#[test]
fn regression_rc_1() {
    let output = run_bin("rc-1");
    assert!(output.starts_with("nums:"), "unexpected output: {}", output);
    assert!(output.contains("strs:"), "unexpected output: {}", output);
}

#[test]
fn regression_rc_2() {
    let output = run_bin("rc-2");
    assert!(output.starts_with("nums:"), "unexpected output: {}", output);
}

#[test]
fn regression_rc_3() {
    let output = run_bin("rc-3");
    assert!(output.starts_with("nums:"), "unexpected output: {}", output);
}

#[test]
fn regression_stack_1() {
    assert_eq!(run_bin("stack-1"), "f1(9) is 16\n");
}

#[test]
fn regression_stack_2() {
    assert_eq!(run_bin("stack-2"), "f1 returned 4\n");
}
