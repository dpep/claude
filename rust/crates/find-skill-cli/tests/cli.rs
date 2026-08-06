//! End-to-end tests driving the real `find-skill` binary over a
//! hermetic environment (temp CLAUDE_DIR + XDG dirs, HOME pointed at a
//! temp so discovery can't reach the real `~/code`).

use std::path::Path;
use std::process::Command;

fn write(path: &Path, body: &str) {
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, body).unwrap();
}

fn json(out: &std::process::Output) -> serde_json::Value {
    serde_json::from_str(std::str::from_utf8(&out.stdout).unwrap()).unwrap()
}

/// A Command for the binary with a fully isolated environment rooted at
/// `home`, and a personal skill `notes` already present.
fn cmd(home: &Path) -> Command {
    let claude = home.join(".claude");
    write(
        &claude.join("skills/notes/SKILL.md"),
        "---\nname: notes\ndescription: Jot down loose notes and todos.\n---\n# Notes\n",
    );
    let mut c = Command::new(env!("CARGO_BIN_EXE_find-skill"));
    c.current_dir(home)
        .env("HOME", home)
        .env("CLAUDE_DIR", &claude)
        .env("XDG_CONFIG_HOME", home.join("cfg"))
        .env("XDG_CACHE_HOME", home.join("cache"))
        .env_remove("XDG_DATA_HOME");
    c
}

#[test]
fn lists_personal_skill_as_text() {
    let home = tempfile::tempdir().unwrap();
    let out = cmd(home.path()).arg("notes").output().unwrap();
    assert!(out.status.success());
    let text = String::from_utf8(out.stdout).unwrap();
    assert!(text.contains("notes"), "got: {text}");
    assert!(
        text.contains(".claude/skills/notes/SKILL.md"),
        "got: {text}"
    );
}

#[test]
fn json_output_carries_structured_fields() {
    let home = tempfile::tempdir().unwrap();
    let out = cmd(home.path()).args(["--json", "notes"]).output().unwrap();
    assert!(out.status.success());
    let text = String::from_utf8(out.stdout).unwrap();
    let v: serde_json::Value = serde_json::from_str(&text).unwrap();
    let first = &v[0];
    assert_eq!(first["label"], "notes");
    assert_eq!(first["editable"], true);
    assert_eq!(first["source"]["kind"], "personal");
}

