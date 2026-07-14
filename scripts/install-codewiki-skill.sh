#!/usr/bin/env bash
set -euo pipefail

REPO_URL="${CODEWIKI_REPO_URL:-https://github.com/dungxbuif/codebase-wiki.git}"
SKILL_NAME="${CODEWIKI_SKILL_NAME:-codewiki}"
CODEX_HOME="${CODEX_HOME:-$HOME/.codex}"
INSTALL_SCOPE="${CODEWIKI_INSTALL_SCOPE:-local}"
TARGET_DIR="${CODEWIKI_TARGET_DIR:-$PWD}"
if [[ "$INSTALL_SCOPE" == "global" ]]; then
  INSTALL_ROOT="$CODEX_HOME/skills"
else
  INSTALL_ROOT="$TARGET_DIR/.agents/skills"
fi
INSTALL_DIR="$INSTALL_ROOT/$SKILL_NAME"

usage() {
  cat <<'USAGE'
Install the CodeWiki Codex skill from its repository.

Usage:
  scripts/install-codewiki-skill.sh

Environment:
  CODEWIKI_REPO_URL   Git URL to install from. Default: https://github.com/dungxbuif/codebase-wiki.git
  CODEWIKI_TARGET_DIR  Target code folder for local install. Default: current directory
  CODEWIKI_INSTALL_SCOPE Install scope: local or global. Default: local
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
stage_root=""
cleanup() {
  rm -rf "$tmpdir"
  if [[ -n "$stage_root" && -d "$stage_root" ]]; then
    rm -rf "$stage_root"
  fi
}
trap cleanup EXIT

SOURCE_DIR=""
SCRIPT_PATH="${BASH_SOURCE[0]:-$0}"
SCRIPT_DIR="$(cd "$(dirname "$SCRIPT_PATH")" >/dev/null 2>&1 && pwd || true)"
if [[ -n "${CODEWIKI_SOURCE_DIR:-}" ]]; then
  SOURCE_DIR="$CODEWIKI_SOURCE_DIR"
elif [[ -f "$SCRIPT_DIR/../skill/codewiki/SKILL.md" ]]; then
  SOURCE_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
else
  git clone --depth 1 "$REPO_URL" "$tmpdir/repo" >/dev/null
  SOURCE_DIR="$tmpdir/repo"
fi

if [[ ! -f "$SOURCE_DIR/skill/codewiki/SKILL.md" ]]; then
  echo "error: skill/codewiki/SKILL.md not found in $SOURCE_DIR" >&2
  exit 1
fi
if [[ ! -f "$SOURCE_DIR/skill/codewiki/package.yml" ]]; then
  echo "error: skill/codewiki/package.yml not found in $SOURCE_DIR" >&2
  exit 1
fi

mkdir -p "$INSTALL_ROOT"
stage_root="$(mktemp -d "$INSTALL_ROOT/.codewiki-stage.XXXXXX")"
STAGE_DIR="$stage_root/$SKILL_NAME"
cp -R "$SOURCE_DIR/skill/codewiki" "$STAGE_DIR"

package_value() {
  local key="$1"
  awk -F: -v key="$key" '$1 == key { value=$2; gsub(/^[[:space:]\"]+|[[:space:]\"]+$/, "", value); print value; exit }' "$STAGE_DIR/package.yml"
}

for required_key in package_version skill_contract_version reference_contract_version companion_interface_version wikiplan_schema_min wikiplan_schema_max; do
  if [[ -z "$(package_value "$required_key")" ]]; then
    echo "error: package.yml lacks required field $required_key" >&2
    exit 1
  fi
done

if [[ -d "$INSTALL_DIR" ]]; then
  while IFS= read -r existing_path; do
    existing_name="$(basename "$existing_path")"
    case "$existing_name" in
      SKILL.md|agents|references|scripts|package.yml|bin|companion|INSTALLATION.yml|project) ;;
      *)
        echo "error: ownership conflict at $existing_path; move or explicitly classify it before reinstalling" >&2
        exit 1
        ;;
    esac
  done < <(find "$INSTALL_DIR" -mindepth 1 -maxdepth 1 -print)
fi

