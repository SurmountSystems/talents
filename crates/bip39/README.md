# talents-bip39

This is an implementation of [BIP39](https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki). It is heavily inspired by and reuses code from [Wagyu](https://github.com/AleoHQ/wagyu) under the [MIT](http://opensource.org/licenses/MIT) license. It uses talents-bip32 to derive extended keys. talents-bip32 uses libsecp256k1 through the Rust `secp256k1` crate, which talents-bip32 re-exports.

## Building

```
$ cargo build
$ cargo build --target wasm32-unknown-unknown
```

Run tests (make sure to run with all feature combinations):
```
$ cargo test
```
