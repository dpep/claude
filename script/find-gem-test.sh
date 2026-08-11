#!/usr/bin/env bash
# Tests for plugins/code/bin/find-gem. Hermetic: `bundle` and `rg` are
# stubbed on PATH, so no gem needs to be installed and no file is
# actually searched. `ruby` is the real one — find-gem depends on it.
set -uo pipefail

repo=$(cd "$(dirname "$0")/.." && pwd)
find_gem="$repo/plugins/code/bin/find-gem"
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT

gem_dir="$tmp/gems/widget-1.0.0"
mkdir -p "$gem_dir" "$tmp/bin"

cat >"$tmp/bin/bundle" <<EOF
#!/bin/sh
echo "$gem_dir"
EOF

# Stubbed rg: canned matches in whichever format was asked for, its own
# argv echoed back in text mode so pass-through is checkable, and a
# failure on demand. printf, not echo — /bin/sh's echo eats the
# backslash escapes that make this valid JSON.
cat >"$tmp/bin/rg" <<EOF
#!/bin/sh
case "\$*" in
  *BOOM*) echo "rg: bad pattern" >&2; exit 2 ;;
esac
# find-gem's own structured mode is the only caller that leads with
# --json; a user's --json after \`--\` must reach us as a plain argument.
case "\$1" in
  --json)
    printf '%s\n' '{"type":"begin","data":{"path":{"text":"$gem_dir/lib/widget.rb"}}}'
    printf '%s\n' '{"type":"match","data":{"path":{"text":"$gem_dir/lib/widget.rb"},"lines":{"text":"def plant \"sapling\"\n"},"line_number":7}}'
    printf '%s\n' '{"type":"match","data":{"path":{"text":"$gem_dir/lib/widget.rb"},"lines":{"bytes":"AAA="},"line_number":9}}'
    printf '%s\n' '{"type":"end","data":{}}'
    ;;
  *) printf '%s\n' "rg-args: \$*" ;;
esac
EOF
chmod +x "$tmp/bin/bundle" "$tmp/bin/rg"

# A PATH with the gem resolver but no rg, for the missing-tool check.
mkdir -p "$tmp/bin-norg"
cp "$tmp/bin/bundle" "$tmp/bin-norg/bundle"

export PATH="$tmp/bin:$PATH"
export BUNDLE_GEMFILE="$tmp/Gemfile"

failures=0
check() { # check <name> <expected> <actual>
  if [ "$2" = "$3" ]; then
    echo "ok   $1"
  else
    echo "FAIL $1"
    echo "     expected: $2"
    echo "     actual:   $3"
    failures=$((failures + 1))
  fi
}

check "locate (text)" \
  "$gem_dir" \
  "$("$find_gem" widget)"

check "locate (--json)" \
  "{\"gem\":\"widget\",\"dir\":\"$gem_dir\"}" \
  "$("$find_gem" widget --json)"

check "locate (-J is the same single object)" \
  "$("$find_gem" widget --json)" \
  "$("$find_gem" widget -J)"

check "search (text mode hands rg the pattern and the gem dir)" \
  "rg-args: --line-number plant $gem_dir" \
  "$("$find_gem" widget plant)"

check "search (rg flags pass through untouched)" \
  "rg-args: --line-number -i plant -g *.rb $gem_dir" \
  "$("$find_gem" widget -i plant -g '*.rb')"

# `--` is how a dash-leading pattern reaches rg: the reserved flags stop
# being ours past that point, so the mode stays text.
check "search (-- hands the rest to rg verbatim)" \
  "rg-args: --line-number -- --json $gem_dir" \
  "$("$find_gem" widget -- --json)"

check "search (rg's long forms don't collide)" \
  "rg-args: --line-number --threads 4 plant $gem_dir" \
  "$("$find_gem" widget --threads 4 plant)"

# Quotes in the matched line must survive as JSON; the non-UTF8 match
# (bytes rather than text) is dropped rather than emitted half-formed.
match='{"file":"'"$gem_dir"'/lib/widget.rb","line":7,"text":"def plant \"sapling\""}'

check "search (--json array)" \
  "[$match]" \
  "$("$find_gem" widget plant --json)"

check "search (-J one object per line)" \
  "$match" \
  "$("$find_gem" widget plant -J)"

# An rg failure must not leave a well-formed "no matches" document on
# stdout — that's what a JSON consumer would believe.
check "search (rg failure: no stdout, rg's exit code)" \
  "|2" \
  "$("$find_gem" widget BOOM --json 2>/dev/null; echo "|$?")"

# Help is a successful outcome; a missing gem name is not.
check "--help prints the whole block and exits 0" \
  "0|find-gem — locate an installed Ruby gem's source, optionally search inside it.|bundled version, otherwise the newest installed one." \
  "$("$find_gem" --help >"$tmp/help.txt"; printf '%s|%s|%s' "$?" "$(head -1 "$tmp/help.txt")" "$(tail -1 "$tmp/help.txt")")"

check "no arguments is a usage error on stderr" \
  "1|" \
  "$("$find_gem" >"$tmp/noargs.txt" 2>/dev/null; printf '%s|%s' "$?" "$(cat "$tmp/noargs.txt")")"

check "missing rg is named, not blamed on the gem" \
  "find-gem: rg is not on PATH" \
  "$(PATH="$tmp/bin-norg:/usr/bin:/bin" "$find_gem" widget plant 2>&1 >/dev/null)"

# Run from a scratch dir: no Gemfile in cwd, so this exercises the real
# ruby resolver rather than the stubbed bundle.
check "unknown gem exits 1" \
  "1" \
  "$(cd "$tmp" && PATH="${PATH#"$tmp/bin:"}" BUNDLE_GEMFILE= "$find_gem" no-such-gem-xyz >/dev/null 2>&1; echo $?)"

if [ "$failures" -gt 0 ]; then
  echo "$failures failure(s)"
  exit 1
fi
echo "find-gem: all checks passed"
