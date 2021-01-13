//! Common primitives used within testkit.
use num::BigUint;
use std::collections::HashMap;
use web3::types::TransactionReceipt;
use zksync_config::ZkSyncConfig;
use zksync_types::TokenId;

#[derive(Debug)]
pub struct TestkitConfig {
    pub chain_id: u8,
    pub gas_price_factor: f64,
    pub web3_url: String,
    pub available_block_chunk_sizes: Vec<usize>,
}

impl TestkitConfig {
    pub fn from_env() -> Self {
        let config = ZkSyncConfig::from_env();
        TestkitConfig {
            chain_id: config.eth_client.chain_id,
            gas_price_factor: config.eth_client.gas_price_factor,
            web3_url: config.eth_client.web3_url,
            available_block_chunk_sizes: config.chain.state_keeper.block_chunk_sizes,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct ETHAccountId(pub usize);

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct ZKSyncAccountId(pub usize);

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Token(pub TokenId);

#[derive(Debug, Clone)]
pub struct BlockExecutionResult {
    pub commit_result: TransactionReceipt,
    pub verify_result: TransactionReceipt,
    pub withdrawals_result: TransactionReceipt,
    pub block_size_chunks: usize,
}

impl BlockExecutionResult {
    pub fn new(
        commit_result: TransactionReceipt,
        verify_result: TransactionReceipt,
        withdrawals_result: TransactionReceipt,
        block_size_chunks: usize,
    ) -> Self {
        Self {
            commit_result,
            verify_result,
            withdrawals_result,
            block_size_chunks,
        }
    }
}

// Struct used to keep expected balance changes after transactions execution.
#[derive(Default)]
pub struct ExpectedAccountState {
    pub eth_accounts_state: HashMap<(ETHAccountId, TokenId), BigUint>,
    pub sync_accounts_state: HashMap<(ZKSyncAccountId, TokenId), BigUint>,

    // Amount of withdraw operations performed in block.
    pub withdraw_ops: usize,
}
