pub use crate::Bip32Error;
pub use crate::derived::{DerivedKey, DerivedPubkey, DerivedXPriv, DerivedXPub};
pub use crate::enc::{MAINNET, NetworkVersions, TESTNET};
pub use crate::path::KeyDerivation;
pub use crate::primitives::*;
pub use crate::xkeys::{Parent, XPriv, XPub};

/// Re-exported secp256k1 types.
pub use secp256k1;
pub use secp256k1::{
    PublicKey, SecretKey,
    ecdsa::{RecoveryId, Signature},
};

/// shortcut for easy usage
pub fn fingerprint_of(k: &PublicKey) -> KeyFingerprint {
    use talents_core::hashes::Digest;
    let digest = talents_core::hashes::Hash160::digest(k.serialize());
    let mut fingerprint = [0u8; 4];
    fingerprint.copy_from_slice(&digest[..4]);
    fingerprint.into()
}
