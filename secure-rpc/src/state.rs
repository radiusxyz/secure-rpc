use std::sync::Arc;

use pvde::{
    halo2_proofs::{
        halo2curves::bn256::{Bn256, G1Affine},
        plonk::{ProvingKey, VerifyingKey},
        poly::kzg::commitment::ParamsKZG,
    },
    time_lock_puzzle::TimeLockPuzzleParam,
};
use radius_sdk::context::SharedContext;

use crate::{cli::Config, client::*};

pub struct AppState {
    inner: Arc<AppStateInner>,
}

struct AppStateInner {
    config: Config,
    sequencer_rpc_client: SequencerRpcClient,
    rollup_rpc_client: RollupRpcClient,
    pvde_params: SharedContext<Option<PvdeParams>>,
    skde_params: skde::delay_encryption::SkdeParams,
    distributed_key_generation_client: Option<DistributedKeyGenerationClient>,
}

impl Clone for AppState {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl AppState {
    pub fn new(
        config: Config,
        skde_params: skde::delay_encryption::SkdeParams,
        distributed_key_generation_client: Option<DistributedKeyGenerationClient>,
    ) -> Self {
        let sequencer_rpc_client = SequencerRpcClient::new(config.sequencer_rpc_url()).unwrap();
        let rollup_rpc_client = RollupRpcClient::new(config.rollup_rpc_url()).unwrap();

        let inner = AppStateInner {
            config,
            sequencer_rpc_client,
            rollup_rpc_client,
            pvde_params: SharedContext::from(None),
            skde_params,
            distributed_key_generation_client,
        };

        Self {
            inner: Arc::new(inner),
        }
    }

    pub fn config(&self) -> &Config {
        &self.inner.config
    }

    pub fn sequencer_rpc_client(&self) -> SequencerRpcClient {
        self.inner.sequencer_rpc_client.clone()
    }

    pub fn rollup_rpc_client(&self) -> RollupRpcClient {
        self.inner.rollup_rpc_client.clone()
    }

    pub fn pvde_params(&self) -> SharedContext<Option<PvdeParams>> {
        self.inner.pvde_params.clone()
    }

    pub fn skde_params(&self) -> &skde::delay_encryption::SkdeParams {
        &self.inner.skde_params
    }

    pub fn distributed_key_generation_client(&self) -> &Option<DistributedKeyGenerationClient> {
        &self.inner.distributed_key_generation_client
    }
}

// TODO: Import from sequencer
#[derive(Clone, Debug, Default)]
pub struct PvdeParams {
    time_lock_puzzle_param: Option<TimeLockPuzzleParam>,
    key_validation_zkp_param: Option<ParamsKZG<Bn256>>,
    key_validation_proving_key: Option<ProvingKey<G1Affine>>,
    key_validation_verifying_key: Option<VerifyingKey<G1Affine>>,
    poseidon_encryption_zkp_param: Option<ParamsKZG<Bn256>>,
    poseidon_encryption_proving_key: Option<ProvingKey<G1Affine>>,
    poseidon_encryption_verifying_key: Option<VerifyingKey<G1Affine>>,
}

impl PvdeParams {
    pub fn new(
        time_lock_puzzle_param: Option<TimeLockPuzzleParam>,
        key_validation_zkp_param: Option<ParamsKZG<Bn256>>,
        key_validation_proving_key: Option<ProvingKey<G1Affine>>,
        key_validation_verifying_key: Option<VerifyingKey<G1Affine>>,
        poseidon_encryption_zkp_param: Option<ParamsKZG<Bn256>>,
        poseidon_encryption_proving_key: Option<ProvingKey<G1Affine>>,
        poseidon_encryption_verifying_key: Option<VerifyingKey<G1Affine>>,
    ) -> Self {
        Self {
            time_lock_puzzle_param,
            key_validation_zkp_param,
            key_validation_proving_key,
            key_validation_verifying_key,
            poseidon_encryption_zkp_param,
            poseidon_encryption_proving_key,
            poseidon_encryption_verifying_key,
        }
    }

    pub fn time_lock_puzzle_param(&self) -> &Option<TimeLockPuzzleParam> {
        &self.time_lock_puzzle_param
    }

    pub fn key_validation_zkp_param(&self) -> &Option<ParamsKZG<Bn256>> {
        &self.key_validation_zkp_param
    }

    pub fn key_validation_proving_key(&self) -> &Option<ProvingKey<G1Affine>> {
        &self.key_validation_proving_key
    }

    pub fn key_validation_verifying_key(&self) -> &Option<VerifyingKey<G1Affine>> {
        &self.key_validation_verifying_key
    }

    pub fn poseidon_encryption_zkp_param(&self) -> &Option<ParamsKZG<Bn256>> {
        &self.poseidon_encryption_zkp_param
    }

    pub fn poseidon_encryption_proving_key(&self) -> &Option<ProvingKey<G1Affine>> {
        &self.poseidon_encryption_proving_key
    }

    pub fn poseidon_encryption_verifying_key(&self) -> &Option<VerifyingKey<G1Affine>> {
        &self.poseidon_encryption_verifying_key
    }

    pub fn update_time_lock_puzzle_param(&mut self, time_lock_puzzle_param: TimeLockPuzzleParam) {
        self.time_lock_puzzle_param = Some(time_lock_puzzle_param);
    }
    pub fn update_key_validation_zkp_param(&mut self, key_validation_zkp_param: ParamsKZG<Bn256>) {
        self.key_validation_zkp_param = Some(key_validation_zkp_param);
    }

    pub fn update_key_validation_proving_key(
        &mut self,
        key_validation_proving_key: ProvingKey<G1Affine>,
    ) {
        self.key_validation_proving_key = Some(key_validation_proving_key);
    }

    pub fn update_key_validation_verifying_key(
        &mut self,
        key_validation_verifying_key: VerifyingKey<G1Affine>,
    ) {
        self.key_validation_verifying_key = Some(key_validation_verifying_key);
    }

    pub fn update_poseidon_encryption_zkp_param(
        &mut self,
        poseidon_encryption_zkp_param: ParamsKZG<Bn256>,
    ) {
        self.poseidon_encryption_zkp_param = Some(poseidon_encryption_zkp_param);
    }
    pub fn update_poseidon_encryption_proving_key(
        &mut self,
        poseidon_encryption_proving_key: ProvingKey<G1Affine>,
    ) {
        self.poseidon_encryption_proving_key = Some(poseidon_encryption_proving_key);
    }

    pub fn update_poseidon_encryption_verifying_key(
        &mut self,
        poseidon_encryption_verifying_key: VerifyingKey<G1Affine>,
    ) {
        self.poseidon_encryption_verifying_key = Some(poseidon_encryption_verifying_key);
    }
}
