#![warn(
    missing_docs,
    missing_copy_implementations,
    missing_debug_implementations,
    unreachable_pub,
    unused_crate_dependencies,
    clippy::missing_const_for_fn
)]

//! The bip39 crate is heavily inspired by and reuses code from
//! [Wagyu](https://github.com/AleoHQ/wagyu) under the [MIT](http://opensource.org/licenses/MIT)
//! license. Extended private keys come from talents-bip32. That crate uses
//! libsecp256k1 through the Rust `secp256k1` crate, which talents-bip32 re-exports.

/// Mnemonic phrases
pub mod mnemonic;
pub use self::mnemonic::*;

/// Wordlists
pub mod wordlist;
pub use self::wordlist::*;

// Only used by tests gated on the `english` feature.
#[cfg(test)]
use hex as _;
