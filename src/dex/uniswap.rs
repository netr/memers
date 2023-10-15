use std::{str::FromStr, sync::Arc};

use anyhow::{anyhow, Result};
use crossbeam_channel::Sender;
use ethers_contract::BaseContract;
use ethers_core::{
    abi::FixedBytes,
    types::{Address, Bytes, Log, Uint8, H160, H256, U256},
};
use ethers_providers::{Http, Middleware, Provider, StreamExt, Ws};
use log::{debug, error, info};

use crate::{
    abi::{
        erc20::{OwnershipTransferredFilter, TransferFilter, ERC20},
        pink_lock::{LockAddedFilter, PinkLock},
        trust_swap_lp_locker::{DepositFilter, TrustSwapLpLocker},
        uncx_lp_locker::{OnDepositFilter, UncxLpLocker},
        uniswap_v2_factory::{PairCreatedFilter, UniswapV2Factory},
        uniswap_v2_pair::{BurnFilter, MintFilter, SwapFilter, UniswapV2Pair},
        ABI,
    },
    eth::{self, transactions::Transaction},
    utils::{to_hex_str, DistinctStore},
};

// Type aliases for Uniswap's `swap` return types
type SwapTokensForEth = (U256, U256, Vec<Address>, Address, U256);
type SwapEthForTokens = (U256, Vec<Address>, Address, U256);
type AddLiquidity = (Address, Address, U256, U256, U256, U256, Address, U256);
type AddLiquidityETH = (Address, U256, U256, U256, Address, U256);
type RemoveLiquidityEthWithPermit = (
    Address,
    U256,
    U256,
    U256,
    Address,
    U256,
    bool,
    Uint8,
    FixedBytes,
    FixedBytes,
);
type RemoveLiquidityWithPermit = (
    Address,
    Address,
    U256,
    U256,
    U256,
    Address,
    U256,
    bool,
    Uint8,
    FixedBytes,
    FixedBytes,
);
type RemoveLiquidity = (Address, Address, U256, U256, U256, Address, U256);
type RemoveLiquidityETH = (Address, U256, U256, U256, Address, U256);

pub const PAIR_CREATED_TOPIC: &str =
    "0x0d3648bd0f6ba80134a33ba9275ac585d9d315f0ad8355cddefde31afa28d0e9";
pub const MINT_TOPIC: &str = "0x4c209b5fc8ad50758f13e2e1088ba56a560dff690a1c6fef26394f4c03821c4f";
pub const BURN_TOPIC: &str = "0xdccd412f0b1252819cb1fd330b93224ca42612892bb3f4f789976e6d81936496";
pub const SWAP_TOPIC: &str = "0xd78ad95fa46c994b6551d0da85fc275fe613ce37657fb8d5e3d130840159d822";

pub const UNISWAP_V2_FACTORY_ADDRESS: &str = "0x5c69bee701ef814a2b6a3edd4b1652cb9cc5aa6f";
pub const UNISWAP_V2_ROUTER_ADDRESS: &str = "0x7a250d5630b4cf539739df2c5dacb4c659f2488d";

#[derive(Debug, Clone)]
pub enum UniswapV2RouterFuncs {
    SwapTokensForEth(SwapTokensForEth),
    SwapEthForTokens(SwapEthForTokens),
    AddLiquidity(AddLiquidity),
    AddLiquidityETH(AddLiquidityETH),
    RemoveLiquidityEthWithPermit(RemoveLiquidityEthWithPermit),
    RemoveLiquidityWithPermit(RemoveLiquidityWithPermit),
    RemoveLiquidity(RemoveLiquidity),
    RemoveLiquidityETH(RemoveLiquidityETH),
}

impl UniswapV2RouterFuncs {
    pub fn from_input(contract: BaseContract, tx_input: &Bytes) -> Result<UniswapV2RouterFuncs> {
        for (prefix, mapper) in FUNCTION_PREFIXES.iter() {
            let result = contract.as_ref().functions().find_map(|function| {
                if function.name.starts_with(prefix) {
                    if let Some(mapped_enum) = mapper(&contract, tx_input) {
                        return Some(mapped_enum);
                    }
                }
                None
            });
            if let Some(result) = result {
                return Ok(result);
            }
        }

        Err(anyhow!("No matching function"))
    }
}

type UniswapV2RouterFuncMapper = fn(&BaseContract, &Bytes) -> Option<UniswapV2RouterFuncs>;

const FUNCTION_PREFIXES: &[(&str, UniswapV2RouterFuncMapper)] = &[
    ("swap", map_swap_function),
    ("addLiquidity", map_liquidity_function),
    ("removeLiquidity", map_remove_liquidity_function),
];

