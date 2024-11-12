use anyhow::{anyhow, Context, Result};
use aries_askar::kms::{KeyAlg, LocalKey};
use sha2::{Digest, Sha256};

const ED25519_PUBLIC_KEY_LENGTH: usize = 32;

pub struct IndyWallet {
    pub did: String,
    pub verkey: String,
    pub key: LocalKey,
}

impl IndyWallet {
    pub async fn new(seed: Option<&str>, did_version_value: usize) -> anyhow::Result<IndyWallet> {
        let (did, key, verkey) = IndyWallet::create_did(seed, Some(did_version_value))
            .await
            .context("Failed to create DID")?;
        Ok(IndyWallet { did, verkey, key })
    }

    pub async fn sign(&self, bytes: &[u8]) -> Vec<u8> {
        self.key
            .sign_message(bytes, None)
            .expect("Signing operation failed")
    }

    pub async fn create_did(
        seed: Option<&str>,
        version: Option<usize>,
    ) -> anyhow::Result<(String, LocalKey, String)> {
        let key = match seed {
            Some(seed) => LocalKey::from_secret_bytes(KeyAlg::Ed25519, seed.as_bytes())
                .context("Failed to create key from seed")?,
            None => LocalKey::generate(KeyAlg::Ed25519, false)
                .context("Failed to generate random key")?,
        };

        // Get public key bytes
        let verkey_bytes = key
            .to_public_bytes()
            .context("Failed to get public key bytes")?;

        // Verify public key length
        if verkey_bytes.len() != ED25519_PUBLIC_KEY_LENGTH {
            return Err(anyhow!(
                "Invalid public key length: expected {}, got {}",
                ED25519_PUBLIC_KEY_LENGTH,
                verkey_bytes.len()
            ));
        }

        // Generate verkey (base58 of public key)
        let verkey = bs58::encode(verkey_bytes.as_ref()).into_string();

        let did = match version {
            Some(1) => {
                // DID:SOV - first 16 bytes of verkey
                bs58::encode(&verkey_bytes.as_ref()[..16]).into_string()
            }
            Some(2) | None => {
                // DID:INDY - first 16 bytes of SHA256(verkey)
                let mut hasher = Sha256::new();
                hasher.update(verkey_bytes.as_ref());
                let hash = hasher.finalize();
                bs58::encode(&hash[..16]).into_string()
            }
            _ => return Err(anyhow!("Invalid DID version: must be 1 or 2")),
        };

        // Verify DID-verkey relationship for DID:INDY
        if version == Some(2) || version.is_none() {
            Self::verify_did_verkey_relationship(&did, &verkey)
                .context("DID-verkey verification failed")?;
        }

        Ok((did, key, verkey))
    }

    /// Verifies that a DID was derived from a verkey according to DID:INDY specs
    fn verify_did_verkey_relationship(did: &str, verkey: &str) -> Result<()> {
        // Decode verkey from base58
        let verkey_bytes = bs58::decode(verkey)
            .into_vec()
            .context("Failed to decode verkey from base58")?;

        // Calculate hash
        let mut hasher = Sha256::new();
        hasher.update(&verkey_bytes);
        let hash = hasher.finalize();

        // Get expected DID
        let expected_did = bs58::encode(&hash[..16]).into_string();

        if did != expected_did {
            return Err(anyhow!(
                "DID-verkey mismatch: DID does not match first 16 bytes of SHA256(verkey)"
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_did_indy_generation() {
        let seed = "000000000000000000000000Trustee1";
        let (did, _, verkey) = IndyWallet::create_did(Some(seed), Some(2)).await.unwrap();

        // Verify DID-verkey relationship
        IndyWallet::verify_did_verkey_relationship(&did, &verkey).unwrap();
    }

    #[tokio::test]
    async fn test_invalid_verkey() {
        let invalid_verkey = bs58::encode(vec![0u8; 31]).into_string(); // Wrong length
        let did = "GAAguaTbEHjvxL6i64YmAo";

        assert!(IndyWallet::verify_did_verkey_relationship(did, &invalid_verkey).is_err());
    }
}
