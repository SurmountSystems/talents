# Talents

Talents is tooling for BIP32, BIP39, and Ledger device communication. It is a toolbox for building signers for blockchain applications.

`talents-core`, `talents-bip32`, and `talents-bip39` build natively and for `wasm32-unknown-unknown`. `talents-ledger` is native only. The Nix checks do not build it for wasm, do not enable a browser feature, and do not run a Speculos emulator.

`libsecp256k1` is compiled through the `secp256k1` crate. On Linux, the ledger hidraw backend links libudev.

## Checks

`just check` runs `nix flake check` on this machine. The Nix client is niced to 19, and the job and core counts are half of `nproc` at runtime (at least 1).

`just check-remote` runs `nix flake check` on the builders already configured for Nix (`nix config show builders`). If that list is empty, it sends the build to `ssh-ng://nixbuilder@surmount-1`, with max-jobs set from half of that machine's `nproc`. No environment variable is required.

`nix develop` provides stable Rust 1.100 (rustfmt, clippy, and the wasm32-unknown-unknown target), cargo-audit, just, pkg-config, clang, and a C toolchain. On Linux it also provides libudev and the hidraw kernel headers.

## Status

There has not been a comprehensive security review. Interfaces can change.

## License

All work here is published under a maximally permissive public domain license due to recent legal license abuse.

Statist restrictions run counter to Kirckhoff's principle.

+
