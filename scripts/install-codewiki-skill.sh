#!/usr/bin/env bash
set -euo pipefail

REPO_URL="${CODEWIKI_REPO_URL:-https://github.com/dungxbuif/codebase-wiki.git}"
SKILL_NAME="${CODEWIKI_SKILL_NAME:-codewiki}"
CODEX_HOME="${CODEX_HOME:-$HOME/.codex}"
INSTALL_ROOT="$CODEX_HOME/skills"
INSTALL_DIR="$INSTALL_ROOT/$SKILL_NAME"

usage() {
  cat <<'USAGE'
Install the CodeWiki Codex skill from its repository.

Usage:
  scripts/install-codewiki-skill.sh

Environment:
  CODEWIKI_REPO_URL   Git URL to install from. Default: https://github.com/dungxbuif/codebase-wiki.git
  CODEX_HOME          Codex home. Default: ~/.codex
  CODEWIKI_SKILL_NAME Skill install folder. Default: codewiki

One-command install after this repo is pushed:
  curl -fsSL https://raw.githubusercontent.com/dungxbuif/codebase-wiki/master/scripts/install-codewiki-skill.sh | bash
USAGE
}

if [[ "${1:-}" == "-h" || "${1:-}" == "--help" ]]; then
  usage
  exit 0
fi

tmpdir="$(mktemp -d)"
cleanup() {
  rm -rf "$tmpdir"
}
trap cleanup EXIT

git clone --depth 1 "$REPO_URL" "$tmpdir/repo" >/dev/null

if [[ ! -f "$tmpdir/repo/skill/codewiki/SKILL.md" ]]; then
  echo "error: skill/codewiki/SKILL.md not found in $REPO_URL" >&2
  exit 1
fi

mkdir -p "$INSTALL_ROOT"
rm -rf "$INSTALL_DIR"
cp -R "$tmpdir/repo/skill/codewiki" "$INSTALL_DIR"
if [[ -f "$tmpdir/repo/Cargo.toml" && -d "$tmpdir/repo/crates/codewiki-cli" ]]; then
  mkdir -p "$INSTALL_DIR/companion"
  cp "$tmpdir/repo/Cargo.toml" "$INSTALL_DIR/companion/Cargo.toml"
  if [[ -f "$tmpdir/repo/Cargo.lock" ]]; then
    cp "$tmpdir/repo/Cargo.lock" "$INSTALL_DIR/companion/Cargo.lock"
  fi
  cp -R "$tmpdir/repo/crates" "$INSTALL_DIR/companion/crates"

  if command -v cargo >/dev/null 2>&1; then
    mkdir -p "$INSTALL_DIR/bin"
    if cargo build --release --manifest-path "$tmpdir/repo/Cargo.toml" -p codewiki-cli >/dev/null; then
      cp "$tmpdir/repo/target/release/codewiki" "$INSTALL_DIR/bin/codewiki"
      chmod +x "$INSTALL_DIR/bin/codewiki"
    else
      echo "warning: cargo build failed; installed companion source fallback but did not build bin/codewiki" >&2
    fi
  else
    echo "warning: cargo not found; installed companion source fallback but did not build bin/codewiki" >&2
  fi
fi
if compgen -G "$INSTALL_DIR/scripts/*.sh" >/dev/null; then
  chmod +x "$INSTALL_DIR"/scripts/*.sh
fi

echo "Installed CodeWiki skill to $INSTALL_DIR"
