use crate::helpers::wallet::IndyWallet;
use aries_askar::kms::LocalKey;
use indy_data_types::did::{generate_did, DidValue};
use indy_data_types::keys::{KeyType, PrivateKey};
use indy_vdr::common::error::VdrResult;
use indy_vdr::ledger::constants::UpdateRole;
use indy_vdr::ledger::RequestBuilder;
use indy_vdr::pool::helpers::perform_ledger_request;
use indy_vdr::pool::{PreparedRequest, ProtocolVersion, RequestResult, SharedPool};
use serde_json::Value;

pub fn create_did(seed: String, version: usize) -> anyhow::Result<DidInfo> {
    let (did, prv, vk) = generate_did(Some(seed.as_bytes()), Some(version))?;
    let endorser_did = DidInfo {
        did: DidValue::from(did.to_string()),
        privatekey: prv,
        verkey: vk.to_string(),
    };

    anyhow::Ok(endorser_did)
}

pub fn sign_transaction(data: DidInfo, txn: String) -> VdrResult<Value> {
    let mut req = PreparedRequest::from_request_json(txn)?;
    let sigin = req.get_signature_input()?;
    let sig = data.privatekey.sign(sigin.as_bytes()).unwrap();
    req.set_multi_signature(&data.did, &sig)?;
    Ok(req.req_json)
}

#[derive(Debug)]
pub struct DidInfo {
    pub(crate) did: DidValue,
    pub(crate) privatekey: PrivateKey,
    pub(crate) verkey: String,
}

impl Default for DidInfo {
    fn default() -> Self {
        Self {
            did: DidValue("".to_string()),
            privatekey: PrivateKey::new(vec![], None),
            verkey: String::default(),
        }
    }
}

/// Sign a NYM transaction request with a DID
pub fn sign_nym_request(nym_request: &mut PreparedRequest, trustee: &DidInfo) -> VdrResult<()> {
    let sigin = nym_request.get_signature_input()?;
    let sig = trustee.privatekey.sign(sigin.as_bytes()).unwrap();
    nym_request.set_multi_signature(&trustee.did, &sig)?;
    Ok(())
}

pub async fn register_nym(
    trustee: &DidInfo,
    nym_did: &DidInfo,
    role: &UpdateRole,
    pool: &SharedPool,
) -> VdrResult<String> {
    // Create a NYM transaction request
    let request_builder = RequestBuilder::new(ProtocolVersion::from_id(2).unwrap());
    let mut nym_request = request_builder
        .build_nym_request(
            &trustee.did,
            &nym_did.did,
            Some(nym_did.verkey.clone()),
            None,
            Some(*role),
            None,
            None,
        )
        .unwrap();

    // Sign the NYM transaction request with the helper fn
    sign_nym_request(&mut nym_request, trustee)?;

    // Submit the signed request to the ledger
    if cfg!(debug_assertions) {
        println!(
            "Submitting NYM request: {:?}",
            nym_request.req_json.to_string()
        );
    }
    let (request_result, _) = perform_ledger_request(pool, &nym_request, None).await?;

    match request_result {
        RequestResult::Reply(message) => {
            if cfg!(debug_assertions) {
                println!("Reply: {:?}", message);
            }
            Ok(message)
        }
        RequestResult::Failed(error) => {
            if cfg!(debug_assertions) {
                println!("Error: {:?}", error);
            }
            Err(error)
        }
    }
}

// pub async fn write_signed_transaction_to_ledger(
//     pool: &SharedPool,
//     signed_txn: &PreparedRequest,
// ) -> VdrResult<()> {
//     let (request_result, _) = perform_ledger_request(pool, signed_txn, None).await?;
//
//     match request_result {
//         RequestResult::Reply(message) => {
//             println!("Transaction successfully written to ledger: {:?}", message);
//             Ok(())
//         }
//         RequestResult::Failed(error) => {
//             println!("Failed to write transaction to ledger: {:?}", error);
//             Err(error)
//         }
//     }
// }

// -- Region Test Section --
#[cfg(test)]
mod tests {
    use super::*;
    use indy_data_types::keys::KeyType;
    use serde_json::json;

    #[test]
    fn test_create_did_sov() {
        let seed = "000000000000000000000000Trustee1".to_string();
        let version = 1;
        let result = create_did(seed, version);

        assert!(result.is_ok());

        let did_info = result.unwrap();
        assert_eq!(did_info.did.to_string(), "V4SGRU86Z58d6TV7PBUe6f");
        assert_eq!(
            did_info.verkey,
            "GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL"
        );
        assert!(matches!(did_info.privatekey.alg, KeyType::ED25519));
    }

    #[test]
    fn test_create_did_indy() {
        let seed = "000000000000000000000000Trustee1".to_string();
        let version = 2;
        let result = create_did(seed, version);

        assert!(result.is_ok());

        let did_info = result.unwrap();
        assert_eq!(did_info.did.to_string(), "GAAguaTbEHjvxL6i64YmAo");
        assert_eq!(
            did_info.verkey,
            "GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL"
        );
        assert!(matches!(did_info.privatekey.alg, KeyType::ED25519));
    }

    #[test]
    fn test_sign_transaction() {
        // Create mock DidInfo
        let did_info = create_did("000000000000000000000000Trustee1".to_string(), 2);
        assert!(did_info.is_ok());
        let did_info = did_info.unwrap();

        // Create mock transaction
        let txn = json!({
            "identifier": "Bhhsxc585EVgbbmosZr65J",
            "operation": {
                "dest": "VsKV7grR1BUE29mG2Fm2kX",
                "role": "101",
                "type": "1",
                "verkey": "GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa"
            },
            "protocolVersion": 2,
            "reqId": 1689080604840377700i64
        });
        // Call sign_transaction
        let result = sign_transaction(did_info, serde_json::to_string(&txn).unwrap());
        // Assert function returns Ok
        assert!(result.is_ok());

        // Assert the result matches the expected signed transaction
        // Note: The expected signed transaction is a placeholder. Replace it with the actual expected signed transaction.
        let expected_signed_transaction = json!({
            "identifier": "Bhhsxc585EVgbbmosZr65J",
            "operation": {
                "dest": "VsKV7grR1BUE29mG2Fm2kX",
                "role": "101",
                "type": "1",
                "verkey": "GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa"
            },
            "protocolVersion": 2,
            "reqId": 1689080604840377700i64,
            "signatures": {
                "GAAguaTbEHjvxL6i64YmAo": "5xtuhdLH1iZvNVeeAr5yuQD9RBsZ17zW5iwTqyxrWk89LaYpkLRGg3juuyiJNgwnSHMNq7nfXPx8AMvhfvHnXVQX"
            }
        });
        assert_eq!(result.unwrap(), expected_signed_transaction);
    }
}

// -- End Region Test Section --
