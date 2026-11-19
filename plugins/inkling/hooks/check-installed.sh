#!/usr/bin/env bash
# plugins/inkling/hooks/check-installed.sh — SessionStart hook.
#
# The other hooks are no-ops without the binary (`command -v inkling || true`),
# so a missing one would fail silently. Say so once per session instead.

command -v inkling >/dev/null 2>&1 && exit 0

cat <<'JSON'
{
  "systemMessage": "⚠️  inkling is not on PATH, so nothing will be watched or pushed. Install it: cargo install inkling-dev",
  "hookSpecificOutput": {
    "hookEventName": "SessionStart",
    "additionalContext": "The inkling plugin is enabled but the inkling binary is not on PATH, so its hooks are no-ops this session: no watches, no pushed notifications. If the user asks you to fix it: `cargo install inkling-dev` (needs a Rust toolchain), then confirm ~/.cargo/bin is on PATH. The hooks pick the binary up on their next call; no restart needed."
  }
}
JSON
