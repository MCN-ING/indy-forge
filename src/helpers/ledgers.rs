use crate::app::NymInfo;
use crate::helpers::wallet::IndyWallet;
use indy_data_types::anoncreds::schema::Schema;
use indy_data_types::did::DidValue;
use indy_vdr::common::error::VdrResult;
use indy_vdr::config::PoolConfig;
use indy_vdr::ledger::constants::UpdateRole;
use indy_vdr::pool::helpers::perform_ledger_request;
use indy_vdr::pool::{
    LocalPool, Pool, PoolBuilder, PoolTransactions, PreparedRequest, RequestResult,
};

#[derive(Clone)]
pub struct IndyLedger {
    pub pool: LocalPool,
}

impl IndyLedger {
    pub async fn new(genesis_path: String) -> IndyLedger {
        let pool_transactions = PoolTransactions::from_json_file(genesis_path).unwrap();

        let pool = PoolBuilder::new(PoolConfig::default(), pool_transactions)
            .into_local()
            .unwrap();
        IndyLedger { pool }
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
    ) -> VdrResult<String> {
        let mut request = self
            .pool
            .get_request_builder()
            .build_schema_request(&DidValue(submitter_did.to_string()), schema.clone())
            .unwrap();

        self._sign_and_submit_request(wallet, &mut request).await
    }

    pub async fn publish_nym(
        &self,
        wallet: &IndyWallet,
        submitter_did: &str,
        nym_info: &mut NymInfo,
        // target_did: &str,
        // verkey: &str,
        role: UpdateRole,
    ) -> VdrResult<String> {
        let alias = nym_info.alias.clone().filter(|a| !a.trim().is_empty());
        let mut request = self
            .pool
            .get_request_builder()
            .build_nym_request(
                &DidValue(submitter_did.to_string()),
                &DidValue(nym_info.did.to_string()),
                Some(nym_info.verkey.to_string()),
                alias,
                Some(role),
                None,
                None,
            )
            .unwrap();

        self._sign_and_submit_request(wallet, &mut request).await
    }

    // function to only send a transaction that is already signed
    pub async fn write_signed_transaction_to_ledger(
        &self,
        wallet: &IndyWallet,
        signed_txn: &mut String,
    ) -> VdrResult<String> {
        let mut req = PreparedRequest::from_request_json(signed_txn)?;
        println!("Request: {:?}", req.req_json.to_string());
        self._sign_and_submit_request(wallet, &mut req).await
    }

    async fn _sign_and_submit_request(
        &self,
        wallet: &IndyWallet,
        request: &mut PreparedRequest,
    ) -> VdrResult<String> {
        let sig_bytes = request.get_signature_input()?;
        let signature = wallet.sign(sig_bytes.as_bytes()).await;
        request.set_signature(&signature)?;
        println!("Request2: {:?}", request.req_json.to_string());
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
