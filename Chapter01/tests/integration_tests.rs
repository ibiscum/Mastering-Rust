use std::process::Command;

fn run_bin(name: &str) -> String {
    let output = Command::new("cargo")
        .args(["run", "--bin", name, "--quiet"])
        .current_dir(".")
        .output()
        .expect("failed to execute cargo run");

    if !output.status.success() {
        panic!(
            "binary {} failed with stderr: {}",
            name,
            String::from_utf8_lossy(&output.stderr)
        );
    }

    String::from_utf8_lossy(&output.stdout).to_string()
}

#[test]
fn first_program_prints_greetings() {
    let out = run_bin("first-program");
    assert!(out.contains("Hi there, inferred world"));
    assert!(out.contains("Hi there, non-inferred world"));
}

#[test]
fn variables_rebinds_target() {
    let out = run_bin("variables");
    assert!(out.contains("Howdy, world"));
    assert!(out.contains("Howdy, mate"));
}

#[test]
fn conditionals_branch_on_boolean() {
    let out = run_bin("conditionals");
    assert_eq!(out.trim(), "Condition was true");
}

#[test]
fn ifexpression_returns_else_branch() {
    let out = run_bin("ifexpression");
    assert_eq!(out.trim(), "Result of computation: Sanity reigns");
}

#[test]
fn semicolon_changes_expression_type() {
    let out = run_bin("semicolon");
    assert_eq!(out.trim(), "Result of computation: ()");
}

#[test]
fn loop_counts_down() {
    let out = run_bin("loop");
    let first = out.lines().next().expect("no output");
    assert!(first.contains("1000 more runs to go"));
    assert!(out.lines().count() == 1001);
}

#[test]
fn while_counts_down() {
    let out = run_bin("while");
    let first = out.lines().next().expect("no output");
    assert!(first.contains("1000 more runs to go"));
    assert!(out.lines().count() == 1000);
}

#[test]
fn arrays_prints_expected_values() {
    let out = run_bin("arrays");
    assert!(out.contains("The first number is 1"));
    assert!(out.lines().count() == 14);
}

#[test]
fn arrays_first_number_matches_index_zero() {
    let out = run_bin("arrays");
    let first_number_line = out
        .lines()
        .find(|line| line.contains("The first number is"))
        .expect("missing first number line");
    assert_eq!(first_number_line, "The first number is 1");
}

#[test]
fn slices_produces_expected_subslice() {
    let out = run_bin("slices");
    assert!(out.contains("All numbers: [1, 2, 4, 5]"));
    assert!(out.contains("The second of the first two numbers: 2"));
}

#[test]
fn tuples_prints_tuple_contents() {
    let out = run_bin("tuples");
    assert_eq!(out.trim(), "Number and string in a tuple: (40, \"a static string\")");
}

#[test]
fn vec_macro_and_push_produce_equal_vectors() {
    let out = run_bin("vec");
    assert_eq!(out.trim(), "Both vectors have equal contents: true");
}

#[test]
fn hashmap_inserts_and_reads_path() {
    let out = run_bin("hashmap");
    assert_eq!(out.trim(), "Configured path is Some(\"/home/user/\")");
}

#[test]
fn use_converts_to_ascii_uppercase() {
    let out = run_bin("use");
    assert_eq!(out.trim(), "a upper cased is A");
}

#[test]
fn use_without_use_converts_to_ascii_uppercase() {
    let out = run_bin("use-without-use");
    assert_eq!(out.trim(), "a upper cased is A");
}

#[test]
fn struct_prints_name_and_strength() {
    let out = run_bin("struct");
    assert_eq!(
        out.trim(),
        "Character's name is Generic AD&D Hero, and his/her strength is 9"
    );
}

#[test]
fn structmethods_prints_strength() {
    let out = run_bin("structmethods");
    assert_eq!(out.trim(), "Character's strength is 9");
}

#[test]
fn tuplestruct_equality_and_field_access() {
    let out = run_bin("tuplestruct");
    let expected = "Is temperature 1 the same as temperature 2? Answer: true\n\
                    Temperature 1 is 10 fahrenheit\n\
                    Temperature 2 is 10 celsius";
    assert_eq!(out.trim(), expected);
}

#[test]
fn enums_and_match_handles_all_variants() {
    let out = run_bin("enums-and-match");
    let expected = "Player wants to wait\n\
                    Player wants to attack direction North\n\
                    Player wants to move in direction Northeast with speed 2";
    assert_eq!(out.trim(), expected);
}

#[test]
fn enums_and_match_without_debug_trait_handles_all_variants() {
    let out = run_bin("enums-and-match-without-debug-trait");
    let expected = "Player wants to wait\n\
                    Player wants to attack direction North\n\
                    Player wants to move in direction Northeast with speed 2";
    assert_eq!(out.trim(), expected);
}
