//! Output mode plumbing. Every command supports `-j/--json` (one
//! compact document) and `-J/--ndjson` (one object per line) so the
//! tool is agent-friendly; text is the human default.

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

/// Emit a list: JSON array (`--json`) or one object per line (`--ndjson`).
pub fn emit_list<T: Serialize>(mode: Mode, items: &[T]) {
    match mode {
        Mode::Json => println!(
            "{}",
            serde_json::to_string(items).unwrap_or_else(|_| "[]".into())
        ),
        Mode::Ndjson => {
            for item in items {
                if let Ok(s) = serde_json::to_string(item) {
                    println!("{s}");
                }
            }
        }
        Mode::Text => {}
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
