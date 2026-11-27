# talents-bip32

This is an implementation of
[BIP32](https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki). It
uses [secp256k1](https://docs.rs/secp256k1/0.33.1/secp256k1/) and re-exports
the signature, recovery id, secret key, and public key types.

It can be used to build wallets and applications for Bitcoin and Ethereum.

## Building

```
$ cargo build
$ cargo build --target wasm32-unknown-unknown
```

Run tests:
```
$ cargo test
```

Run bench marks
```
$ cargo bench
```
