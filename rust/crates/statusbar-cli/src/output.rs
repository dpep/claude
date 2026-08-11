//! Output mode plumbing. `-j/--json` emits one compact object and
//! `-J/--ndjson` one object per line (identical here — the status bar is
//! a single value); text is the human default that Claude Code renders.

use serde::Serialize;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Text,
    Json,
    Ndjson,
}

impl Mode {
    pub fn new(json: bool, ndjson: bool) -> Self {
        if ndjson {
            Mode::Ndjson
        } else if json {
            Mode::Json
        } else {
            Mode::Text
        }
    }

    pub fn structured(self) -> bool {
        self != Mode::Text
    }
}

/// Emit a single object (identical under `--json` and `--ndjson`).
pub fn emit_one<T: Serialize>(mode: Mode, item: &T) {
    if mode.structured() {
        if let Ok(s) = serde_json::to_string(item) {
            println!("{s}");
        }
    }
}
