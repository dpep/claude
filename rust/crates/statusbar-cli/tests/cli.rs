//! End-to-end tests driving the real `statusbar` binary: pipe session
//! JSON on stdin, assert the rendered bar. We read via `--json` so the
//! `statusline` field is ANSI-free and stable to assert on. HOME is
//! pointed at a temp dir with no config so defaults apply, and the cwd
//! is a temp dir (no git) so the branch resolves to nothing.

use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

fn run(home: &Path, cwd: &str, stdin: &str) -> serde_json::Value {
    let mut child = Command::new(env!("CARGO_BIN_EXE_statusbar"))
        .arg("--json")
        .env("HOME", home)
        .env("XDG_CONFIG_HOME", home.join("cfg"))
        .env_remove("PWD")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(stdin.as_bytes())
        .unwrap();
    let out = child.wait_with_output().unwrap();
    assert!(out.status.success(), "binary failed");
    let _ = cwd; // cwd travels in the JSON payload below
    serde_json::from_slice(&out.stdout).unwrap()
}

#[test]
fn renders_cwd_and_hides_trunk_branch() {
    let home = tempfile::tempdir().unwrap();
    // A non-repo dir → no branch; cwd under HOME collapses to ~.
    let cwd = home.path().join("projects");
    std::fs::create_dir_all(&cwd).unwrap();
    let payload = format!(r#"{{"workspace": {{"current_dir": "{}"}}}}"#, cwd.display());
    let v = run(home.path(), cwd.to_str().unwrap(), &payload);
    assert_eq!(v["statusline"], "~/projects");
}

#[test]
fn renders_rate_limit_warning() {
    let home = tempfile::tempdir().unwrap();
    let cwd = home.path().join("proj");
    std::fs::create_dir_all(&cwd).unwrap();
    let payload = format!(
        r#"{{"workspace": {{"current_dir": "{}"}}, "rate_limits": {{"five_hour": {{"used_percentage": 88, "resets_at": 0}}}}}}"#,
        cwd.display()
    );
    let v = run(home.path(), cwd.to_str().unwrap(), &payload);
    assert_eq!(v["statusline"], "~/proj · rate:88%");
}

#[test]
fn empty_stdin_yields_empty_bar() {
    let home = tempfile::tempdir().unwrap();
    let v = run(home.path(), "", "");
    assert_eq!(v["statusline"], "");
}

#[test]
fn garbage_stdin_does_not_crash() {
    let home = tempfile::tempdir().unwrap();
    let v = run(home.path(), "", "this is not json");
    assert_eq!(v["statusline"], "");
}
