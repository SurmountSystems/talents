# Local and remote Nix checks for Talents, plus lock and publish recipes.
# Core counts are read at runtime. Nothing here is a fixed host size.

check:
    #!/usr/bin/env bash
    set -euo pipefail
    half=$(( $(nproc) / 2 ))
    if (( half < 1 )); then
      half=1
    fi
    nice -n 19 nix flake check --option cores "$half" --max-jobs "$half"

check-remote:
    #!/usr/bin/env bash
    set -euo pipefail
    builders="$(nix config show builders)"
    # Non-empty configured builders stay as they are. Nix 2.36 builder specs
    # have no niceness field, so this does not rewrite the list to add one.
    if [[ -n "${builders//[[:space:]]/}" ]]; then
      nice -n 19 nix flake check --max-jobs 0
      exit 0
    fi
    # Pure `nix eval --raw --expr builtins.currentSystem` fails on Nix 2.36:
    # attribute 'currentSystem' missing. Impure eval is the form that works.
    if ! system="$(nix eval --impure --raw --expr builtins.currentSystem)"; then
      echo "check-remote: nix eval --impure --raw --expr builtins.currentSystem failed" >&2
      exit 1
    fi
    if ! remote_nproc="$(ssh nixbuilder@surmount-1 nproc)"; then
      echo "check-remote: ssh nixbuilder@surmount-1 nproc failed" >&2
      exit 1
    fi
    if [[ ! "$remote_nproc" =~ ^[0-9]+$ ]]; then
      echo "check-remote: ssh nixbuilder@surmount-1 nproc did not print an integer: ${remote_nproc}" >&2
      exit 1
    fi
    half=$(( remote_nproc / 2 ))
    if (( half < 1 )); then
      half=1
    fi
    # Field order from the Nix 2.34 manual (builders): URI, systems, SSH key,
    # max-jobs, speed factor, supported features, required features, public host key.
    # There is no niceness column. Local niceness is the nice(1) wrapper.
    spec="ssh-ng://nixbuilder@surmount-1 ${system} - ${half} 1 - - -"
    nice -n 19 nix flake check \
      --max-jobs 0 \
      --builders "$spec" \
      --option builders-use-substitutes true

# Fail if Cargo.lock or flake.lock would change. Does not write either lock.
check-locks:
    #!/usr/bin/env bash
    set -euo pipefail
    half=$(( $(nproc) / 2 ))
    if (( half < 1 )); then
      half=1
    fi
    echo "check-locks: cargo metadata --locked"
    nice -n 19 cargo metadata --locked --format-version 1 >/dev/null
    echo "check-locks: nix flake metadata --no-update-lock-file"
    nice -n 19 nix flake metadata --no-update-lock-file --option cores "$half" --max-jobs "$half" .

# Add missing flake.lock entries. Does not bump inputs that are already locked.
fill-locks:
    #!/usr/bin/env bash
    set -euo pipefail
    half=$(( $(nproc) / 2 ))
    if (( half < 1 )); then
      half=1
    fi
    nice -n 19 nix flake lock --option cores "$half" --max-jobs "$half"

# Bump every flake input and refresh Cargo.lock.
update-locks:
    #!/usr/bin/env bash
    set -euo pipefail
    half=$(( $(nproc) / 2 ))
    if (( half < 1 )); then
      half=1
    fi
    nice -n 19 nix flake update --option cores "$half" --max-jobs "$half"
    nice -n 19 cargo update

# Package every public crate. No arguments: dry-run. Pass --upload to upload.
publish *args:
    #!/usr/bin/env bash
    set -euo pipefail
    bash scripts/publish.sh {{ args }}