fn map_swap_function(contract: &BaseContract, tx_input: &Bytes) -> Option<UniswapV2RouterFuncs> {
    if let Ok(func) = contract.decode_input::<SwapEthForTokens, _>(tx_input) {
        return Some(UniswapV2RouterFuncs::SwapEthForTokens(func));
    }
    if let Ok(func) = contract.decode_input::<SwapTokensForEth, _>(tx_input) {
        return Some(UniswapV2RouterFuncs::SwapTokensForEth(func));
    }

    None
}

fn map_liquidity_function(
    contract: &BaseContract,
    tx_input: &Bytes,
) -> Option<UniswapV2RouterFuncs> {
    if let Ok(func) = contract.decode_input::<AddLiquidity, _>(tx_input) {
        return Some(UniswapV2RouterFuncs::AddLiquidity(func));
    }
    if let Ok(func) = contract.decode_input::<AddLiquidityETH, _>(tx_input) {
        return Some(UniswapV2RouterFuncs::AddLiquidityETH(func));
    }

    None
}

fn map_remove_liquidity_function(
    contract: &BaseContract,
    tx_input: &Bytes,
) -> Option<UniswapV2RouterFuncs> {
    if let Ok(func) = contract.decode_input::<RemoveLiquidityEthWithPermit, _>(tx_input) {
        return Some(UniswapV2RouterFuncs::RemoveLiquidityEthWithPermit(func));
    }
    if let Ok(func) = contract.decode_input::<RemoveLiquidityWithPermit, _>(tx_input) {
        return Some(UniswapV2RouterFuncs::RemoveLiquidityWithPermit(func));
    }
    if let Ok(func) = contract.decode_input::<RemoveLiquidity, _>(tx_input) {
        return Some(UniswapV2RouterFuncs::RemoveLiquidity(func));
    }
    if let Ok(func) = contract.decode_input::<RemoveLiquidityETH, _>(tx_input) {
        return Some(UniswapV2RouterFuncs::RemoveLiquidityETH(func));
    }

    None
}

pub fn try_into_uniswap_v2_router(
    contract: BaseContract,
    input: &Bytes,
) -> Option<UniswapV2RouterFuncs> {
    if let Ok(router_func) = UniswapV2RouterFuncs::from_input(contract, input) {
        return Some(router_func);
    }
    None
}

pub async fn transactions_from_block_stream(
    s: Sender<UniswapTopic>,
    ws_provider: Arc<Provider<Ws>>,
    http_provider: Arc<Provider<Http>>,
) {
    let mut stream = ws_provider.subscribe_blocks().await.expect("should work");

    while let Some(block) = stream.next().await {
        debug!("New block: {}", block.number.unwrap());
        match eth::transactions::get_transactions_from_block(
            http_provider.clone(),
            ethers_core::types::BlockId::Hash(block.hash.unwrap()),
        )
        .await
        {
            Ok(block_with_txs) => {
                for tx in block_with_txs.transactions {
                    match http_provider.clone().get_transaction_receipt(tx.hash).await {
                        Ok(Some(receipt)) => {
                            receipt.logs.iter().for_each(|log| {
                                if let Some(topic) = try_uniswap_topic_from_log(
                                    tx.clone(),
                                    log,
                                    http_provider.clone(),
                                ) {
                                    s.send(topic).unwrap();
                                }
                            });
                        }
                        Ok(None) => error!(
                            "{:?}",
                            TransactionError::new(
                                "block_with_txs".to_string(),
                                "No transaction receipt found".to_string(),
                                tx.hash,
                                "".to_string()
                            )
                        ),
                        Err(e) => error!(
                            "{}",
                            TransactionError::new(
                                "block_with_txs".to_string(),
                                "Failed to get transaction receipt from provider".to_string(),
                                tx.hash,
                                e.to_string()
                            )
                        ),
                    }
                }
            }
            Err(e) => error!("Could not get transactions from block: {}", e),
        }
    }
}

fn get_topic_from_log(log: &ethers_core::types::Log) -> Option<H256> {
    if log.topics.len() > 0 {
        let topic = log.topics[0];
        return Some(topic);
    }
    None
}

#[derive(Debug, Clone)]
pub enum UniswapTopic {
    PairCreated(Transaction, Log, PairCreatedFilter),
    Transfer(Transaction, Log, TransferFilter),
    OwnershipTransferred(Transaction, Log, OwnershipTransferredFilter),
    TrustSwapDeposit(Transaction, Log, DepositFilter),
    UncxOnDeposit(Transaction, Log, OnDepositFilter),
    PinkLockAdded(Transaction, Log, LockAddedFilter),
    Mint(Transaction, Log, MintFilter),
    Burn(Transaction, Log, BurnFilter),
    Swap(Transaction, Log, SwapFilter),
}

