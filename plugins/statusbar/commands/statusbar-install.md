---
description: Build the statusbar binary and wire it into your Claude Code status line
---

Install the statusbar status line end to end: get the binary onto PATH, wire
the `statusLine` block into the user's settings, and prove it renders.

1. **Check whether it's already there.** `command -v statusbar` and
   `ls ~/.claude/bin/statusbar`. If either resolves, skip to step 3 — but
   compare `statusbar --version` against the plugin's manifest version and
   rebuild if they differ, since a stale binary reads new config keys as
   their defaults.

2. **Get the binary.** Two ways; prefer the first, it needs no clone:

   ```sh
   cargo install --git https://github.com/dpep/claude statusbar-cli
   ```

   That lands `statusbar` in `~/.cargo/bin`. If the user has this repo cloned
   (or would rather build from it), `make install` from the repo root builds
   the workspace and symlinks into `~/.claude/bin` instead.

   No Rust toolchain? Say so and stop — `rustup` is the user's call to install,
   not yours. Everything below needs the binary.

   Note where it actually landed: step 3 needs that absolute path, and the two
   routes differ.

3. **Wire up the status line.** Merge this block into the user's
   `~/.claude/settings.json` (create the file if absent; preserve every other
   key — read, modify, write back):

   ```json
   {
     "statusLine": {
       "type": "command",
       "command": "<absolute path from step 2 — ~/.cargo/bin/statusbar or ~/.claude/bin/statusbar>",
       "refreshInterval": 10
     }
   }
   ```

   If a `statusLine` already points somewhere else, show it to the user and ask
   before replacing it. Use an absolute path: `${CLAUDE_PLUGIN_ROOT}` is not
   substituted in statusLine commands, and the plugin's own directory carries
   its version — so a path through it breaks on the next update. `~/.cargo/bin`
   and `~/.claude/bin` are both stable.

4. **Smoke-test the render.** Pipe a sample payload through the binary and show
   the user the result:

   ```
   echo '{"workspace":{"current_dir":"'"$PWD"'"},"context_window":{"used_percentage":62}}' | statusbar
   ```

5. **Report.** Tell the user it's live, that the bar refreshes every ~10s, and
   that they can tune segments via `~/.config/claude/statusbar/config.json`
   (point them at the plugin README for the full option list — PR-as-branch,
   prefix stripping, model abbreviation, thresholds). No restart needed; the
   status line picks up the new command on the next refresh.
