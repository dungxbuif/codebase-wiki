#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
codewiki-helper.sh [codewiki-args...]

Run the CodeWiki Rust companion from an installed CodeWiki skill.

Resolution order:
  1. CODEWIKI_COMPANION_BIN, when it points to an executable.
  2. codewiki from PATH.
  3. cargo run from CODEWIKI_REPO.
  4. cargo run from the source checkout that contains this skill.

Examples:
  scripts/codewiki-helper.sh status
  CODEWIKI_REPO=/path/to/codebase-wiki scripts/codewiki-helper.sh init /repo
  CODEWIKI_COMPANION_BIN=/usr/local/bin/codewiki scripts/codewiki-helper.sh sync /repo
USAGE
}

if [[ "${1:-}" == "-h" || "${1:-}" == "--help" ]]; then
  usage
  exit 0
fi

if [[ -n "${CODEWIKI_COMPANION_BIN:-}" ]]; then
  if [[ ! -x "$CODEWIKI_COMPANION_BIN" ]]; then
    echo "CODEWIKI_COMPANION_BIN is set but is not executable: $CODEWIKI_COMPANION_BIN" >&2
    exit 127
  fi
  exec "$CODEWIKI_COMPANION_BIN" "$@"
fi

if command -v codewiki >/dev/null 2>&1; then
  exec codewiki "$@"
fi

run_from_repo() {
  local repo="$1"
  shift
  if [[ -f "$repo/Cargo.toml" && -d "$repo/crates/codewiki-cli" ]]; then
    exec cargo run --manifest-path "$repo/Cargo.toml" -p codewiki-cli -- "$@"
  fi
}

if [[ -n "${CODEWIKI_REPO:-}" ]]; then
  run_from_repo "$CODEWIKI_REPO" "$@"
fi

script_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
run_from_repo "$script_dir/../companion" "$@"

candidate="$script_dir"
while [[ "$candidate" != "/" ]]; do
  run_from_repo "$candidate" "$@"
  candidate="$(dirname "$candidate")"
done

cat >&2 <<'ERROR'
Could not locate the CodeWiki Rust companion.

Set one of:
  CODEWIKI_COMPANION_BIN=/path/to/codewiki
  CODEWIKI_REPO=/path/to/codebase-wiki

Or install/build a `codewiki` binary on PATH.
ERROR
exit 127