fn try_uniswap_topic_from_log(
    tx: Transaction,
    log: &Log,
    provider: Arc<Provider<Http>>,
) -> Option<UniswapTopic> {
    if let Some(topic) = get_topic_from_log(log) {
        return match to_hex_str(topic.as_bytes()).as_str() {
            PAIR_CREATED_TOPIC => {
                let factory = UniswapV2Factory::new(
                    H160::from_str(UNISWAP_V2_FACTORY_ADDRESS).unwrap(),
                    provider,
                );
                match factory.clone().decode_event::<PairCreatedFilter>(
                    "PairCreated",
                    log.topics.clone(),
                    log.data.clone(),
                ) {
                    Ok(event) => Some(UniswapTopic::PairCreated(
                        tx.to_owned(),
                        log.to_owned(),
                        event,
                    )),
                    Err(_) => None,
                }
            }
            eth::constants::TRANSFER_TOPIC => {
                let erc20 = ERC20::new(log.address, provider);
                match erc20.clone().decode_event::<TransferFilter>(
                    "Transfer",
                    log.topics.clone(),
                    log.data.clone(),
                ) {
                    Ok(event) => Some(UniswapTopic::Transfer(tx.to_owned(), log.to_owned(), event)),
                    Err(_) => None,
                }
            }
            eth::constants::OWNERSHIP_TRANSFERRED_TOPIC => {
                let erc20 = ERC20::new(log.address, provider);
                match erc20.clone().decode_event::<OwnershipTransferredFilter>(
                    "OwnershipTransferred",
                    log.topics.clone(),
                    log.data.clone(),
                ) {
                    Ok(event) => Some(UniswapTopic::OwnershipTransferred(
                        tx.to_owned(),
                        log.to_owned(),
                        event,
                    )),
                    Err(_) => None,
                }
            }
            eth::constants::TRUST_SWAP_DEPOSIT_TOPIC => {
                let trust_swap = TrustSwapLpLocker::new(log.address, provider);
                match trust_swap.clone().decode_event::<DepositFilter>(
                    "Deposit",
                    log.topics.clone(),
                    log.data.clone(),
                ) {
                    Ok(event) => Some(UniswapTopic::TrustSwapDeposit(
                        tx.to_owned(),
                        log.to_owned(),
                        event,
                    )),
                    Err(_) => None,
                }
            }
            eth::constants::UNCX_ON_DEPOSIT_TOPIC => {
                let uncx = UncxLpLocker::new(log.address, provider);
                match uncx.clone().decode_event::<OnDepositFilter>(
                    "OnDeposit",
                    log.topics.clone(),
                    log.data.clone(),
                ) {
                    Ok(event) => Some(UniswapTopic::UncxOnDeposit(
                        tx.to_owned(),
                        log.to_owned(),
                        event,
                    )),
                    Err(_) => None,
                }
            }
            eth::constants::PINK_LOCK_ADDED_TOPIC => {
                let pink = PinkLock::new(log.address, provider);
                match pink.clone().decode_event::<LockAddedFilter>(
                    "LockAdded",
                    log.topics.clone(),
                    log.data.clone(),
                ) {
                    Ok(event) => Some(UniswapTopic::PinkLockAdded(
                        tx.to_owned(),
                        log.to_owned(),
                        event,
                    )),
                    Err(_) => None,
                }
            }
            MINT_TOPIC => {
                let pair = UniswapV2Pair::new(log.address, provider);
                match pair.clone().decode_event::<MintFilter>(
                    "Mint",
                    log.topics.clone(),
                    log.data.clone(),
                ) {
                    Ok(event) => Some(UniswapTopic::Mint(tx.to_owned(), log.to_owned(), event)),
                    Err(_) => None,
                }
            }
            BURN_TOPIC => {
                let pair = UniswapV2Pair::new(log.address, provider);
                match pair.clone().decode_event::<BurnFilter>(
                    "Burn",
                    log.topics.clone(),
                    log.data.clone(),
                ) {
                    Ok(event) => Some(UniswapTopic::Burn(tx.to_owned(), log.to_owned(), event)),
                    Err(_) => None,
                }
            }
            SWAP_TOPIC => {
                let pair = UniswapV2Pair::new(log.address, provider);
                match pair.clone().decode_event::<SwapFilter>(
                    "Swap",
                    log.topics.clone(),
                    log.data.clone(),
                ) {
                    Ok(event) => Some(UniswapTopic::Swap(tx.to_owned(), log.to_owned(), event)),
                    Err(_) => None,
                }
            }
            _ => None,
        };
    }

    None
}

