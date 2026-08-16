---
description: Build the statusbar binary and wire it into your Claude Code status line
---

Install the statusbar status line end to end: get the binary onto PATH, wire
the `statusLine` block into the user's settings, and prove it renders.

1. **Check whether it's already there.** `command -v statusbar` and
   `ls -lL ~/.claude/bin/statusbar`. If either resolves, skip to step 3 — but
   compare `statusbar --version` against the plugin's manifest version and
   rebuild if they differ, since a stale binary reads new config keys as
   their defaults.

   An empty `command -v` does NOT mean `~/.claude/bin` is off PATH — the
   symlink goes dangling whenever `cargo clean` removes its target, and
   `command -v` skips anything it can't execute. That's the common case, and
   a rebuild fixes it. `ls -lL` is what tells the two apart: it fails on a
   dangling link and succeeds on a live one.

2. **Build and install it.** Everything this marketplace ships is built by its
   Makefile, including this binary. Find the repo root — the directory holding
   the top-level `Makefile` — trying these in order:

   - `~/.claude/plugins/marketplaces/dpep` — a marketplace install clones the
     whole repo, so this is the usual answer
   - `${CLAUDE_PLUGIN_ROOT}/../..` — a local dev checkout. Note the plugin's
     own directory is under `plugins/cache/`, not the marketplace clone, so
     this only works when the user is developing against a checkout.

   Then `make -C <root> install`. That builds the Rust workspace and symlinks
   `statusbar` into `~/.claude/bin`, alongside the marketplace's other
   binaries.

   No Rust toolchain? Say so and stop — `rustup` is the user's call, not
   yours. Everything below needs the binary.

3. **Wire up the status line.** Merge this block into the user's
   `~/.claude/settings.json` (create the file if absent; preserve every other
   key — read, modify, write back):

   ```json
   {
     "statusLine": {
       "type": "command",
       "command": "~/.claude/bin/statusbar",
       "refreshInterval": 10
     }
   }
   ```

   If a `statusLine` already points somewhere else, show it to the user and ask
   before replacing it. Use the absolute path: `${CLAUDE_PLUGIN_ROOT}` is not
   substituted in statusLine commands, and the plugin's own directory carries
   its version — so a path through it breaks on the next update. The
   `~/.claude/bin` symlink is stable across both.

4. **Smoke-test the render.** Pipe a sample payload through the same absolute
   path the `statusLine` block uses, and show the user the result:

   ```
   echo '{"workspace":{"current_dir":"'"$PWD"'"},"context_window":{"used_percentage":62}}' | ~/.claude/bin/statusbar
   ```

   Then re-run `command -v statusbar` — after the build, not before. If it
   resolves, the bare command works and there is nothing to mention. If it's
   still empty, that alone means only "the binary wasn't found"; before
   blaming PATH, inspect PATH itself:

   ```
   echo $PATH | tr ':' '\n' | grep -n '\.claude/bin'
   ```

   The directory being absent from PATH and the binary being missing are
   different problems with different fixes, and only the second one breaks
   the status line — `statusLine` calls an absolute path, so a PATH gap costs
   you nothing but typing `statusbar` by name in a shell. Say which one you
   found; never infer one from the other.

5. **Report.** Tell the user it's live, that the bar refreshes every ~10s, and
   that they can tune segments via `~/.config/claude/statusbar/config.json`
   (point them at the plugin README for the full option list — PR-as-branch,
   prefix stripping, model abbreviation, thresholds). No restart needed; the
   status line picks up the new command on the next refresh.