if [[ -d "$INSTALL_DIR/project" ]]; then
  rm -rf "$STAGE_DIR/project"
  cp -R "$INSTALL_DIR/project" "$STAGE_DIR/project"
fi

if [[ -f "$SOURCE_DIR/Cargo.toml" && -d "$SOURCE_DIR/crates/codewiki-cli" ]]; then
  mkdir -p "$STAGE_DIR/companion"
  cp "$SOURCE_DIR/Cargo.toml" "$STAGE_DIR/companion/Cargo.toml"
  if [[ -f "$SOURCE_DIR/Cargo.lock" ]]; then
    cp "$SOURCE_DIR/Cargo.lock" "$STAGE_DIR/companion/Cargo.lock"
  fi
  cp -R "$SOURCE_DIR/crates" "$STAGE_DIR/companion/crates"

  if command -v cargo >/dev/null 2>&1; then
    mkdir -p "$STAGE_DIR/bin"
    if cargo build --release --manifest-path "$SOURCE_DIR/Cargo.toml" -p codewiki-cli >/dev/null; then
      cp "$SOURCE_DIR/target/release/codewiki" "$STAGE_DIR/bin/codewiki"
      chmod +x "$STAGE_DIR/bin/codewiki"
    else
      echo "warning: cargo build failed; installed companion source fallback but did not build bin/codewiki" >&2
    fi
  else
    echo "warning: cargo not found; installed companion source fallback but did not build bin/codewiki" >&2
  fi
fi
if compgen -G "$STAGE_DIR/scripts/*.sh" >/dev/null; then
  chmod +x "$STAGE_DIR"/scripts/*.sh
fi

source_revision="unknown"
source_dirty="unknown"
if git -C "$SOURCE_DIR" rev-parse HEAD >/dev/null 2>&1; then
  source_revision="$(git -C "$SOURCE_DIR" rev-parse HEAD)"
  if [[ -n "$(git -C "$SOURCE_DIR" status --porcelain)" ]]; then
    source_dirty="true"
  else
    source_dirty="false"
  fi
fi

managed_digest="unavailable"
if [[ -x "$STAGE_DIR/bin/codewiki" ]]; then
  managed_digest="$($STAGE_DIR/bin/codewiki package-digest "$STAGE_DIR")"
fi

cat >"$STAGE_DIR/INSTALLATION.yml" <<EOF
schema_version: 1
package_version: "$(package_value package_version)"
skill_contract_version: $(package_value skill_contract_version)
reference_contract_version: $(package_value reference_contract_version)
companion_interface_version: $(package_value companion_interface_version)
source_revision: "$source_revision"
source_dirty: $source_dirty
managed_digest: "$managed_digest"
install_scope: "$INSTALL_SCOPE"
install_root: "$INSTALL_DIR"
installed_at_utc: "$(date -u +%Y-%m-%dT%H:%M:%SZ)"
installer_version: 2
EOF

if [[ -x "$STAGE_DIR/bin/codewiki" ]]; then
  "$STAGE_DIR/bin/codewiki" doctor "$STAGE_DIR" >/dev/null
else
  echo "warning: installed package is legacy_unverified until a compatible companion binary is built" >&2
fi

backup_dir=""
if [[ -e "$INSTALL_DIR" ]]; then
  backup_dir="$INSTALL_ROOT/.codewiki-backup.$$.${RANDOM:-0}"
  mv "$INSTALL_DIR" "$backup_dir"
fi
if ! mv "$STAGE_DIR" "$INSTALL_DIR"; then
  if [[ -n "$backup_dir" && -e "$backup_dir" ]]; then
    mv "$backup_dir" "$INSTALL_DIR"
  fi
  echo "error: failed to activate staged CodeWiki installation" >&2
  exit 1
fi
if [[ -n "$backup_dir" && -e "$backup_dir" ]]; then
  rm -rf "$backup_dir"
fi

rmdir "$stage_root"
stage_root=""
echo "Installed CodeWiki skill to $INSTALL_DIR"
if [[ -x "$INSTALL_DIR/bin/codewiki" ]]; then
  "$INSTALL_DIR/bin/codewiki" doctor "$INSTALL_DIR"
fi
