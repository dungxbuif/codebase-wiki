#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
codewiki-helper.sh [codewiki-args...]

Run the CodeWiki Rust companion from an installed CodeWiki skill.

Resolution order:
  1. CODEWIKI_COMPANION_BIN, when it points to an executable.
  2. bundled bin/codewiki from the installed skill.
  3. codewiki from PATH.
  4. built target binaries or cargo run from CODEWIKI_REPO.
  5. built target binaries or cargo run from bundled companion/source checkout.

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

script_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
skill_root="$(cd -- "$script_dir/.." && pwd)"

exec_companion() {
  local binary="$1"
  shift
  case "${1:-}" in
    status|init|sync|validate)
      if [[ -f "$skill_root/INSTALLATION.yml" ]]; then
        "$binary" doctor "$skill_root" >/dev/null
      elif [[ "${1:-}" == "validate" || "${1:-}" == "status" ]]; then
        echo "CodeWiki installation is legacy_unverified; reinstall before marking reader docs ready" >&2
        exit 1
      else
        echo "warning: CodeWiki installation provenance is unavailable; generation cannot become reader_docs_ready until reinstalled" >&2
      fi
      ;;
  esac
  export CODEWIKI_SKILL_ROOT="$skill_root"
  exec "$binary" "$@"
}

if [[ -n "${CODEWIKI_COMPANION_BIN:-}" ]]; then
  if [[ ! -x "$CODEWIKI_COMPANION_BIN" ]]; then
    echo "CODEWIKI_COMPANION_BIN is set but is not executable: $CODEWIKI_COMPANION_BIN" >&2
    exit 127
  fi
  exec_companion "$CODEWIKI_COMPANION_BIN" "$@"
fi

bundled_bin="$script_dir/../bin/codewiki"
if [[ -x "$bundled_bin" ]]; then
  exec_companion "$bundled_bin" "$@"
fi

if command -v codewiki >/dev/null 2>&1; then
  exec_companion "$(command -v codewiki)" "$@"
fi

run_from_repo() {
  local repo="$1"
  shift
  if [[ -f "$repo/Cargo.toml" && -d "$repo/crates/codewiki-cli" ]]; then
    if [[ -x "$repo/target/release/codewiki" ]]; then
      exec_companion "$repo/target/release/codewiki" "$@"
    fi
    if [[ -x "$repo/target/debug/codewiki" ]]; then
      exec_companion "$repo/target/debug/codewiki" "$@"
    fi
    export CODEWIKI_SKILL_ROOT="$skill_root"
    exec cargo run --manifest-path "$repo/Cargo.toml" -p codewiki-cli -- "$@"
  fi
}

if [[ -n "${CODEWIKI_REPO:-}" ]]; then
  run_from_repo "$CODEWIKI_REPO" "$@"
fi

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

Or install/build a `codewiki` binary on PATH or in the installed skill's bin/ directory.
ERROR
exit 127
