#!/usr/bin/env bash
# plugins/statusbar/hooks/label-session.sh — Stop hook.
#
# Keeps a short, *current* label for the status line. Claude Code derives a
# session name from the opening task and keeps it forever, so a long session
# ends up advertising work it finished hours ago. This re-derives one from the
# recent transcript and caches it where the binary looks.
#
# Off unless `session.auto_label` is true in the statusbar config: it spends a
# model call, and nobody should pay for that without asking.

set -uo pipefail

# A headless `claude -p` fires Stop hooks of its own. Without this the
# labeller labels itself, recursively, forever.
[[ -n "${STATUSBAR_LABELING:-}" ]] && exit 0

payload="$(cat)"

read -r session_id transcript < <(
    printf '%s' "$payload" | python3 -c '
import json, sys
try:
    d = json.load(sys.stdin)
except Exception:
    print("  "); raise SystemExit
sid = d.get("session_id") or ""
tp = d.get("transcript_path") or ""
# Guard the id: it becomes a path component.
if not all(c.isalnum() or c in "-_" for c in sid):
    sid = ""
print(sid or "-", tp or "-")
' 2>/dev/null
)
[[ -z "${session_id:-}" || "$session_id" == "-" ]] && exit 0
[[ -z "${transcript:-}" || "$transcript" == "-" || ! -f "$transcript" ]] && exit 0

cfg="${XDG_CONFIG_HOME:-$HOME/.config}/claude/statusbar/config.json"
read -r enabled ttl_min max_len < <(
    python3 - "$cfg" <<'PY' 2>/dev/null
import json, sys
try:
    with open(sys.argv[1]) as f:
        c = json.load(f)
except Exception:
    c = {}
s = c.get("session")
s = s if isinstance(s, dict) else {}
print(
    "1" if s.get("auto_label") else "0",
    int(s.get("label_ttl_minutes", 30)),
    int(s.get("max_len", 24)),
)
PY
)
[[ "${enabled:-0}" != "1" ]] && exit 0

labels="${XDG_CACHE_HOME:-$HOME/.cache}/claude/statusbar/labels"
label_file="$labels/$session_id"

# The generated title, as of now. A label is written against one of these; the
# binary compares them so a later manual rename quietly wins.
ai_title="$(python3 - "$transcript" <<'PY' 2>/dev/null
import json, sys
title = ""
try:
    with open(sys.argv[1]) as f:
        for line in f:
            try:
                o = json.loads(line)
            except Exception:
                continue
            if "aiTitle" in o:
                title = o["aiTitle"] or title
except Exception:
    pass
print(title)
PY
)"

# TTL: re-label a few times an hour, not every turn. A missing file always
# labels, so a new session gets one on its first stop.
if [[ -f "$label_file" ]]; then
    now=$(date +%s)
    then=$(stat -f %m "$label_file" 2>/dev/null || stat -c %Y "$label_file" 2>/dev/null || echo 0)
    (( now - then < ttl_min * 60 )) && exit 0
fi

mkdir -p "$labels" || exit 0

# The recent user turns say what the session is doing *now* — which is the
# whole point, since the stale name already covers what it started doing.
recent="$(python3 - "$transcript" <<'PY' 2>/dev/null
import json, sys
msgs = []
try:
    with open(sys.argv[1]) as f:
        for line in f:
            try:
                o = json.loads(line)
            except Exception:
                continue
            if o.get("type") != "user" or o.get("isMeta") or o.get("isSidechain"):
                continue
            c = (o.get("message") or {}).get("content")
            if isinstance(c, list):
                c = " ".join(b.get("text", "") for b in c if isinstance(b, dict))
            if isinstance(c, str) and c.strip() and not c.startswith("<"):
                msgs.append(" ".join(c.split())[:200])
except Exception:
    pass
print("\n".join(msgs[-12:]))
PY
)"
[[ -z "$recent" ]] && exit 0

# Backgrounded: a Stop hook that waits on a model call delays the turn ending.
(
    label="$(
        STATUSBAR_LABELING=1 claude -p \
            "These are the recent requests in a coding session, oldest first. Reply with a status-bar label for what it is working on NOW — the latest thread of work, not the earliest. Plain noun phrase, at most ${max_len} characters, no quotes, no punctuation, no trailing period. Reply with the label and nothing else." \
            --model haiku <<<"$recent" 2>/dev/null | head -1
    )"
    label="$(printf '%s' "$label" | tr -d '\r\n"' | sed 's/^ *//; s/ *$//')"
    [[ -z "$label" ]] && exit 0
    # Refuse a model that answered with prose instead of a label.
    (( ${#label} > max_len * 2 )) && exit 0

    tmp="$(mktemp "${label_file}.XXXXXX")" || exit 0
    printf '%s\n%s\n' "$label" "$ai_title" > "$tmp"
    mv -f "$tmp" "$label_file"
) >/dev/null 2>&1 &

exit 0
