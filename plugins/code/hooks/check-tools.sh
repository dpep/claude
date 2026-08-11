#!/usr/bin/env bash
# SessionStart hook — report which of this plugin's CLIs aren't callable.
#
# Most skills here are pure instructions, but four drive a binary. A skill
# whose CLI is missing doesn't fail loudly: Claude reads the skill, runs the
# command, gets "not found", and falls back to grep — quietly worse, with
# nothing telling the user why. This says so once per session.
#
# Silent when everything resolves. Reports every missing tool at once rather
# than the first, since a fresh install is usually missing several and one
# nudge per session is the budget.

set -u

# Each entry: binary|what it's for|how to get it. The remedy differs by tool —
# two ship from their own repos, one builds from this one, one is a script
# sitting in this plugin — so a single "run make install" line would be wrong
# for three of the four.
tools=(
  "rq|find a definition by name|brew install dpep/tools/rq   (or: cargo install reference-query)"
  "gqls|search a GraphQL schema|brew install dpep/tools/gqls   (or: cargo install gqls-cli)"
  "find-skill|locate where a skill is defined|make -C ~/.claude/plugins/marketplaces/dpep install"
)

missing=()
for entry in "${tools[@]}"; do
    IFS='|' read -r bin purpose remedy <<< "$entry"
    command -v "$bin" >/dev/null 2>&1 || missing+=("${bin} (${purpose}) — ${remedy}")
done

# find-gem ships in this plugin rather than being installed, so its remedy is
# a symlink, not a package. CLAUDE_PLUGIN_ROOT is stamped with the plugin
# version, so link it somewhere stable rather than calling it by that path.
if ! command -v find-gem >/dev/null 2>&1; then
    src="${CLAUDE_PLUGIN_ROOT:-}/bin/find-gem"
    missing+=("find-gem (locate installed gem source) — ln -sf '${src}' ~/.claude/bin/find-gem")
fi

[[ ${#missing[@]} -eq 0 ]] && exit 0

list=$(printf '%s\\n' "${missing[@]}")

# A common cause of "installed but not found": the bin dir exists and isn't on
# PATH. Worth saying, because the install commands above will appear to work
# and change nothing.
path_note=""
if [[ -d "${HOME}/.claude/bin" && ":${PATH}:" != *":${HOME}/.claude/bin:"* ]]; then
    path_note="\\n\\n~/.claude/bin exists but is not on PATH — add it in your shell rc: export PATH=\$HOME/.claude/bin:\$PATH"
fi

count=${#missing[@]}
total=$(( ${#tools[@]} + 1 ))

# Who to tell, and how loudly. A plugin ships several skills and someone may
# want only one of them — nagging every session about a CLI they never
# intended to install is worse than saying nothing. So:
#
#   nothing works  -> say it out loud once; this is a fresh install and the
#                     user almost certainly meant to finish it
#   some works     -> tell Claude only. It can offer the install at the moment
#                     the user actually asks for that capability, which is when
#                     the information is worth having.
#
# Either way Claude is told, so it never silently substitutes grep for a tool
# it can't run.
system_message=""
if [[ $count -eq $total ]]; then
    system_message="⚠️  None of the code plugin's CLIs are on PATH, so its rq/gqls/find-skill/find-gem skills can't run. Ask Claude to install them, or see the plugin README."
fi

cat <<EOF
{
  "systemMessage": "${system_message}",
  "hookSpecificOutput": {
    "hookEventName": "SessionStart",
    "additionalContext": "These code-plugin CLIs are not on PATH. The skills that drive them cannot run — say so rather than silently falling back to grep, and offer the install when the user asks for that capability:\\n${list}${path_note}\\n\\nDon't raise this unprompted if the user is working on something else; several skills ship together and they may only want some. The remedies take effect immediately, no reload needed."
  }
}
EOF
exit 0
