use std::str::FromStr;

use pvde::{
    encryption::poseidon_encryption, num_bigint::BigUint, poseidon::hash,
    time_lock_puzzle::solve_time_lock_puzzle,
};

use crate::{rpc::prelude::*, state::AppState};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DecryptTransaction {
    rollup_id: u32,
    encrypted_transaction: EncryptedTransaction,
    time_lock_puzzle: TimeLockPuzzle,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DecryptTransactionResponse {
    pub rollup_id: u32,
    pub raw_transaction: RawTransaction,
}

impl DecryptTransaction {
    pub const METHOD_NAME: &'static str = stringify!(DecryptTransaction);

    pub async fn handler(
        parameter: RpcParameter,
        _context: Arc<AppState>,
    ) -> Result<DecryptTransactionResponse, RpcError> {
        let parameter = parameter.parse::<Self>()?;

        let time_lock_puzzle = parameter.time_lock_puzzle.clone();
        let encrypted_data = parameter.encrypted_transaction.encrypted_data().clone();
        let open_data = parameter.encrypted_transaction.open_data().clone();

        let o = BigUint::from_str(time_lock_puzzle.o()).unwrap();
        let t = time_lock_puzzle.t();
        let n = BigUint::from_str(time_lock_puzzle.n()).unwrap();
        let solved_k = solve_time_lock_puzzle(o, t, n);
        let solved_k_hash_value = hash::hash(solved_k.clone());

        let decrypted_data = poseidon_encryption::decrypt(
            encrypted_data.into_inner().as_str(),
            &solved_k_hash_value,
        );

        // TODO(jaemin): generalize
        let eth_encrypt_data: EthEncryptData = serde_json::from_str(&decrypted_data).unwrap();
        let ressembled_raw_transaction = open_data.to_raw_transaction(&eth_encrypt_data);
        let eth_raw_transaction = EthRawTransaction::from(to_raw_tx(ressembled_raw_transaction));
        let raw_transaction = RawTransaction::from(eth_raw_transaction);

        Ok(DecryptTransactionResponse {
            rollup_id: parameter.rollup_id,
            raw_transaction,
        })
    }
}
