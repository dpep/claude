#!/usr/bin/env bash
# plugins/statusbar/hooks/check-installed.sh — SessionStart hook.
#
# Unlike other plugins, statusbar doesn't run as a hook — it's wired into the
# user's settings.json as a `statusLine` command. ${CLAUDE_PLUGIN_ROOT} isn't
# substituted there, and the plugin's own directory is stamped with its
# version, so the binary has to sit at a stable absolute path. This hook checks
# it's there; a miss nudges /statusbar-install.

set -e

# `make install` symlinks into ~/.claude/bin, but check PATH first: someone
# building from a dev checkout may have it elsewhere, and a hook that only
# knows one location calls a working install broken.
bin="$(command -v statusbar 2>/dev/null || true)"
[[ -z "$bin" && -x "${HOME}/.claude/bin/statusbar" ]] && bin="${HOME}/.claude/bin/statusbar"

if [[ -z "$bin" || ! -x "$bin" ]]; then
    # A dangling symlink is the common failure — `cargo clean` removes the
    # target and `command -v` skips what it can't execute. Name that state
    # rather than blaming PATH, which is still fine.
    if [[ -L "${HOME}/.claude/bin/statusbar" ]]; then
        why="The ~/.claude/bin/statusbar symlink is dangling — its build target is gone (usually 'cargo clean'), so nothing executes and the status line renders empty. This says nothing about PATH; a rebuild relinks it."
    else
        why="No statusbar binary was found — nothing executable at ~/.claude/bin/statusbar, and nothing named statusbar on PATH — so the custom status line won't render. That is a statement about the search, not about PATH's contents: don't report ~/.claude/bin as missing from PATH without inspecting PATH itself, and note the status line calls an absolute path either way."
    fi
    cat <<EOF
{
  "systemMessage": "⚠️  statusbar binary is not runnable. Run /statusbar-install to (re)build it and wire up your status line.",
  "hookSpecificOutput": {
    "hookEventName": "SessionStart",
    "additionalContext": "${why} If the user asks you to fix it: run /statusbar-install, which handles it end to end. The short version: 'make -C ~/.claude/plugins/marketplaces/dpep install' (a marketplace install clones the whole repo, Makefile included), then point settings.json statusLine at ~/.claude/bin/statusbar."
  }
}
EOF
    exit 0
fi

# Staleness: the plugin's declarative files ship via git while the binary
# is built locally, so the two drift. Drift is not inert — a config using a
# field the binary predates is read with that field's default.
want="$(sed -n 's/.*"version": *"\([^"]*\)".*/\1/p' "${CLAUDE_PLUGIN_ROOT}/.claude-plugin/plugin.json" 2>/dev/null | head -1)"
got="$("$bin" --version 2>/dev/null | awk '{print $2}')"

if [[ -n "$want" && -n "$got" && "$want" != "$got" ]]; then
    cat <<EOF
{
  "systemMessage": "⚠️  statusbar binary is stale (binary ${got}, plugin ${want}). Run /statusbar-install to rebuild it.",
  "hookSpecificOutput": {
    "hookEventName": "SessionStart",
    "additionalContext": "The statusbar binary at ${bin} is version ${got} but the plugin ships ${want}, so the status line is running old code. If the user asks you to fix it: run /statusbar-install, which handles it end to end. The short version: 'make -C ~/.claude/plugins/marketplaces/dpep install' (a marketplace install clones the whole repo, Makefile included), then point settings.json statusLine at ~/.claude/bin/statusbar."
  }
}
EOF
    exit 0
fi

exit 0
