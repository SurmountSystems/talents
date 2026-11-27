//! AI-written tests for extended keys. A submodule of `src/xkeys.rs`.

use super::*;
use crate::Bip32Error;
use crate::enc::TESTNET;

/// BIP-32 test vector 1, master xprv.
const VECTOR_1_XPRV: &str = "xprv9s21ZrQH143K3QTDL4LXw2F7HEK3wJUD2nW2nRk4stbPy6cq3jPPqjiChkVvvNKmPGJxWUtg6LnF5kejMRNNU3TGtRBeJgk33yuGBxrMPHi";

#[test]
fn xpriv_serde_round_trips() {
    let xpriv = XPriv::root_from_seed(&[7u8; 32], None).unwrap();
    let json = serde_json::to_string(&xpriv).unwrap();
    assert!(json.starts_with("\"zprv"), "{json}");
    let back: XPriv = serde_json::from_str(&json).unwrap();
    assert_eq!(back, xpriv);
}

#[test]
fn xpub_serde_round_trips() {
    let xpub = XPriv::root_from_seed(&[7u8; 32], None)
        .unwrap()
        .verify_key();
    let json = serde_json::to_string(&xpub).unwrap();
    assert!(json.starts_with("\"zpub"), "{json}");
    let back: XPub = serde_json::from_str(&json).unwrap();
    assert_eq!(back, xpub);
}

#[test]
fn equality_ignores_network() {
    let a = XPriv::root_from_seed(&[7u8; 32], None).unwrap();
    let mut b = a.clone();
    b.set_network(&TESTNET);
    assert_eq!(a, b);
    assert_eq!(a.verify_key(), b.verify_key());
    let (ia, ib): (&XKeyInfo, &XKeyInfo) = (a.as_ref(), b.as_ref());
    assert_eq!(ia, ib);
}

#[test]
fn testnet_xpriv_serde_round_trips_losslessly() {
    let mut xpriv = XPriv::root_from_seed(&[7u8; 32], Some(Hint::Legacy)).unwrap();
    xpriv.set_network(&TESTNET);
    let json = serde_json::to_string(&xpriv).unwrap();
    assert!(json.starts_with("\"tprv"), "{json}");
    let back: XPriv = serde_json::from_str(&json).unwrap();
    assert_eq!(serde_json::to_string(&back).unwrap(), json);
    let info: &XKeyInfo = back.as_ref();
    assert_eq!(info.network, &TESTNET);
}

#[test]
fn xpriv_parses_from_str() {
    let xpriv: XPriv = VECTOR_1_XPRV.parse().unwrap();
    assert_eq!(xpriv.to_base58().as_str(), VECTOR_1_XPRV);
}

#[test]
fn keys_deserialize_from_owned_string_sources() {
    let xpriv = XPriv::root_from_seed(&[7u8; 32], None).unwrap();
    let xpub = xpriv.verify_key();
    // `from_value` hands the visitor owned strings; `from_reader` hands it transient ones.
    let priv_value = serde_json::to_value(&xpriv).unwrap();
    let pub_value = serde_json::to_value(xpub).unwrap();
    assert_eq!(serde_json::from_value::<XPriv>(priv_value).unwrap(), xpriv);
    assert_eq!(serde_json::from_value::<XPub>(pub_value).unwrap(), xpub);
    let priv_bytes = serde_json::to_vec(&xpriv).unwrap();
    let pub_bytes = serde_json::to_vec(&xpub).unwrap();
    assert_eq!(
        serde_json::from_reader::<_, XPriv>(&priv_bytes[..]).unwrap(),
        xpriv
    );
    assert_eq!(
        serde_json::from_reader::<_, XPub>(&pub_bytes[..]).unwrap(),
        xpub
    );
}

/// Secret scalar 1. Not produced by running the backend.
const SECRET_ONE: [u8; 32] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
];

/// secp256k1 order minus one. Adding it to scalar 1 is 0 mod n.
const ORDER_MINUS_ONE: [u8; 32] = [
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xfe,
    0xba, 0xae, 0xdc, 0xe6, 0xaf, 0x48, 0xa0, 0x3b, 0xbf, 0xd2, 0x5e, 0x8c, 0xd0, 0x36, 0x41, 0x40,
];

#[test]
fn out_of_range_tweak_or_zero_key_returns_bad_tweak() {
    let secret = crate::curve::parse_secret(&SECRET_ONE).unwrap();
    let out_of_range = [0xff; 32];
    assert!(matches!(
        crate::curve::tweak_secret(&secret, &out_of_range),
        Err(Bip32Error::BadTweak)
    ));
    assert!(matches!(
        crate::curve::tweak_secret(&secret, &ORDER_MINUS_ONE),
        Err(Bip32Error::BadTweak)
    ));
    let public = secret.public_key();
    assert!(matches!(
        crate::curve::tweak_public(&public, &out_of_range),
        Err(Bip32Error::BadTweak)
    ));
}
