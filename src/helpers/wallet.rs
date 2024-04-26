use aries_askar::kms::{KeyAlg, LocalKey};
use indy_data_types::did::{generate_did, DidValue};
use indy_data_types::keys::PrivateKey;
use sha2::{Digest, Sha256};

pub struct IndyWallet {
    pub did: String,
    pub verkey: String,
    pub key: LocalKey,
}

impl IndyWallet {
    pub async fn new(seed: Option<&str>, did_version_value: usize) -> anyhow::Result<IndyWallet> {
        // let key = match seed {
        //     Some(seed) => LocalKey::from_secret_bytes(KeyAlg::Ed25519, seed.as_bytes()).unwrap(),
        //     None => LocalKey::generate(KeyAlg::Ed25519, false).unwrap(),
        // };
        // let verkey_bytes = key.to_public_bytes().unwrap();
        // let did = bs58::encode(&verkey_bytes[0..16]).into_string();
        // let verkey = bs58::encode(verkey_bytes.as_ref()).into_string();
        let (did, key, verkey) = IndyWallet::create_did(seed, Some(did_version_value)).await?;
        anyhow::Ok(IndyWallet { did, verkey, key })
    }

    pub async fn sign(&self, bytes: &[u8]) -> Vec<u8> {
        self.key.sign_message(bytes, None).unwrap()
    }

    pub async fn create_did(
        seed: Option<&str>,
        version: Option<usize>,
    ) -> anyhow::Result<(String, LocalKey, String)> {
        let key = match seed {
            Some(seed) => LocalKey::from_secret_bytes(KeyAlg::Ed25519, seed.as_bytes()).unwrap(),
            None => LocalKey::generate(KeyAlg::Ed25519, false).unwrap(),
        };

        let verkey_bytes = key.to_public_bytes().unwrap();
        let did = match version {
            Some(1) => anyhow::Ok(bs58::encode(&verkey_bytes.as_ref()[..16]).into_string()),
            Some(2) | None => {
                let mut hasher = Sha256::new();
                Digest::update(&mut hasher, verkey_bytes.as_ref());
                let hash = hasher.finalize();
                anyhow::Ok(bs58::encode(&hash[..16]).into_string())
            }
            _ => Err(anyhow::anyhow!("Version must be one of 1,2")),
        }?;
        // let did = bs58::encode(&verkey_bytes[0..16]).into_string();
        let verkey = bs58::encode(verkey_bytes.as_ref()).into_string();

        anyhow::Ok((did, key, verkey))
    }
}
