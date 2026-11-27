# Talents Core

`talents-core` contains utilities and traits used by the `talents-bip32` and
`talents-bip39` crates.

## Crate Layout

### Hashes

The hashes module provides utilities for newtyping hash outputs, including
sha2 and ripemd160. These newtypes are called `Marked__` and are
intended to be used for a specific purpose. E.g. `Hash256` is a marked type for
Bitcoin's double-sha2, while `Hash160` is a marked type for Bitcoin's
`ripemd160(sha2(x))`.
