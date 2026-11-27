#!/usr/bin/env bash
# Package the public Talents crates in one cargo publish.
# just publish-dry-run checks without uploading.
# just publish --upload performs the real upload.
# No arguments: tell the operator which command to run, then exit.
# Never prints a crates.io token or credential file.

set -euo pipefail

repo="$(cd "$(dirname "$0")/.." && pwd)"
cd "$repo"

if [[ ! -f Cargo.toml || ! -f Cargo.lock ]]; then
  echo "publish: Cargo.toml or Cargo.lock is missing in ${repo}" >&2
  exit 1
fi

if [[ $# -gt 1 ]]; then
  echo "publish: expected no arguments, --dry-run, or exactly --upload" >&2
  exit 2
fi

mode=help
if [[ $# -eq 1 ]]; then
  case "$1" in
    --upload)
      mode=upload
      ;;
    --dry-run)
      mode=dry-run
      ;;
    *)
      echo "publish: expected no arguments, --dry-run, or exactly --upload" >&2
      exit 2
      ;;
  esac
fi

if [[ "$mode" == help ]]; then
  echo "publish: no arguments does not package or upload."
  echo "Use \`just publish-dry-run\` to check each crate without uploading."
  echo "Use \`just publish --upload\` to upload."
  exit 0
fi

half=$(( $(nproc) / 2 ))
if (( half < 1 )); then
  half=1
fi

# One publish of the explicit set. Separate publishes cannot select
# talents-bip32 or talents-bip39: those crates depend on talents-core,
# which is not on crates.io yet. Cargo packages this set together.
# Uploading lines may follow in a different order. That order is Cargo's.
packages=(
  --package talents-core
  --package talents-bip32
  --package talents-bip39
  --package talents-ledger
)

if [[ "$mode" == upload ]]; then
  echo "publish: upload talents-core, talents-bip32, talents-bip39, talents-ledger"
  nice -n 19 cargo publish \
    --locked \
    --allow-dirty \
    --jobs "$half" \
    "${packages[@]}"
else
  echo "publish: dry-run talents-core, talents-bip32, talents-bip39, talents-ledger"
  nice -n 19 cargo publish \
    --locked \
    --dry-run \
    --allow-dirty \
    --jobs "$half" \
    "${packages[@]}"
fi