#[test]
fn flags_a_cached_but_disabled_plugin() {
    let home = tempfile::tempdir().unwrap();
    let claude = home.path().join(".claude");
    write(
        &claude.join("plugins/marketplaces/dpep/plugins/code/skills/gqls/SKILL.md"),
        "---\nname: gqls\ndescription: Search a GraphQL schema.\n---\n",
    );
    // The plugin is downloaded but never enabled.
    write(&claude.join("settings.json"), r#"{"enabledPlugins":{}}"#);

    let out = cmd(home.path()).arg("gqls").output().unwrap();
    assert!(out.status.success());
    let text = String::from_utf8(out.stdout).unwrap();
    // The marker rides the skill's own row, not just the trailing note.
    let row = text
        .lines()
        .find(|l| l.contains("gqls"))
        .unwrap_or_default();
    assert!(row.contains("(disabled)"), "got: {text}");
    assert!(text.contains("\"code@dpep\": true"), "got: {text}");

    let out = cmd(home.path()).args(["--json", "gqls"]).output().unwrap();
    let first = json(&out)[0].clone();
    assert_eq!(first["enabled"], false);
    assert_eq!(first["plugin_ref"], "code@dpep");

    // Enabling it clears both the marker and the note.
    write(
        &claude.join("settings.json"),
        r#"{"enabledPlugins":{"code@dpep":true}}"#,
    );
    let out = cmd(home.path()).arg("gqls").output().unwrap();
    let text = String::from_utf8(out.stdout).unwrap();
    assert!(!text.contains("disabled"), "got: {text}");
    let out = cmd(home.path()).args(["--json", "gqls"]).output().unwrap();
    assert_eq!(json(&out)[0]["enabled"], true);
}

/// The binary runs with cwd inside HOME, so the ancestor walk for
/// project settings passes right by `~/.claude` — a stray local file
/// there must not decide enablement.
#[test]
fn stray_local_settings_beside_user_settings_are_ignored() {
    let home = tempfile::tempdir().unwrap();
    let claude = home.path().join(".claude");
    write(
        &claude.join("plugins/marketplaces/dpep/plugins/code/skills/gqls/SKILL.md"),
        "---\nname: gqls\ndescription: Search a GraphQL schema.\n---\n",
    );
    write(
        &claude.join("settings.json"),
        r#"{"enabledPlugins":{"code@dpep":true}}"#,
    );
    write(
        &claude.join("settings.local.json"),
        r#"{"enabledPlugins":{"code@dpep":false}}"#,
    );

    let out = cmd(home.path()).args(["--json", "gqls"]).output().unwrap();
    assert_eq!(json(&out)[0]["enabled"], true);
}

#[test]
fn no_match_exits_nonzero() {
    let home = tempfile::tempdir().unwrap();
    let out = cmd(home.path()).arg("zzzznope").output().unwrap();
    assert_eq!(out.status.code(), Some(1));
}

#[test]
fn first_prints_only_the_path() {
    let home = tempfile::tempdir().unwrap();
    let out = cmd(home.path()).args(["-1", "notes"]).output().unwrap();
    assert!(out.status.success());
    let text = String::from_utf8(out.stdout).unwrap();
    assert_eq!(
        text.trim(),
        home.path()
            .join(".claude/skills/notes/SKILL.md")
            .to_string_lossy()
    );
}

#[test]
fn add_then_paths_shows_registered_dir() {
    let home = tempfile::tempdir().unwrap();
    // A local repo with a skill to discover.
    let repo = home.path().join("work/proj");
    write(
        &repo.join(".git/config"),
        "[remote \"origin\"]\n\turl = git@github.com:rewild/proj.git\n",
    );
    write(
        &repo.join("skills/thing/SKILL.md"),
        "---\nname: thing\n---\n",
    );

    let workspace = repo.parent().unwrap().to_str().unwrap().to_string();
    let out = cmd(home.path())
        .args(["--add", &workspace, "--json"])
        .output()
        .unwrap();
    assert!(out.status.success());
    let v: serde_json::Value =
        serde_json::from_str(&String::from_utf8(out.stdout).unwrap()).unwrap();
    assert_eq!(v["ok"], true);
    assert_eq!(v["roots"].as_array().unwrap().len(), 1);

    let paths = cmd(home.path())
        .args(["--paths", "--json"])
        .output()
        .unwrap();
    let v: serde_json::Value =
        serde_json::from_str(&String::from_utf8(paths.stdout).unwrap()).unwrap();
    assert_eq!(v["roots"][0]["remote"], "rewild/proj");
}

#[test]
fn reset_clears_config_and_cache() {
    let home = tempfile::tempdir().unwrap();
    let repo = home.path().join("proj");
    write(
        &repo.join(".git/config"),
        "[remote \"origin\"]\n\turl = git@github.com:rewild/proj.git\n",
    );
    write(
        &repo.join("skills/thing/SKILL.md"),
        "---\nname: thing\n---\n",
    );

    // Register a path (writes config + cache), then reset.
    let repo_str = repo.to_str().unwrap().to_string();
    cmd(home.path())
        .args(["--add", &repo_str])
        .output()
        .unwrap();
    let reset = cmd(home.path())
        .args(["--reset", "--json"])
        .output()
        .unwrap();
    assert!(reset.status.success());
    assert_eq!(json(&reset)["ok"], true);

    // After reset, nothing is registered anymore.
    let paths = cmd(home.path())
        .args(["--paths", "--json"])
        .output()
        .unwrap();
    assert_eq!(json(&paths)["search_paths"].as_array().unwrap().len(), 0);
}
