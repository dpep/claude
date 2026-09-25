#!/usr/bin/env bash
# Tests for plugins/code/bin/code-gc. Hermetic: builds throwaway repos and
# projects under a temp root, so nothing real is inspected or deleted.
set -uo pipefail

repo=$(CDPATH= cd "$(dirname "$0")/.." && pwd)
gc="$repo/plugins/code/bin/code-gc"
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT
root="$tmp/code"

export GIT_AUTHOR_NAME=t GIT_AUTHOR_EMAIL=t@example.com
export GIT_COMMITTER_NAME=t GIT_COMMITTER_EMAIL=t@example.com
export GIT_CONFIG_GLOBAL=/dev/null

failures=0
check() { # name, haystack, needle
  if printf '%s' "$2" | grep -qF -- "$3"; then echo "ok   $1"
  else echo "FAIL $1: expected '$3' in:"; printf '%s\n' "$2" | sed 's/^/     /'; failures=$((failures + 1)); fi
}
refute() { # name, haystack, needle
  if printf '%s' "$2" | grep -qF -- "$3"; then echo "FAIL $1: unexpected '$3' in:"; printf '%s\n' "$2" | sed 's/^/     /'; failures=$((failures + 1))
  else echo "ok   $1"; fi
}
age() { find "$1" -exec touch -t 202001010000 {} +; }

# Rust crate with a build dir, plus a bare target/ that isn't Cargo's.
mkdir -p "$root/crate/target/debug" "$root/site/target"
touch "$root/crate/Cargo.toml" "$root/crate/target/debug/big"

# One stale JS project, one active Python project.
mkdir -p "$root/old-js/node_modules/pkg" "$root/live-py/venv/lib"
touch "$root/old-js/index.js" "$root/old-js/node_modules/pkg/x.js" "$root/live-py/app.py" "$root/live-py/venv/lib/y.py"
age "$root/old-js"
age "$root/live-py/venv"

# Git repo: merged branch, squash-merged branch in a worktree, unmerged branch.
g="$root/proj"
git init -q -b main "$g"
git -C "$g" commit -q --allow-empty -m init
git -C "$g" branch merged
git -C "$g" switch -q -c squashed
echo a >"$g/a" && git -C "$g" add a && git -C "$g" commit -q -m a
git -C "$g" switch -q main
git -C "$g" cherry-pick squashed >/dev/null
git -C "$g" branch wip
git -C "$g" worktree add -q "$g/.claude/worktrees/wip" wip
echo b >"$g/.claude/worktrees/wip/b" && git -C "$g/.claude/worktrees/wip" add b && git -C "$g/.claude/worktrees/wip" commit -q -m b
echo dirty >"$g/.claude/worktrees/wip/c"
git -C "$g" worktree add -q "$g/.claude/worktrees/sq" squashed

# An installed binary symlinked into a build dir dies with it.
mkdir -p "$tmp/bin" && ln -s "$root/crate/target/debug/big" "$tmp/bin/big"
export CODE_GC_BIN_DIRS="$tmp/bin"

out=$("$gc" targets "$root")
check "targets: finds cargo target" "$out" "$root/crate/target"
check "targets: flags binaries linked into it" "$out" "$(printf 'relink\t%s' "$tmp/bin/big")"
refute "targets: ignores non-cargo target" "$out" "$root/site/target"
check "targets: prints total" "$out" "total"

out=$("$gc" deps "$root")
check "deps: finds stale project" "$out" "$root/old-js/node_modules"
refute "deps: keeps active project" "$out" "$root/live-py/venv"

out=$("$gc" deps --days 100000 "$root")
refute "deps: --days widens the window" "$out" "$root/old-js/node_modules"

out=$("$gc" worktrees "$root")
check "worktrees: unmerged + dirty counted" "$out" "$(printf '1\t1\twip\t')"
check "worktrees: cherry-picked reads as merged" "$out" "$(printf '0\t0\tsquashed\t')"
refute "worktrees: skips primary" "$out" "$(printf '\tmain\t')"

out=$("$gc" branches "$root")
check "branches: lists merged" "$out" "  merged"
refute "branches: skips unmerged" "$out" "  wip"

"$gc" targets "$root" >/dev/null
[ -d "$root/crate/target" ] && echo "ok   dry run deletes nothing" || { echo "FAIL dry run deleted"; failures=$((failures + 1)); }

"$gc" targets --apply "$root" >/dev/null
[ ! -d "$root/crate/target" ] && [ -d "$root/site/target" ] && echo "ok   targets --apply" || { echo "FAIL targets --apply"; failures=$((failures + 1)); }

"$gc" deps --apply "$root" >/dev/null
[ ! -d "$root/old-js/node_modules" ] && [ -d "$root/live-py/venv" ] && echo "ok   deps --apply" || { echo "FAIL deps --apply"; failures=$((failures + 1)); }

"$gc" branches --apply "$root" >/dev/null
branches=$(git -C "$g" branch --format='%(refname:short)')
refute "branches --apply: deletes merged" "$branches" "merged"
check "branches --apply: keeps unmerged" "$branches" "wip"

out=$("$gc" bogus 2>&1); code=$?
[ $code -eq 2 ] && echo "ok   unknown command exits 2" || { echo "FAIL unknown command exit $code"; failures=$((failures + 1)); }

echo
[ $failures -eq 0 ] && echo "all passed" || { echo "$failures failed"; exit 1; }
