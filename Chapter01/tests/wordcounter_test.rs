use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;

#[test]
fn wordcounter_counts_words_in_file() {
    let mut file = NamedTempFile::new().expect("failed to create temp file");
    writeln!(file, "hello world hello").expect("failed to write");
    let path = file.path().to_str().expect("invalid path");

    let output = Command::new("cargo")
        .args(["run", "--bin", "wordcounter", "--quiet", "--", path])
        .current_dir(".")
        .output()
        .expect("failed to execute wordcounter");

    assert!(
        output.status.success(),
        "wordcounter failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("hello: 2"));
    assert!(stdout.contains("world: 1"));
}

#[test]
fn wordcounter_ignores_empty_splits() {
    let mut file = NamedTempFile::new().expect("failed to create temp file");
    writeln!(file, "one  two").expect("failed to write");
    let path = file.path().to_str().expect("invalid path");

    let output = Command::new("cargo")
        .args(["run", "--bin", "wordcounter", "--quiet", "--", path])
        .current_dir(".")
        .output()
        .expect("failed to execute wordcounter");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("one: 1"));
    assert!(stdout.contains("two: 1"));
}

#[test]
fn wordcounter_handles_tabs_and_punctuation() {
    let mut file = NamedTempFile::new().expect("failed to create temp file");
    writeln!(file, "hello, world\thello").expect("failed to write");
    let path = file.path().to_str().expect("invalid path");

    let output = Command::new("cargo")
        .args(["run", "--bin", "wordcounter", "--quiet", "--", path])
        .current_dir(".")
        .output()
        .expect("failed to execute wordcounter");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("hello,: 1"));
    assert!(stdout.contains("hello: 1"));
    assert!(stdout.contains("world: 1"));
}

#[test]
fn wordcounter_empty_file_produces_no_output() {
    let file = NamedTempFile::new().expect("failed to create temp file");
    let path = file.path().to_str().expect("invalid path");

    let output = Command::new("cargo")
        .args(["run", "--bin", "wordcounter", "--quiet", "--", path])
        .current_dir(".")
        .output()
        .expect("failed to execute wordcounter");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "");
}

#[test]
fn wordcounter_missing_arg_exits_with_error() {
    let output = Command::new("cargo")
        .args(["run", "--bin", "wordcounter", "--quiet"])
        .current_dir(".")
        .output()
        .expect("failed to execute wordcounter");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Usage: wordcounter <filename>") || stderr.contains("error"));
}
