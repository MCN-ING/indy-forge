use crate::app::{NymInfo, TransactionOptions};
use crate::helpers::genesis::GenesisSource;
use crate::helpers::wallet::IndyWallet;
use indy_data_types::anoncreds::schema::Schema;
use indy_data_types::did::DidValue;
use indy_vdr::common::error::{VdrError, VdrErrorKind, VdrResult};
use indy_vdr::config::PoolConfig;
use indy_vdr::ledger::constants::UpdateRole;
use indy_vdr::pool::helpers::perform_ledger_request;
use indy_vdr::pool::{LocalPool, Pool, PoolBuilder, PreparedRequest, RequestResult};

#[derive(Clone)]
pub struct IndyLedger {
    pub pool: LocalPool,
}

impl IndyLedger {
    pub async fn new(genesis_source: GenesisSource) -> VdrResult<Self> {
        let pool_transactions = genesis_source.load_transactions().await.map_err(|e| {
            VdrError::new(
                VdrErrorKind::Input,
                Some(format!("Failed to load genesis transactions: {}", e)),
                None,
            )
        })?;

        let pool = PoolBuilder::new(PoolConfig::default(), pool_transactions)
            .into_local()
            .map_err(|e| {
                VdrError::new(
                    VdrErrorKind::Config,
                    Some("Failed to create pool from transactions".to_string()),
                    Some(Box::new(e)),
                )
            })?;

        Ok(IndyLedger { pool })
    }

    pub async fn check_connection(&self) -> VdrResult<bool> {
        // Get config transaction which should always be available (#1)
        let request = self
            .pool
            .get_request_builder()
            .build_get_txn_request(None, 1, 1)?; // ledger_type = 1 (DOMAIN), seq_no = 1

        match perform_ledger_request(&self.pool, &request, None).await {
            Ok((RequestResult::Reply(_), _)) => Ok(true),
            Ok((RequestResult::Failed(err), _)) => {
                log::debug!("Connection check failed: {}", err);
                Ok(false)
            }
            Err(e) => {
                log::error!("Connection check error: {}", e);
                Err(e)
            }
        }
    }

    // pub async fn publish_cred_def(
    //     &self,
    //     wallet: &IndyWallet,
    //     submitter_did: &str,
    //     cred_def: &CredentialDefinition,
    // ) -> VdrResult<String> {
    //     // hack to clone cred def
    //     let cred_def_json = json!(cred_def).to_string();
    //     let cred_def = serde_json::from_str(&cred_def_json).unwrap();
    //
    //     let mut request = self
    //         .pool
    //         .get_request_builder()
    //         .build_cred_def_request(&DidValue(submitter_did.to_string()), cred_def)
    //         .unwrap();
    //
    //     self._sign_and_submit_request(wallet, &mut request).await
    // }
    pub async fn publish_schema(
        &self,
        wallet: &IndyWallet,
        submitter_did: &str,
        schema: &Schema,
        options: &TransactionOptions,
    ) -> VdrResult<String> {
        let mut request = self
            .pool
            .get_request_builder()
            .build_schema_request(&DidValue(submitter_did.to_string()), schema.clone())?;

        let result = if options.sign {
            let sig_bytes = request.get_signature_input()?;
            let signature = wallet.sign(sig_bytes.as_bytes()).await;
            request.set_signature(&signature)?;
            serde_json::to_string_pretty(&request.req_json).map_err(|e| {
                VdrError::new(
                    VdrErrorKind::Input,
                    Some(format!(
                        "Failed to serialize signed schema transaction: {}",
                        e
                    )),
                    None,
                )
            })?
        } else {
            serde_json::to_string_pretty(&request.req_json).map_err(|e| {
                VdrError::new(
                    VdrErrorKind::Input,
                    Some(format!(
                        "Failed to serialize unsigned schema transaction: {}",
                        e
                    )),
                    None,
                )
            })?
        };

        if options.send {
            self._submit_request(&request).await
        } else {
            Ok(result)
        }
    }

