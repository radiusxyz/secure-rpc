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

use crate::{
    context::{context as static_context, static_str::*},
    rpc::prelude::*,
    state::AppState,
};

pub type DecryptionKey = String;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EncryptTransaction {
    rollup_id: u32,
    raw_transaction: RawTransaction,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EncryptTransactionResponse {
    pub encrypted_transaction: EncryptedTransaction,
    pub decryption_key: DecryptionKey,
    pub time_lock_puzzle: TimeLockPuzzle,
}

impl EncryptTransaction {
    pub const METHOD_NAME: &'static str = stringify!(EncryptTransaction);

    pub async fn handler(
        parameter: RpcParameter,
        context: Arc<AppState>,
    ) -> Result<EncryptTransactionResponse, RpcError> {
        let parameter = parameter.parse::<Self>()?;

        let raw_transaction_string = match &parameter.raw_transaction {
            RawTransaction::Eth(raw_transaction) => {
                serde_json::to_string(&raw_transaction).unwrap()
            }
            RawTransaction::EthBundle(raw_transaction) => {
                serde_json::to_string(&raw_transaction).unwrap()
            }
        };
        let parsed_raw_transaction_string: String = serde_json::from_str(&raw_transaction_string)?;

        let time_lock_puzzle_param = setup_time_lock_puzzle_param(2048);

        let (
            sigma_protocol_public_input,
            key_validation_param,
            key_validation_public_input,
            key_validation_secret_input,
        ) = generate_time_lock_puzzle(time_lock_puzzle_param.clone())?;

        let _raw_data;
        let encrypted_transaction;
        let decryption_key = DecryptionKey::from(key_validation_secret_input.k.to_str_radix(10));

        if context.config().is_using_zkp() {
            let key_validation_zkp_param = static_context()
                .load(KEY_VALIDATION_ZKP_PARAM)
                .await
                .unwrap();
            let key_validation_proving_key = static_context()
                .load(KEY_VALIDATION_PROVE_KEY)
                .await
                .unwrap();

            let poseidon_encryption_zkp_param = static_context()
                .load(POSEIDON_ENCRYPTION_ZKP_PARAM)
                .await
                .unwrap();
            let poseidon_encryption_proving_key = static_context()
                .load(POSEIDON_ENCRYPTION_PROVE_KEY)
                .await
                .unwrap();

            (_raw_data, encrypted_transaction) = encrypt_tx_with_zkp(
                &parsed_raw_transaction_string,
                &sigma_protocol_public_input,
                &key_validation_param,
                &key_validation_public_input,
                &key_validation_secret_input,
                &key_validation_zkp_param,
                &key_validation_proving_key,
                &poseidon_encryption_zkp_param,
                &poseidon_encryption_proving_key,
            )?;
        } else {
            (_raw_data, encrypted_transaction) = encrypt_transaction(
                &parsed_raw_transaction_string,
                &key_validation_secret_input.k,
            )
            .map_err(|error| {
                tracing::error!("encrypt_tx error: {:?}", error);
                RpcError::from(error)
            })?;
        }

        Ok(EncryptTransactionResponse {
            encrypted_transaction,
            decryption_key,
            time_lock_puzzle: TimeLockPuzzle::new(
                time_lock_puzzle_param.t,
                sigma_protocol_public_input.o.to_string(),
                time_lock_puzzle_param.n.to_string(),
            ),
        })
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
    let decoded_transaction = decode_rlp_transaction(&raw_tx).map_err(|error| {
        tracing::error!("decode_rlp_transaction error: {:?}", error);
        Error::DecodeFailed
    })?;
    let encrypt_data = to_encrypt_data_string(&decoded_transaction);

    Ok((EthOpenData::from(decoded_transaction), encrypt_data))
}

pub fn encrypt_transaction(
    raw_tx: &str,
    k: &BigUint,
) -> Result<(String, EncryptedTransaction), Error> {
    let (open_data, to_encrypt_data) = get_open_and_encrypted_data(&raw_tx)?;

    let encryption_key = hash::hash(k.clone());

    let encrypted_data = EncryptedData::new(poseidon_encryption::encrypt(
        &to_encrypt_data,
        &encryption_key,
    ));

    Ok((
        to_encrypt_data,
        EncryptedTransaction::Eth(EthEncryptedTransaction::new(
            open_data,
            encrypted_data,
            None,
        )),
    ))
}

pub fn encrypt_tx_with_zkp(
    raw_tx: &str,

    sigma_protocol_public_input: &SigmaProtocolPublicInput,
    key_validation_param: &KeyValidationParam,
    key_validation_public_input: &KeyValidationPublicInput,
    key_validation_secret_input: &KeyValidationSecretInput,

    key_validation_zkp_param: &ParamsKZG<Bn256>,
    key_validation_proving_key: &ProvingKey<G1Affine>,
    poseidon_encryption_zkp_param: &ParamsKZG<Bn256>,
    poseidon_encryption_proving_key: &ProvingKey<G1Affine>,
) -> Result<(String, EncryptedTransaction), Error> {
    let (to_encrypt_data, mut encrypted_tx) =
        encrypt_transaction(raw_tx, &key_validation_secret_input.k)?;

    // Generate key validation zkp
    let proof_of_key_validation = prove_key_validation(
        &key_validation_zkp_param,
        &key_validation_proving_key,
        &key_validation_param,
        &key_validation_public_input,
        &key_validation_secret_input,
    );

    // Generate position encryption public & secret input
    let poseidon_encryption_public_input = PoseidonEncryptionPublicInput {
        encrypted_data: encrypted_tx.encrypted_data().clone().into_inner().clone(),
        k_hash_value: key_validation_public_input.k_hash_value.clone(),
    };
    let poseidon_encryption_secret_input = PoseidonEncryptionSecretInput {
        data: to_encrypt_data.clone(),
        k: key_validation_secret_input.k.clone(),
    };
    let proof_of_poseidon_encryption = prove_poseidon_encryption(
        &poseidon_encryption_zkp_param,
        &poseidon_encryption_proving_key,
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
    encrypted_tx.update_pvde_zkp(Some(pvde_zkp));

    Ok((to_encrypt_data, encrypted_tx))
}
