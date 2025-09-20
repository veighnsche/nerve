#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
TODO_FILE="${REPO_ROOT}/todo.md"
DONE_DIR="${REPO_ROOT}/.done"

if [[ ! -f "${TODO_FILE}" ]]; then
  echo "todo.md not found at repo root: ${TODO_FILE}" >&2
  exit 1
fi

mkdir -p "${DONE_DIR}"

next_index=1
shopt -s nullglob
for archived in "${DONE_DIR}"/*.md; do
  base="$(basename "${archived}")"
  if [[ ${base} =~ ^([0-9]+)_ ]]; then
    value=$((10#${BASH_REMATCH[1]}))
    if (( value >= next_index )); then
      next_index=$((value + 1))
    fi
  fi
done
shopt -u nullglob

slot=$(printf "%03d" "${next_index}")
target="${DONE_DIR}/${slot}_todo.md"

mv "${TODO_FILE}" "${target}"

echo "Archived todo.md to ${target}" >&2
