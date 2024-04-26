use crate::helpers::wallet::IndyWallet;
use indy_data_types::did::DidValue;
use indy_vdr::common::error::VdrResult;
use indy_vdr::config::PoolConfig;
use indy_vdr::ledger::constants::UpdateRole;
use indy_vdr::pool::helpers::perform_ledger_request;
use indy_vdr::pool::{
    LocalPool, Pool, PoolBuilder, PoolTransactions, PreparedRequest, RequestResult,
};

pub enum Ledgers {
    Indy,
    Besu,
}

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

    pub async fn publish_nym(
        &self,
        wallet: &IndyWallet,
        submitter_did: &str,
        target_did: &str,
        verkey: &str,
        role: UpdateRole,
    ) -> VdrResult<String> {
        let mut request = self
            .pool
            .get_request_builder()
            .build_nym_request(
                &DidValue(submitter_did.to_string()),
                &DidValue(target_did.to_string()),
                Some(verkey.to_string()),
                None,
                Some(role),
                None,
                None,
            )
            .unwrap();

        self._sign_and_submit_request(wallet, &mut request).await
    }
    async fn _sign_and_submit_request(
        &self,
        wallet: &IndyWallet,
        request: &mut PreparedRequest,
    ) -> VdrResult<String> {
        let sig_bytes = request.get_signature_input().unwrap();
        let signature = wallet.sign(sig_bytes.as_bytes()).await;
        request.set_signature(&signature).unwrap();
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
