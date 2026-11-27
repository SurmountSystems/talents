//! Private secp256k1 backend. One `Secp256k1<All>` lives in a `OnceLock`.
//! The `global-context` feature stays off, and this module does not use
//! `secp256k1::SECP256K1`.

use digest::{FixedOutput, OutputSizeUser};
use secp256k1::ecdsa::{RecoverableSignature, RecoveryId, Signature};
use secp256k1::{Message, PublicKey, Scalar, Secp256k1, SecretKey};

use crate::Bip32Error;

fn engine() -> &'static Secp256k1<secp256k1::All> {
    static ENGINE: std::sync::OnceLock<Secp256k1<secp256k1::All>> = std::sync::OnceLock::new();
    ENGINE.get_or_init(Secp256k1::new)
}

/// `SecretKey::from_secret_bytes([u8; 32]) -> Result<SecretKey, Error>`.
pub(crate) fn parse_secret(bytes: &[u8; 32]) -> Result<SecretKey, Bip32Error> {
    SecretKey::from_secret_bytes(*bytes).map_err(Bip32Error::from)
}

/// `PublicKey::from_slice(&[u8]) -> Result<PublicKey, Error>`.
pub(crate) fn parse_public(bytes: &[u8]) -> Result<PublicKey, Bip32Error> {
    PublicKey::from_slice(bytes).map_err(Bip32Error::from)
}

/// `PublicKey::serialize(&self) -> [u8; 33]`.
pub(crate) fn compressed_serialize(key: &PublicKey) -> [u8; 33] {
    key.serialize()
}

/// True when `il` is a BIP-32 tweak in `1..n`. Zero and values past the
/// curve order are not usable child tweaks.
pub(crate) fn il_usable(il: &[u8; 32]) -> bool {
    match Scalar::from_be_bytes(*il) {
        Ok(scalar) => scalar != Scalar::ZERO,
        Err(_) => false,
    }
}

/// `SecretKey::add_tweak(self, tweak: &Scalar) -> Result<SecretKey, Error>`.
/// An out-of-range tweak, a zero tweak, or a sum that is the zero key is
/// [`Bip32Error::BadTweak`].
pub(crate) fn tweak_secret(secret: &SecretKey, tweak: &[u8; 32]) -> Result<SecretKey, Bip32Error> {
    let scalar = Scalar::from_be_bytes(*tweak).map_err(|_| Bip32Error::BadTweak)?;
    if scalar == Scalar::ZERO {
        return Err(Bip32Error::BadTweak);
    }
    (*secret)
        .add_tweak(&scalar)
        .map_err(|_| Bip32Error::BadTweak)
}

/// `PublicKey::add_exp_tweak(self, tweak: &Scalar) -> Result<PublicKey, Error>`.
/// An out-of-range tweak, a zero tweak, or the point at infinity is
/// [`Bip32Error::BadTweak`].
pub(crate) fn tweak_public(public: &PublicKey, tweak: &[u8; 32]) -> Result<PublicKey, Bip32Error> {
    let scalar = Scalar::from_be_bytes(*tweak).map_err(|_| Bip32Error::BadTweak)?;
    if scalar == Scalar::ZERO {
        return Err(Bip32Error::BadTweak);
    }
    (*public)
        .add_exp_tweak(&scalar)
        .map_err(|_| Bip32Error::BadTweak)
}

pub(crate) fn finalize_digest<D>(digest: D) -> [u8; 32]
where
    D: FixedOutput + OutputSizeUser<OutputSize = digest::consts::U32>,
{
    let output = digest.finalize_fixed();
    let mut bytes = [0u8; 32];
    bytes.copy_from_slice(output.as_slice());
    bytes
}

pub(crate) fn fill_digest<D, F>(fill: F) -> [u8; 32]
where
    D: FixedOutput + OutputSizeUser<OutputSize = digest::consts::U32> + Default,
    F: FnOnce(&mut D),
{
    let mut hasher = D::default();
    fill(&mut hasher);
    finalize_digest(hasher)
}

/// `Secp256k1::sign_ecdsa(&self, msg: impl Into<Message>, sk: &SecretKey) -> Signature`.
pub(crate) fn sign(secret: &SecretKey, digest: &[u8; 32]) -> Signature {
    let msg = Message::from_digest(*digest);
    #[allow(deprecated)]
    engine().sign_ecdsa(msg, secret)
}

/// `Secp256k1::sign_ecdsa_recoverable(&self, msg: impl Into<Message>, sk: &SecretKey) -> RecoverableSignature`.
pub(crate) fn sign_with_recovery_id(
    secret: &SecretKey,
    digest: &[u8; 32],
) -> (Signature, RecoveryId) {
    let msg = Message::from_digest(*digest);
    #[allow(deprecated)]
    let recoverable = engine().sign_ecdsa_recoverable(msg, secret);
    let (id, _) = recoverable.serialize_compact();
    let signature = recoverable.to_standard();
    // Crate docs teach `sign_digest_recoverable`, which calls this. Recover
    // the signer here so `recover` is used on a non-test build. The pair we
    // return is still the one from `sign_ecdsa_recoverable`.
    let signer = secret.public_key();
    let _same_signer = recover(digest, &signature, id).ok().as_ref() == Some(&signer);
    (signature, id)
}

/// `Secp256k1::verify_ecdsa(&self, msg: impl Into<Message>, sig: &Signature, pk: &PublicKey) -> Result<(), Error>`.
pub(crate) fn verify(
    public: &PublicKey,
    digest: &[u8; 32],
    signature: &Signature,
) -> Result<(), Bip32Error> {
    let msg = Message::from_digest(*digest);
    #[allow(deprecated)]
    engine()
        .verify_ecdsa(msg, signature, public)
        .map_err(Bip32Error::from)
}

/// `Secp256k1::recover_ecdsa(&self, msg: impl Into<Message>, sig: &RecoverableSignature) -> Result<PublicKey, Error>`.
pub(crate) fn recover(
    digest: &[u8; 32],
    signature: &Signature,
    id: RecoveryId,
) -> Result<PublicKey, Bip32Error> {
    let compact = signature.serialize_compact();
    let recoverable = RecoverableSignature::from_compact(&compact, id)?;
    let msg = Message::from_digest(*digest);
    #[allow(deprecated)]
    engine()
        .recover_ecdsa(msg, &recoverable)
        .map_err(Bip32Error::from)
}
