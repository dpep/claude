#!/usr/bin/env bash
# Run prlink's suite (plugins/code/bin/prlink). Gold cases clone their public
# repos into $TMPDIR/prlink-gold on first run, offline after that.
# Their expected output tracks trekr's main branch, which can be ahead of the
# released binary: point PRLINK_TREKR at a build of it.
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
exec ruby -e 'Dir["'"$ROOT"'/test/prlink/test_*.rb"].sort.each { |f| require f }' -- "$@"
