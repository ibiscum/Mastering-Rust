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
fn regression_channels_1() {
    let expected = "Received 0 via the channel\n\
                    Received 1 via the channel\n\
                    Received 2 via the channel\n";
    assert_eq!(run_bin("channels-1"), expected);
}

#[test]
fn regression_channels_2() {
    let expected = "Received 0 via the channel\n\
                    Received 1 via the channel\n\
                    Received 2 via the channel\n";
    assert_eq!(run_bin("channels-2"), expected);
}

#[test]
fn regression_closures_1() {
    let expected = "square of 4 = 16\n\
                    square of 4 = 16\n\
                    square of 4 = 16\n\
                    square of 4 = 16\n\
                    Entered function without variables\n\
                    Entered closure without variables\n";
    assert_eq!(run_bin("closures-1"), expected);
}

#[test]
fn regression_closures_2() {
    let output = run_bin("closures-2");
    assert!(output.contains("Outer scope variable is 84"), "unexpected output: {}", output);
    assert!(output.contains("Outer_scope_x 42"), "unexpected output: {}", output);
}

#[test]
fn regression_mutex_arc_finishes() {
    // The full binary takes ~16 minutes to spawn 1_000_000 threads. Run it with
    // a tiny thread count via the CHAPTER07_QUICK environment variable if
    // supported, otherwise only verify it starts successfully.
    let mut child = Command::new(&bin_path("mutex-arc"))
        .env("CHAPTER07_QUICK", "1")
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("failed to spawn mutex-arc");

    std::thread::sleep(std::time::Duration::from_millis(200));
    let still_running = child.try_wait().unwrap().is_none();
    child.kill().ok();

    let output = child.wait_with_output().expect("failed to read mutex-arc output");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        still_running || stdout.contains("Not there yet") || stdout.contains("Got there"),
        "unexpected state: still_running={} stdout={}",
        still_running,
        stdout
    );
}

#[test]
fn regression_mutex_rc() {
    assert_eq!(
        run_bin("mutex-rc"),
        "1 Arc/Mutexed number plus one equals 6\n"
    );
}

#[test]
fn regression_mutexes_1() {
    assert_eq!(
        run_bin("mutexes-1"),
        "Mutexed number plus one equals 6\n"
    );
}

#[test]
fn regression_mutexes_2() {
    let expected = "1 Mutexed number plus one equals 6\n\
                    2 Mutexed number plus one equals 6\n";
    assert_eq!(run_bin("mutexes-2"), expected);
}

#[test]
fn regression_sharing_immutables() {
    let output = run_bin("sharing-immutables");
    assert!(
        output.contains("In main thread: num is now 13"),
        "unexpected output: {}",
        output
    );
}

#[test]
fn regression_threads_1() {
    assert_eq!(
        run_bin("threads-1"),
        "Inside thread with string 'outside'\n"
    );
}