pub async fn pending_transaction_stream(
    s: Sender<UniswapV2RouterFuncs>,
    ws_provider: Arc<Provider<Ws>>,
    http_provider: Arc<Provider<Http>>,
) {
    let abis = ABI::new();
    let router = BaseContract::from(abis.uniswap_v2_router.clone());
    let tx_store = DistinctStore::new();
    let mut stream = ws_provider
        .subscribe_pending_txs()
        .await
        .expect("should work");

    while let Some(tx_hash) = stream.next().await {
        match http_provider.get_transaction(tx_hash).await {
            Ok(tx) => match tx {
                Some(tx) => {
                    let tx = Transaction::from(tx.to_owned());
                    if !tx.is_to_uniswap_v2_router() || tx_store.contains(tx.hash) {
                        continue;
                    }
                    if let Ok(_) = tx_store.add(tx.hash) {
                        match try_into_uniswap_v2_router(router.clone(), &tx.input) {
                            Some(contract) => match s.send(contract) {
                                Ok(_) => {}
                                Err(e) => error!(
                                    "{}",
                                    TransactionError::new(
                                        "send".to_string(),
                                        "Failed sending transaction to channel".to_string(),
                                        tx.hash,
                                        e.to_string()
                                    )
                                ),
                            },
                            None => {}
                        }
                    }
                }
                None => debug!(
                    "{}",
                    TransactionError::new(
                        "get_tx".to_string(),
                        "Transaction not found".to_string(),
                        tx_hash,
                        "".to_string()
                    )
                ),
            },
            Err(e) => error!(
                "{}",
                TransactionError::new(
                    "get_tx".to_string(),
                    "Failed getting transaction from provider".to_string(),
                    tx_hash,
                    e.to_string()
                )
            ),
        }
    }
}

#[derive(Debug, Clone)]
struct TransactionError {
    target: String,
    reason: String,
    hash: H256,
    error: String,
}

impl TransactionError {
    fn new(target: String, reason: String, hash: H256, error: String) -> Self {
        Self {
            target,
            reason,
            hash,
            error,
        }
    }
}

