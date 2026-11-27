macro_rules! inherit_signer {
    ($struct_name:ident.$attr:ident) => {
        impl $struct_name {
            /// Sign the digest produced by `fill`. `fill` receives a fresh hasher.
            pub fn sign_digest<D, F>(&self, fill: F) -> secp256k1::ecdsa::Signature
            where
                D: digest::FixedOutput
                    + digest::OutputSizeUser<OutputSize = digest::consts::U32>
                    + Default,
                F: FnOnce(&mut D),
            {
                let _ = stringify!($attr);
                let key: &secp256k1::SecretKey = self.as_ref();
                crate::curve::sign(key, &crate::curve::fill_digest(fill))
            }

            /// Sign a pre-filled hasher.
            pub fn sign_hasher<D>(
                &self,
                digest: D,
            ) -> Result<secp256k1::ecdsa::Signature, crate::Bip32Error>
            where
                D: digest::FixedOutput + digest::OutputSizeUser<OutputSize = digest::consts::U32>,
            {
                let key: &secp256k1::SecretKey = self.as_ref();
                Ok(crate::curve::sign(
                    key,
                    &crate::curve::finalize_digest(digest),
                ))
            }

            /// Sign a pre-filled hasher and return the recovery id.
            pub fn sign_digest_recoverable<D>(
                &self,
                digest: D,
            ) -> Result<
                (secp256k1::ecdsa::Signature, secp256k1::ecdsa::RecoveryId),
                crate::Bip32Error,
            >
            where
                D: digest::FixedOutput + digest::OutputSizeUser<OutputSize = digest::consts::U32>,
            {
                let key: &secp256k1::SecretKey = self.as_ref();
                Ok(crate::curve::sign_with_recovery_id(
                    key,
                    &crate::curve::finalize_digest(digest),
                ))
            }
        }
    };
}

macro_rules! inherit_verifier {
    ($struct_name:ident.$attr:ident) => {
        impl $struct_name {
            /// Compressed SEC1 encoding of the public key (33 bytes).
            pub fn to_sec1_bytes(&self) -> [u8; 33] {
                let _ = stringify!($attr);
                let key: &secp256k1::PublicKey = self.as_ref();
                crate::curve::compressed_serialize(key)
            }

            /// Verify `signature` against the digest produced by `fill`.
            pub fn verify_digest<D, F>(
                &self,
                fill: F,
                signature: &secp256k1::ecdsa::Signature,
            ) -> Result<(), crate::Bip32Error>
            where
                D: digest::FixedOutput
                    + digest::OutputSizeUser<OutputSize = digest::consts::U32>
                    + Default,
                F: FnOnce(&mut D) -> Result<(), crate::Bip32Error>,
            {
                let mut hasher = D::default();
                fill(&mut hasher)?;
                let key: &secp256k1::PublicKey = self.as_ref();
                crate::curve::verify(key, &crate::curve::finalize_digest(hasher), signature)
            }

            /// Verify `signature` against a pre-filled hasher.
            pub fn verify_hasher<D>(
                &self,
                digest: D,
                signature: &secp256k1::ecdsa::Signature,
            ) -> Result<(), crate::Bip32Error>
            where
                D: digest::FixedOutput + digest::OutputSizeUser<OutputSize = digest::consts::U32>,
            {
                let key: &secp256k1::PublicKey = self.as_ref();
                crate::curve::verify(key, &crate::curve::finalize_digest(digest), signature)
            }
        }
    };
}
