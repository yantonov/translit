//! End to end tests: every case runs the real binary and inspects its
//! stdout / stderr / exit code, so a change of the cli interface itself
//! (and not only of the transliteration) is detected.

use std::process::{Command, Output};

const EXECUTABLE: &str = env!("CARGO_BIN_EXE_translit");

const SUCCESS: i32 = 0;
const USAGE_ERROR: i32 = 2;

fn run(arguments: &[&str]) -> Output {
    Command::new(EXECUTABLE)
        .args(arguments)
        .output()
        .unwrap_or_else(|error| panic!("cannot run {}: {}", EXECUTABLE, error))
}

fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).trim().to_string()
}

/// Runs the binary, checks the expected exit code and returns the output
/// stream which is meaningful for that code.
fn output_for(arguments: &[&str], expected_code: i32) -> String {
    let output = run(arguments);
    assert!(
        !stderr(&output).contains("panicked"),
        "{:?} panicked:\n{}",
        arguments,
        stderr(&output)
    );
    assert_eq!(
        Some(expected_code),
        output.status.code(),
        "unexpected exit code for {:?}\nstdout: {}\nstderr: {}",
        arguments,
        stdout(&output),
        stderr(&output)
    );
    match expected_code {
        SUCCESS => stdout(&output),
        _ => {
            assert_eq!(
                "",
                stdout(&output),
                "nothing should be printed to stdout for {:?}",
                arguments
            );
            stderr(&output)
        }
    }
}

fn translit(arguments: &[&str]) -> String {
    output_for(arguments, SUCCESS)
}

fn usage_error(arguments: &[&str]) -> String {
    output_for(arguments, USAGE_ERROR)
}

/// The binary reports the version together with the full hash of the commit
/// it was built from, so a binary found on a machine can be traced back
/// to the exact sources.
#[test]
fn version_command_shows_the_version_and_the_full_commit_hash() {
    let output = translit(&["version"]);

    let (name, version_and_hash) = output
        .split_once(' ')
        .unwrap_or_else(|| panic!("unexpected version output: {}", output));
    assert_eq!(env!("CARGO_PKG_NAME"), name);

    let (version, hash) = version_and_hash
        .split_once(' ')
        .unwrap_or_else(|| panic!("the commit hash is missing: {}", output));
    assert_eq!(env!("CARGO_PKG_VERSION"), version);

    let hash = hash
        .strip_prefix('(')
        .and_then(|value| value.strip_suffix(')'))
        .unwrap_or_else(|| panic!("the commit hash is not parenthesized: {}", output));
    // the full hash, not the abbreviated one
    assert_eq!(
        40,
        hash.len(),
        "the full commit hash is expected, got: {}",
        hash
    );
    assert!(
        hash.chars().all(|c| c.is_ascii_hexdigit()),
        "not a commit hash: {}",
        hash
    );
}

/// The flag and the command are two spellings of the same thing.
#[test]
fn version_flag_matches_the_version_command() {
    let expected = translit(&["version"]);
    for flag in ["--version", "-V"] {
        assert_eq!(expected, translit(&[flag]), "{}", flag);
    }
}

#[test]
fn help_mentions_the_version_command() {
    let help = translit(&["--help"]);
    assert!(
        help.contains("version"),
        "the version command is not documented:\n{}",
        help
    );
}

#[test]
fn version_command_takes_no_arguments() {
    usage_error(&["version", "extra"]);
}

/// The version command is an addition, the existing commands keep working.
#[test]
fn other_commands_still_work() {
    assert_eq!(
        "Moskva",
        translit(&["convert", "Москва", "--schema", "wikipedia"])
    );
    assert!(
        translit(&["scheme", "list"]).contains("wikipedia"),
        "the scheme list is expected to mention wikipedia"
    );
}