impl std::fmt::Display for TransactionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.error != "" {
            write!(
                f,
                "[{}] {} - error: {} - hash: {}",
                self.target,
                self.reason,
                self.error,
                to_hex_str(self.hash.as_bytes())
            )
        } else {
            write!(
                f,
                "[{}] {} - hash: {}",
                self.target,
                self.reason,
                to_hex_str(self.hash.as_bytes())
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::abi::ABI;

    use super::*;
    use ethers::utils::hex;

    #[test]
    fn test_try_into_uniswap_v2_router_add_liquidity() {
        let abis = ABI::new();
        let router = BaseContract::from(abis.uniswap_v2_router.clone());

        let input = Bytes::from(
            hex::decode(
                "0xe8e33700000000000000000000000000dac17f958d2ee523a2206206994597c13d831ec7000000000000000000000000f81df93ab37d5b1396139f294418b2741143b28000000000000000000000000000000000000000000000000000000000000f42400000000000000000000000000000000000000000000000000de0b6b3a764000000000000000000000000000000000000000000000000000000000000000f42400000000000000000000000000000000000000000000000000de0b6b3a764000000000000000000000000000045b5a5a7d305ade28ffbc3dffe72a2d3dde642d900000000000000000000000000000000000000000000000000000000652a4ddf",
            )
            .unwrap(),
        );
        match try_into_uniswap_v2_router(router, &input) {
            Some(UniswapV2RouterFuncs::AddLiquidity(_)) => {}
            Some(_) => panic!("wrong function match"),
            None => panic!("no function match"),
        };
    }

    #[test]
    fn test_try_into_uniswap_v2_router_swap_tokens_for() {
        let abis = ABI::new();
        let router = BaseContract::from(abis.uniswap_v2_router.clone());

        let input = Bytes::from(
            hex::decode(
                "0x791ac94700000000000000000000000000000000000000000000000000003575423f722000000000000000000000000000000000000000000000000000e2350b5a1ceb1500000000000000000000000000000000000000000000000000000000000000a000000000000000000000000044d291716765164631b5cf41647f0a6ec3602a8800000000000000000000000000000000000000000000000000000000652a4bc20000000000000000000000000000000000000000000000000000000000000002000000000000000000000000fc4b4ec763722b71eb1d729749b447a9645f5f30000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2",
            )
            .unwrap(),
        );
        match try_into_uniswap_v2_router(router, &input) {
            Some(UniswapV2RouterFuncs::SwapTokensForEth(_)) => {}
            Some(_) => panic!("wrong function match"),
            None => panic!("no function match"),
        };
    }

    #[test]
    fn test_try_into_uniswap_v2_router_swap_eth_for() {
        let abis = ABI::new();
        let router = BaseContract::from(abis.uniswap_v2_router.clone());

        let input = Bytes::from(
            hex::decode(
                "0xb6f9de950000000000000000000000000000000000000000000000000de0b6b3a764000000000000000000000000000000000000000000000000000000000000000000800000000000000000000000003e88f4e2e4f78acd018a463b69eb1ed8d011111a0000000000000000000000000000000000000000000000000000018b2d37bf480000000000000000000000000000000000000000000000000000000000000002000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc20000000000000000000000000b9ae6b1d4f0eeed904d1cef68b9bd47499f3fff",
            )
            .unwrap(),
        );
        match try_into_uniswap_v2_router(router, &input) {
            Some(UniswapV2RouterFuncs::SwapEthForTokens(_)) => {}
            Some(_) => panic!("wrong function match"),
            None => panic!("no function match"),
        };
    }

    #[test]
    fn test_try_into_uniswap_v2_router_add_liquidity_eth() {
        let abis = ABI::new();
        let router = BaseContract::from(abis.uniswap_v2_router.clone());

        let input = Bytes::from(
            hex::decode(
                "0xf305d7190000000000000000000000000cfeb7799cc72838e92c28902bac5c163ca2be9b0000000000000000000000000000000000000000204fce5e3e250261100000000000000000000000000000000000000000000000204fce5e3e25026110000000000000000000000000000000000000000000000000000001f399b1438a100000000000000000000000000000639c24bdc36a8f015d36f172ecd961ea5d1cd73100000000000000000000000000000000000000000000000000000000652a4b9f",
            )
            .unwrap(),
        );
        match try_into_uniswap_v2_router(router, &input) {
            Some(UniswapV2RouterFuncs::AddLiquidityETH(_)) => {}
            Some(_) => panic!("wrong function match"),
            None => panic!("no function match"),
        };
    }

    #[test]
    fn test_try_into_uniswap_v2_router_remove_liquidity_eth_with_permit() {
        let abis = ABI::new();
        let router = BaseContract::from(abis.uniswap_v2_router.clone());

        let input = Bytes::from(
            hex::decode(
                "0xded9382a000000000000000000000000960c4b7d00140e544cc78f3f20ac1bf519162b48000000000000000000000000000000000000000000000000918d2ce7e4af4ce500000000000000000000000000000000000000000000000034419ea35fa83d4e0000000000000000000000000000000000000000000000016ef9a53e10e2f54c000000000000000000000000a1913435e4e6d96324406b16694e259c72d5859900000000000000000000000000000000000000000000000000000000652a61770000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001ccace7af2f336001a508aedaaca8aa791a111c0c89d79b0e3a1845b50a4e48c9b43b51372492a62d8d32b6150c3ed675432e4cc6cfddc76a8ac5181c8e559b616",
            )
            .unwrap(),
        );
        match try_into_uniswap_v2_router(router, &input) {
            Some(UniswapV2RouterFuncs::RemoveLiquidityEthWithPermit(_)) => {}
            Some(_) => panic!("wrong function match"),
            None => panic!("no function match"),
        };
    }

    #[test]
    fn test_try_into_uniswap_v2_router_remove_liquidity_with_permit() {
        let abis = ABI::new();
        let router = BaseContract::from(abis.uniswap_v2_router.clone());

        let input = Bytes::from(
            hex::decode(
                "0x2195995c0000000000000000000000006a3c3db39454416fc9e08da0fe10649fdedca83d000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2000000000000000000000000000000000000000000000000003b4c438798341c000000000000000000000000000000000000000000000000006b7e60c2801256000000000000000000000000000000000000000000000000001cf0b155b310b2000000000000000000000000ef190aa4ad08dad97d9575484e37645f26c4ce1900000000000000000000000000000000000000000000000000000000652a71d30000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001b7a0c521e2d2ef3308c9a61ef55a1f93ff2b237aaa5e460132f87a6c9b00cd58818d0b575fb865bee7f7e47a1fe147f2168017345f2b7852908656b28e5941414",
            )
            .unwrap(),
        );
        match try_into_uniswap_v2_router(router, &input) {
            Some(UniswapV2RouterFuncs::RemoveLiquidityWithPermit(_)) => {}
            Some(_) => panic!("wrong function match"),
            None => panic!("no function match"),
        };
    }
}
