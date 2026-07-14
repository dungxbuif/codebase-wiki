#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
codewiki-preflight.sh <init|sync> <repository-path>

Mandatory first write gate for CodeWiki generation and synchronization.
It verifies the installed package, runs companion evidence initialization/refresh,
and proves the project control plane exists before reader-facing synthesis begins.
USAGE
}

if [[ "$#" -ne 2 ]]; then
  usage >&2
  exit 2
fi

mode="$1"
repo="$2"
if [[ "$mode" != "init" && "$mode" != "sync" ]]; then
  usage >&2
  exit 2
fi
if [[ ! -d "$repo" ]]; then
  echo "CodeWiki preflight repository does not exist: $repo" >&2
  exit 2
fi

script_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
helper="$script_dir/codewiki-helper.sh"
if [[ ! -x "$helper" ]]; then
  echo "CodeWiki helper is missing or not executable: $helper" >&2
  exit 127
fi

"$helper" "$mode" "$repo"

control="$repo/.agents/skills/codewiki/project"
for required in config.yml plan.yml AGENTS.md sources.yml run.yml quality-report.yml; do
  if [[ ! -f "$control/$required" ]]; then
    echo "CodeWiki preflight failed: missing $control/$required" >&2
    exit 1
  fi
done
for required in README.md SOURCES.md CLAIMS.md COMMANDS.md; do
  if [[ ! -f "$repo/docs/evidence/$required" ]]; then
    echo "CodeWiki preflight failed: missing $repo/docs/evidence/$required" >&2
    exit 1
  fi
done
generation_status="$(sed -n 's/^generation_status: //p' "$control/run.yml" | head -n 1)"
if [[ "$mode" == "init" && "$generation_status" != "synthesis_incomplete" ]]; then
  echo "CodeWiki preflight failed: init must stop at synthesis_incomplete before reader synthesis" >&2
  exit 1
fi
if [[ "$mode" == "sync" && "$generation_status" != "synthesis_incomplete" && "$generation_status" != "reader_docs_ready" ]]; then
  echo "CodeWiki preflight failed: sync returned unsupported generation status: $generation_status" >&2
  exit 1
fi
if ! grep -q '^source_commit:' "$control/plan.yml" || ! grep -q '^source_dirty:' "$control/plan.yml"; then
  echo "CodeWiki preflight failed: WikiPlan source provenance is incomplete" >&2
  exit 1
fi

cat <<EOF
CodeWiki preflight complete
mode: $mode
repository: $repo
control_plane: $control
generation_status: $generation_status
next: if synthesis_incomplete, update the mental model and WikiPlan, synthesize reader pages, write quality-report.yml, then run codewiki-helper.sh validate "$repo"; if reader_docs_ready, preserve the no-op result
EOF
