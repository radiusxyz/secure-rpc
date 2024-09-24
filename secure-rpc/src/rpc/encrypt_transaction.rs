use std::str::FromStr;

use pvde::{
    encryption::{
        poseidon_encryption_zkp::{
            prove as prove_poseidon_encryption, PoseidonEncryptionPublicInput,
            PoseidonEncryptionSecretInput,
        },
        *,
    },
    halo2_proofs::{
        halo2curves::bn256::{Bn256, G1Affine},
        plonk::ProvingKey,
        poly::kzg::commitment::ParamsKZG,
    },
    num_bigint::{BigUint, RandomBits},
    poseidon::hash,
    time_lock_puzzle::{
        key_validation_zkp::{
            prove as prove_key_validation, KeyValidationParam, KeyValidationPublicInput,
            KeyValidationSecretInput,
        },
        setup as setup_time_lock_puzzle_param,
        sigma_protocol::{
            generate_sigma_protocol_public_input, SigmaProtocolParam, SigmaProtocolPublicInput,
        },
        TimeLockPuzzleParam,
    },
};
use rand::{thread_rng, Rng};
use sequencer::skde::{self, delay_encryption::PublicKey};
use tracing::info;

use crate::{rpc::prelude::*, state::AppState};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EncryptTransaction {
    pub raw_transaction: RawTransaction,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EncryptTransactionResponse {
    pub encrypted_transaction: EncryptedTransaction,
}

impl EncryptTransaction {
    pub const METHOD_NAME: &'static str = "encrypt_transaction";

    pub async fn handler(
        parameter: RpcParameter,
        context: Arc<AppState>,
    ) -> Result<EncryptTransactionResponse, RpcError> {
        let parameter = parameter.parse::<Self>()?;

        info!(
            "EncryptTransaction - raw_transaction: {:?}",
            parameter.raw_transaction
        );

        // Convert raw transaction to string
        let raw_transaction_string: String = match &parameter.raw_transaction {
            RawTransaction::Eth(raw_transaction) => {
                serde_json::from_str(&serde_json::to_string(&raw_transaction).unwrap())?
            }
            RawTransaction::EthBundle(raw_transaction) => {
                serde_json::from_str(&serde_json::to_string(&raw_transaction).unwrap())?
            }
        };

        let encrypted_transaction_type = context.config().encrypted_transaction_type();

        match encrypted_transaction_type {
            EncryptedTransactionType::Pvde => {
                // Generate time lock puzzle
                let time_lock_puzzle_param = setup_time_lock_puzzle_param(2048);
                let (
                    sigma_protocol_public_input,
                    key_validation_param,
                    key_validation_public_input,
                    key_validation_secret_input,
                ) = generate_time_lock_puzzle(time_lock_puzzle_param.clone())?;

                let time_lock_puzzle = TimeLockPuzzle::new(
                    time_lock_puzzle_param.t,
                    sigma_protocol_public_input.o.to_string(),
                    time_lock_puzzle_param.n.to_string(),
                );

                let encrypted_transaction;

                if context.config().is_using_zkp() {
                    let pvde_params = context.pvde_params().load().as_ref().clone().unwrap();
                    let key_validation_zkp_param =
                        pvde_params.key_validation_zkp_param().clone().unwrap();
                    let key_validation_proving_key =
                        pvde_params.key_validation_proving_key().clone().unwrap();

                    let poseidon_encryption_zkp_param =
                        pvde_params.poseidon_encryption_zkp_param().clone().unwrap();
                    let poseidon_encryption_proving_key = pvde_params
                        .poseidon_encryption_proving_key()
                        .clone()
                        .unwrap();

                    let pvde_encrypted_transaction = pvde_encrypt_tx_with_zkp(
                        &raw_transaction_string,
                        &sigma_protocol_public_input,
                        &key_validation_param,
                        &key_validation_public_input,
                        &key_validation_secret_input,
                        &key_validation_zkp_param,
                        &key_validation_proving_key,
                        &poseidon_encryption_zkp_param,
                        &poseidon_encryption_proving_key,
                        &time_lock_puzzle,
                    )?;

                    encrypted_transaction = EncryptedTransaction::Pvde(pvde_encrypted_transaction);
                } else {
                    let pvde_encrypted_transaction = pvde_encrypt_transaction(
                        &raw_transaction_string,
                        &key_validation_secret_input.k,
                        &time_lock_puzzle,
                    )
                    .map_err(|error| {
                        tracing::error!("encrypt_tx error: {:?}", error);
                        RpcError::from(error)
                    })?;

                    encrypted_transaction = EncryptedTransaction::Pvde(pvde_encrypted_transaction);
                }

                Ok(EncryptTransactionResponse {
                    encrypted_transaction,
                })
            }
            EncryptedTransactionType::Skde => {
                let key_management_system_client = context.key_management_client().clone().unwrap();

                let get_latest_encryption_key_response = key_management_system_client
                    .get_latest_encryption_key()
                    .await?;

                let encrypted_transaction = skde_encrypt_transaction(
                    &raw_transaction_string,
                    &get_latest_encryption_key_response.key_id,
                    &get_latest_encryption_key_response.encryption_key,
                )?;

                Ok(EncryptTransactionResponse {
                    encrypted_transaction: EncryptedTransaction::Skde(encrypted_transaction),
                })
            }
            _ => {
                unimplemented!()
            }
        }
    }
}

pub fn generate_time_lock_puzzle(
    time_lock_puzzle_param: TimeLockPuzzleParam,
) -> Result<
    (
        SigmaProtocolPublicInput,
        KeyValidationParam,
        KeyValidationPublicInput,
        KeyValidationSecretInput,
    ),
    Error,
> {
    let g = time_lock_puzzle_param.g.clone();
    let n = time_lock_puzzle_param.n.clone();
    let y = time_lock_puzzle_param.y.clone();
    let y_two = time_lock_puzzle_param.y_two.clone();

    let r = thread_rng().sample::<BigUint, _>(RandomBits::new(128));
    let s = thread_rng().sample::<BigUint, _>(RandomBits::new(128));

    // Generate sigma protocol public input
    let sigma_protocol_param = SigmaProtocolParam {
        n: n.clone(),
        g: g.clone(),
        y_two: y_two.clone(),
    };
    let sigma_protocol_public_input =
        generate_sigma_protocol_public_input(&sigma_protocol_param, &r, &s);

    // k = y^s mod n
    let k = y.modpow(&s, &n);
    let k_two = y_two.modpow(&s, &n);
    let k_hash_value = hash::hash(k.clone());

    // Generate key validation param & public & secret input
    let key_validation_param = KeyValidationParam { n: n.clone() };
    let key_validation_public_input = KeyValidationPublicInput {
        k_two: k_two.clone(),
        k_hash_value: k_hash_value.clone(),
    };
    let key_validation_secret_input = KeyValidationSecretInput { k: k.clone() };

    Ok((
        sigma_protocol_public_input,
        key_validation_param,
        key_validation_public_input,
        key_validation_secret_input,
    ))
}

pub fn get_open_and_encrypted_data(raw_tx: &str) -> Result<(EthOpenData, String), Error> {
    let decoded_transaction = decode_rlp_transaction(raw_tx).map_err(|error| {
        tracing::error!("decode_rlp_transaction error: {:?}", error);
        Error::DecodeFailed
    })?;
    let encrypt_data = to_encrypt_data_string(&decoded_transaction);

    Ok((EthOpenData::from(decoded_transaction), encrypt_data))
}

pub fn skde_encrypt_transaction(
    raw_tx: &str,
    key_id: &u64,
    encryption_key: &PublicKey,
) -> Result<SkdeEncryptedTransaction, Error> {
    const PRIME_P: &str = "8155133734070055735139271277173718200941522166153710213522626777763679009805792017274916613411023848268056376687809186180768200590914945958831360737612803";
    const PRIME_Q: &str = "13379153270147861840625872456862185586039997603014979833900847304743997773803109864546170215161716700184487787472783869920830925415022501258643369350348243";
    const GENERATOR: &str = "4";
    const TIME_PARAM_T: u32 = 2;
    const MAX_KEY_GENERATOR_NUMBER: u32 = 2;

    let time = 2_u32.pow(TIME_PARAM_T);
    let p = BigUint::from_str(PRIME_P).expect("Invalid PRIME_P");
    let q = BigUint::from_str(PRIME_Q).expect("Invalid PRIME_Q");
    let g = BigUint::from_str(GENERATOR).expect("Invalid GENERATOR");
    let max_key_generator_number = BigUint::from(MAX_KEY_GENERATOR_NUMBER);

    let (open_data, to_encrypt_data) = get_open_and_encrypted_data(raw_tx)?;

    let skde_params = skde::setup(time, p, q, g, max_key_generator_number);
    let encrypted_data =
        skde::delay_encryption::encrypt(&skde_params, &to_encrypt_data, &encryption_key).unwrap();
    let encrypted_data = format!("{}/{}", encrypted_data.c1, encrypted_data.c2);
    let encrypted_data = EncryptedData::from(encrypted_data);
    let transaction_data = TransactionData::Eth(EthTransactionData::new(encrypted_data, open_data));

    Ok(SkdeEncryptedTransaction::new(
        transaction_data,
        key_id.clone(),
    ))
}

pub fn pvde_encrypt_transaction(
    raw_tx: &str,
    k: &BigUint,
    time_lock_puzzle: &TimeLockPuzzle,
) -> Result<PvdeEncryptedTransaction, Error> {
    let (open_data, to_encrypt_data) = get_open_and_encrypted_data(raw_tx)?;

    let encryption_key = hash::hash(k.clone());

    let encrypted_data = poseidon_encryption::encrypt(&to_encrypt_data, &encryption_key);
    let encrypted_data = EncryptedData::from(encrypted_data);
    let transaction_data = TransactionData::Eth(EthTransactionData::new(encrypted_data, open_data));

    Ok(PvdeEncryptedTransaction::new(
        transaction_data,
        time_lock_puzzle.clone(),
        None,
    ))
}

#[allow(clippy::too_many_arguments)]
pub fn pvde_encrypt_tx_with_zkp(
    raw_tx: &str,

    sigma_protocol_public_input: &SigmaProtocolPublicInput,
    key_validation_param: &KeyValidationParam,
    key_validation_public_input: &KeyValidationPublicInput,
    key_validation_secret_input: &KeyValidationSecretInput,

    key_validation_zkp_param: &ParamsKZG<Bn256>,
    key_validation_proving_key: &ProvingKey<G1Affine>,
    poseidon_encryption_zkp_param: &ParamsKZG<Bn256>,
    poseidon_encryption_proving_key: &ProvingKey<G1Affine>,
    time_lock_puzzle: &TimeLockPuzzle,
) -> Result<PvdeEncryptedTransaction, Error> {
    let (_open_data, to_encrypt_data) = get_open_and_encrypted_data(raw_tx)?;
    let mut pvde_encrypted_transaction =
        pvde_encrypt_transaction(raw_tx, &key_validation_secret_input.k, time_lock_puzzle)?;

    // Generate key validation zkp
    let proof_of_key_validation = prove_key_validation(
        key_validation_zkp_param,
        key_validation_proving_key,
        key_validation_param,
        key_validation_public_input,
        key_validation_secret_input,
    );

    // Generate position encryption public & secret input
    let poseidon_encryption_public_input = PoseidonEncryptionPublicInput {
        encrypted_data: pvde_encrypted_transaction
            .transaction_data()
            .encrypted_data()
            .clone()
            .into_inner()
            .clone(),
        k_hash_value: key_validation_public_input.k_hash_value.clone(),
    };
    let poseidon_encryption_secret_input = PoseidonEncryptionSecretInput {
        data: to_encrypt_data.clone(),
        k: key_validation_secret_input.k.clone(),
    };
    let proof_of_poseidon_encryption = prove_poseidon_encryption(
        poseidon_encryption_zkp_param,
        poseidon_encryption_proving_key,
        &poseidon_encryption_public_input,
        &poseidon_encryption_secret_input,
    );

    let public_input = PvdePublicInput::new(
        sigma_protocol_public_input.r1.clone(),
        sigma_protocol_public_input.r2.clone(),
        sigma_protocol_public_input.z.clone(),
        sigma_protocol_public_input.o.clone(),
        key_validation_public_input.k_two.clone(),
        key_validation_public_input.k_hash_value.clone(),
    );
    let time_lock_puzzle_proof = TimeLockPuzzleProof::new(proof_of_key_validation);
    let encryption_proof = EncryptionProof::new(proof_of_poseidon_encryption);

    let pvde_zkp = PvdeZkp::new(public_input, time_lock_puzzle_proof, encryption_proof);
    pvde_encrypted_transaction.set_pvde_zkp(pvde_zkp);

    Ok(pvde_encrypted_transaction)
}
