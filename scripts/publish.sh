#!/usr/bin/env bash
# Package the public Talents crates.
# No arguments: dry-run only. --upload performs the real upload.
# Never prints a crates.io token or credential file.

set -euo pipefail

repo="$(cd "$(dirname "$0")/.." && pwd)"
cd "$repo"

if [[ ! -f Cargo.toml || ! -f Cargo.lock ]]; then
  echo "publish: Cargo.toml or Cargo.lock is missing in ${repo}" >&2
  exit 1
fi

if [[ $# -gt 1 ]]; then
  echo "publish: expected no arguments, or exactly --upload" >&2
  exit 2
fi

upload=0
if [[ $# -eq 1 ]]; then
  case "$1" in
    --upload)
      upload=1
      ;;
    *)
      echo "publish: expected no arguments, or exactly --upload" >&2
      exit 2
      ;;
  esac
fi

half=$(( $(nproc) / 2 ))
if (( half < 1 )); then
  half=1
fi

# One publish of the explicit set, not Cargo's default members.
# Separate publishes cannot select talents-core or talents-bip32:
# those crates are not on crates.io yet. Cargo packages this set together.
packages=(
  --package talents-core
  --package talents-bip32
  --package talents-bip39
  --package talents-ledger
)

if (( upload == 1 )); then
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
