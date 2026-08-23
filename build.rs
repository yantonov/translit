use std::path::{Path, PathBuf};
use std::process::Command;

/// Asks git about the full hash of the current commit.
/// Returns None when git is not available or the sources are not a repository
/// (a build from a published crate/tarball, for example).
fn commit_hash() -> Option<String> {
    let output = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let hash = String::from_utf8(output.stdout).ok()?.trim().to_string();
    if hash.is_empty() { None } else { Some(hash) }
}

fn git_path(name: &str) -> Option<PathBuf> {
    let output = Command::new("git")
        .args(["rev-parse", "--git-path", name])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let path = String::from_utf8(output.stdout).ok()?.trim().to_string();
    if path.is_empty() {
        None
    } else {
        Some(PathBuf::from(path))
    }
}

/// A commit on a branch updates the ref HEAD points to, not HEAD itself.
fn head_reference(head: &Path) -> Option<PathBuf> {
    let content = std::fs::read_to_string(head).ok()?;
    let reference = content.trim().strip_prefix("ref: ")?;
    git_path(reference)
}

/// The hash is baked into the binary, so the build has to be repeated
/// whenever HEAD moves to another commit.
fn watch_head() {
    println!("cargo:rerun-if-changed=build.rs");
    let head = match git_path("HEAD") {
        None => return,
        Some(head) => head,
    };
    println!("cargo:rerun-if-changed={}", head.display());

    if let Some(path) = head_reference(&head) {
        println!("cargo:rerun-if-changed={}", path.display());
    }
    // refs may be packed instead of stored as separate files
    if let Some(packed_refs) = git_path("packed-refs") {
        println!("cargo:rerun-if-changed={}", packed_refs.display());
    }
}

fn main() {
    watch_head();
    println!(
        "cargo:rustc-env=GIT_COMMIT_HASH={}",
        commit_hash().unwrap_or_else(|| "unknown".to_string())
    );
}