    pub async fn publish_nym(
        &self,
        wallet: &IndyWallet,
        submitter_did: &str,
        nym_info: &mut NymInfo,
        role: UpdateRole,
        options: &TransactionOptions,
    ) -> VdrResult<String> {
        let alias = nym_info.alias.clone().filter(|a| !a.trim().is_empty());
        let mut request = self.pool.get_request_builder().build_nym_request(
            &DidValue(submitter_did.to_string()),
            &DidValue(nym_info.did.to_string()),
            Some(nym_info.verkey.to_string()),
            alias,
            Some(role),
            None,
            None,
        )?;

        let result = if options.sign {
            let sig_bytes = request.get_signature_input()?;
            let signature = wallet.sign(sig_bytes.as_bytes()).await;
            request.set_signature(&signature)?;
            serde_json::to_string_pretty(&request.req_json).map_err(|e| {
                VdrError::new(
                    VdrErrorKind::Input, // Using Input for serialization errors
                    Some(format!("Failed to serialize signed transaction: {}", e)),
                    None,
                )
            })?
        } else {
            serde_json::to_string_pretty(&request.req_json).map_err(|e| {
                VdrError::new(
                    VdrErrorKind::Input, // Using Input for serialization errors
                    Some(format!("Failed to serialize unsigned transaction: {}", e)),
                    None,
                )
            })?
        };

        if options.send {
            self._submit_request(&request).await
        } else {
            Ok(result)
        }
    }

    // function to only send a transaction that is already signed
    pub async fn prepare_transaction(
        &self,
        wallet: &IndyWallet,
        signed_txn: &mut String,
        options: &TransactionOptions,
    ) -> VdrResult<String> {
        let mut req = PreparedRequest::from_request_json(signed_txn)?;

        println!("Request: {:?}", req.req_json.to_string());
        // Get the request JSON to check existing signatures
        let req_json = req.req_json.as_object().ok_or_else(|| {
            VdrError::new(
                VdrErrorKind::Input,
                Some("Request JSON is not an object".to_string()),
                None,
            )
        })?;

        if options.sign {
            if req_json.contains_key("signatures") {
                return Err(VdrError::new(
                    VdrErrorKind::Input,
                    Some("Transaction already has signatures".to_string()),
                    None,
                ));
            }

            if req_json.contains_key("signature") {
                return Err(VdrError::new(
                    VdrErrorKind::Input,
                    Some("Transaction uses legacy single signature format. Please use multi-signature format.".to_string()),
                    None,
                ));
            }

            match req.get_signature_input() {
                Ok(sig_bytes) => {
                    let signature = wallet.sign(sig_bytes.as_bytes()).await;
                    req.set_multi_signature(&DidValue(wallet.did.clone()), &signature)
                        .map_err(|e| {
                            VdrError::new(
                                VdrErrorKind::Input,
                                Some("Failed to add multi-signature to request".to_string()),
                                Some(Box::new(e)),
                            )
                        })?;
                }
                Err(e) => {
                    return Err(VdrError::new(
                        VdrErrorKind::Input,
                        Some("Failed to get signature input from request".to_string()),
                        Some(Box::new(e)),
                    ));
                }
            }
        }

        if !options.send {
            return serde_json::to_string_pretty(&req.req_json).map_err(|e| {
                VdrError::new(
                    VdrErrorKind::Input,
                    Some(format!("Failed to serialize transaction: {}", e)),
                    None,
                )
            });
        }

        self._submit_request(&req).await
    }

    async fn _sign_and_submit_request(
        &self,
        wallet: &IndyWallet,
        request: &mut PreparedRequest,
    ) -> VdrResult<String> {
        let sig_bytes = request.get_signature_input()?;
        let signature = wallet.sign(sig_bytes.as_bytes()).await;
        request.set_signature(&signature)?;
        self._submit_request(request).await
    }

    async fn _submit_request(&self, request: &PreparedRequest) -> VdrResult<String> {
        let (request_result, _) = perform_ledger_request(&self.pool, request, None)
            .await
            .unwrap();
        // std::thread::sleep(Duration::from_millis(500));
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
}
