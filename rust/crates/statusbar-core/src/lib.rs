//! statusbar-core — render Claude Code's session JSON into a status bar.
//!
//! Three pieces:
//!   - `input`  — serde structs for the JSON Claude Code pipes on stdin.
//!                Every field is optional / defaulted so a partial or
//!                future-extended payload still parses (fail open).
//!   - `config` — the user's `~/.config/claude/statusbar/config.json`
//!                shape, with sensible anonymized defaults baked in.
//!   - `render` — the pure `render(session, env, config) -> String`.
//!
//! The CLI supplies the impure bits — the git branch, `$HOME`, and the
//! current unix time — via [`Env`] so the render stays deterministic and
//! testable.

mod config;
mod gh;
mod input;
mod render;

pub use config::Config;
pub use gh::handle_from_hosts_yml;
pub use input::Session;
pub use render::{render, strip_ansi, Env};
