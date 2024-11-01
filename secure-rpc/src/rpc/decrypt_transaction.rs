use std::str::FromStr;

use pvde::{
    encryption::{
        poseidon_encryption,
        poseidon_encryption_zkp::{
            verify as verify_poseidon_encryption, PoseidonEncryptionPublicInput,
        },
    },
    num_bigint::BigUint,
    poseidon::hash,
    time_lock_puzzle::{
        key_validation_zkp::verify as verify_key_validation,
        sigma_protocol::{verify as verify_sigma_protocol, SigmaProtocolParam},
        solve_time_lock_puzzle,
    },
};
use radius_sdk::json_rpc::server::RpcError;

use crate::{rpc::prelude::*, state::AppState};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DecryptTransaction {
    encrypted_transaction: EncryptedTransaction,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DecryptTransactionResponse {
    pub raw_transaction: RawTransaction,
}

impl DecryptTransaction {
    pub const METHOD_NAME: &'static str = "decrypt_transaction";

    pub async fn handler(
        parameter: RpcParameter,
        context: Arc<AppState>,
    ) -> Result<DecryptTransactionResponse, RpcError> {
        let parameter = parameter.parse::<Self>()?;

        let transaction_data = parameter.encrypted_transaction.transaction_data();
        let encrypted_data = transaction_data.encrypted_data();

        let decrypted_data = match &parameter.encrypted_transaction {
            EncryptedTransaction::Pvde(pvde_encrypted_transaction) => {
                match context.config().is_using_zkp() {
                    true => {
                        let pvde_params = context.pvde_params().load().as_ref().clone().unwrap();

                        let key_validation_zkp_param = pvde_params
                            .key_validation_zkp_param()
                            .as_ref()
                            .unwrap()
                            .clone();
                        let key_validation_verify_key = pvde_params
                            .key_validation_verifying_key()
                            .as_ref()
                            .unwrap()
                            .clone();

                        let poseidon_encryption_zkp_param = pvde_params
                            .poseidon_encryption_zkp_param()
                            .as_ref()
                            .unwrap()
                            .clone();

                        let poseidon_encryption_verify_key = pvde_params
                            .poseidon_encryption_verifying_key()
                            .as_ref()
                            .unwrap()
                            .clone();

                        let time_lock_puzzle_param = pvde_params
                            .time_lock_puzzle_param()
                            .as_ref()
                            .unwrap()
                            .clone();

                        let pvde_zkp = pvde_encrypted_transaction.pvde_zkp().unwrap();

                        let sigma_protocol_public_input =
                            pvde_zkp.public_input().to_sigma_protocol_public_input();

                        let sigma_protocol_param = SigmaProtocolParam {
                            n: time_lock_puzzle_param.n.clone(),
                            g: time_lock_puzzle_param.g.clone(),
                            y_two: time_lock_puzzle_param.y_two.clone(),
                        };
                        let is_valid = verify_sigma_protocol(
                            &sigma_protocol_public_input,
                            &sigma_protocol_param,
                        );

                        if !is_valid {
                            return Err(RpcError::from(Error::PvdeZkpInvalid));
                        }
                        // log::info!("Done verify_sigma_protocol: {:?}", is_valid);

                        let key_validation_public_input =
                            pvde_zkp.public_input().to_key_validation_public_input();
                        // let key_validation_public_input = KeyValidationPublicInput {
                        //     k_two: pvde_zkp.public_input.k_two.clone(),
                        //     k_hash_value: pvde_zkp.public_input.k_hash_value.clone(),
                        // };
                        let is_valid = verify_key_validation(
                            &key_validation_zkp_param,
                            &key_validation_verify_key,
                            &key_validation_public_input,
                            &pvde_zkp.time_lock_puzzle_proof().clone().into_inner(),
                        );

                        if !is_valid {
                            return Err(RpcError::from(Error::PvdeZkpInvalid));
                        }
                        // log::info!("Done verify_key_validation: {:?}", is_valid);

                        let poseidon_encryption_public_input = PoseidonEncryptionPublicInput {
                            encrypted_data: encrypted_data.clone().into_inner(),
                            k_hash_value: pvde_zkp.public_input().k_hash_value().clone(),
                        };
                        let is_valid = verify_poseidon_encryption(
                            &poseidon_encryption_zkp_param,
                            &poseidon_encryption_verify_key,
                            &poseidon_encryption_public_input,
                            &pvde_zkp.encryption_proof().clone().into_inner(),
                        );

                        if !is_valid {
                            return Err(RpcError::from(Error::PvdeZkpInvalid));
                        }
                        // log::info!("Done verify_poseidon_encryption: {:?}", is_valid);
                    }
                    false => {}
                }

                let time_lock_puzzle = pvde_encrypted_transaction.time_lock_puzzle();

                let o = BigUint::from_str(time_lock_puzzle.o()).unwrap();
                let t = time_lock_puzzle.t();
                let n = BigUint::from_str(time_lock_puzzle.n()).unwrap();
                let solved_k = solve_time_lock_puzzle(o, t, n);
                let solved_k_hash_value = hash::hash(solved_k.clone());

                let decrypted_data = poseidon_encryption::decrypt(
                    encrypted_data.clone().into_inner().as_str(),
                    &solved_k_hash_value,
                );

                decrypted_data
            }
            EncryptedTransaction::Skde(skde_encrypted_transaction) => {
                let distributed_key_generation_client =
                    context.distributed_key_generation_client().clone().unwrap();

                let get_decryption_key_response = distributed_key_generation_client
                    .get_decryption_key(skde_encrypted_transaction.key_id())
                    .await?;

                let skde_params = context.skde_params();

                let encrypted_data = transaction_data.encrypted_data().clone().into_inner();

                tracing::info!("Decrypt SKDE encrypted data");

                skde::delay_encryption::decrypt(
                    skde_params,
                    &encrypted_data,
                    &get_decryption_key_response.decryption_key,
                )
                .map_err(Error::DecryptionError)?
            }
        };

        match transaction_data {
            TransactionData::Eth(eth_transaction_data) => {
                let eth_plain_data: EthPlainData = serde_json::from_str(&decrypted_data).unwrap();

                let rollup_transaction = eth_transaction_data
                    .open_data()
                    .convert_to_rollup_transaction(&eth_plain_data);

                let eth_raw_transaction = EthRawTransaction::from(to_raw_tx(rollup_transaction));
                let raw_transaction = RawTransaction::from(eth_raw_transaction);

                Ok(DecryptTransactionResponse { raw_transaction })
            }
            _ => {
                unimplemented!()
            }
        }
    }
}
