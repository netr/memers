#![allow(dead_code)]

use std::str::FromStr;
use std::sync::Arc;

use anyhow::Result;
use ethers_core::types::{Address, H160, H256, U256};
use memers::abi::erc20::{OwnershipTransferredFilter, TransferFilter, ERC20};
use memers::abi::pink_lock::{LockAddedFilter, PinkLock};
use memers::abi::trust_swap_lp_locker::{DepositFilter, TrustSwapLpLocker};
use memers::abi::uncx_lp_locker::{OnDepositFilter, UncxLpLocker};
use memers::abi::uniswap_v2_factory::{PairCreatedFilter, UniswapV2Factory};
use memers::abi::uniswap_v2_pair::{BurnFilter, MintFilter, SwapFilter, UniswapV2Pair};
use memers::abi::ABI;
use memers::constants::Env;
use memers::dex::uniswap::{self, UNISWAP_V2_FACTORY_ADDRESS, UNISWAP_V2_ROUTER_ADDRESS};
use memers::eth::transactions::Transaction;
use memers::utils::{setup_logger, DistinctStore};
use memers::{eth, utils};

use ethers_contract::{BaseContract, EthEvent};
use ethers_providers::{Http, Middleware, Provider, StreamExt, Ws};
use log::{debug, error, info, warn};
#[derive(Debug, Clone, EthEvent)]
pub struct PairCreated {
    pub token0: Address,
    pub token1: Address,
    pub pair: Address,
    pub nonce: U256,
}

#[derive(Debug, Clone)]
struct TxError {
    target: String,
    reason: String,
    hash: H256,
    error: String,
}

impl TxError {
    fn new(target: String, reason: String, hash: H256, error: String) -> Self {
        Self {
            target,
            reason,
            hash,
            error,
        }
    }
}

impl std::fmt::Display for TxError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.error != "" {
            write!(
                f,
                "[{}] {} - error: {} - hash: {}",
                self.target,
                self.reason,
                self.error,
                utils::to_hex_str(self.hash.as_bytes())
            )
        } else {
            write!(
                f,
                "[{}] {} - hash: {}",
                self.target,
                self.reason,
                utils::to_hex_str(self.hash.as_bytes())
            )
        }
    }
}

#[tokio::main]
#[allow(unreachable_code)]
async fn main() -> Result<()> {
    setup_logger(log::LevelFilter::Info)?;
    dotenvy::dotenv().ok();
    debug!("Starting up memers");

    // let gen = MultiAbigen::from_json_files("./src/abi").unwrap();
    // println!("{:?}", gen);
    // let gen = gen.build().unwrap();
    // gen.write_to_crate(
    //     "my-crate", "0.0.5", "./bindings", false
    // ).unwrap();ga
    // return Ok(());

    let env = Env::new();
    let uniswap_v2_pair_bytecode =
        std::fs::read_to_string("./src/abi/uniswap_v2_pair_bytecode.txt").unwrap();

    let http_provider = Provider::<Http>::try_from(env.https_url.as_str())
        .expect("could not instantiate HTTP Provider");
    let ws_provider = Provider::<Ws>::connect(env.wss_url.as_str()).await?;

    let abis = ABI::new();
    let router = BaseContract::from(abis.uniswap_v2_router.clone());
    let tx_store = DistinctStore::new();

    // run this in a tokio task
    tokio::spawn(async move {
        let mut stream = ws_provider
            .subscribe_pending_txs()
            .await
            .expect("should work");
        while let Some(tx_hash) = stream.next().await {
            match http_provider.get_transaction(tx_hash).await {
                Ok(tx) => match tx {
                    Some(tx) => {
                        let tx = Transaction::from(tx.to_owned());
                        if tx_store.contains(tx.hash) {
                            continue;
                        }
                        if !tx.is_to_address(UNISWAP_V2_ROUTER_ADDRESS) {
                            continue;
                        }
                        if let Ok(_) = tx_store.add(tx.hash) {
                            match uniswap::try_into_uniswap_v2_router(router.clone(), &tx.input) {
                                Some(_) => {}
                                None => {}
                            }
                        }
                    }
                    None => warn!(
                        "{}",
                        TxError::new(
                            "get_tx".to_string(),
                            "Transaction not found".to_string(),
                            tx_hash,
                            "".to_string()
                        )
                    ),
                },
                Err(e) => error!(
                    "{}",
                    TxError::new(
                        "get_tx".to_string(),
                        "Failed getting transaction from provider".to_string(),
                        tx_hash,
                        e.to_string()
                    )
                ),
            }
        }
    });

    // let http_provider = Provider::<Http>::try_from(env.https_url.as_str())
    //     .expect("could not instantiate HTTP Provider");

    // let abis = ABI::new();
    // let router = BaseContract::from(abis.uniswap_v2_router.clone());

    let block_http_provider = Provider::<Http>::try_from(env.https_url.as_str()).unwrap();
    let block_ws_provider = Provider::<Ws>::connect(env.wss_url.as_str()).await?;
    let _abis = ABI::new();

    let client = Arc::new(Provider::<Http>::try_from(env.https_url.as_str()).unwrap());
    let factory = UniswapV2Factory::new(
        H160::from_str(UNISWAP_V2_FACTORY_ADDRESS).unwrap(),
        client.clone(),
    );

    let event = factory.event::<PairCreatedFilter>();
    let _watcher = event.stream().await.unwrap();

    let pair_created_hash = H256::from_str(uniswap::PAIR_CREATED_TOPIC).unwrap();
    let _transfer_topic = H256::from_str(memers::eth::constants::TRANSFER_TOPIC).unwrap();
    let ownership_transferred_topic =
        H256::from_str(memers::eth::constants::OWNERSHIP_TRANSFERRED_TOPIC).unwrap();
    let trust_swap_deposit_topic =
        H256::from_str(memers::eth::constants::TRUST_SWAP_DEPOSIT_TOPIC).unwrap();
    let uncx_on_deposit_topic =
        H256::from_str(memers::eth::constants::UNCX_ON_DEPOSIT_TOPIC).unwrap();
    let pink_lock_added_topic =
        H256::from_str(memers::eth::constants::PINK_LOCK_ADDED_TOPIC).unwrap();
    let uniswap_mint_topic = H256::from_str(uniswap::MINT_TOPIC).unwrap();
    let uniswap_burn_topic = H256::from_str(uniswap::BURN_TOPIC).unwrap();
    let uniswap_swap_topic = H256::from_str(uniswap::SWAP_TOPIC).unwrap();

    // run this in a tokio task
    tokio::spawn(async move {
        let mut stream = block_ws_provider
            .subscribe_blocks()
            .await
            .expect("should work");
        while let Some(block) = stream.next().await {
            debug!("New block: {}", block.number.unwrap());
            match eth::transactions::get_transactions_from_block(
                block_http_provider.clone(),
                ethers_core::types::BlockId::Hash(block.hash.unwrap()),
            )
            .await
            {
                Ok(block_with_txs) => {
                    for tx in block_with_txs.transactions {
                        match block_http_provider
                            .clone()
                            .get_transaction_receipt(tx.hash)
                            .await
                        {
                            Ok(Some(receipt)) => {
                                receipt.logs.iter().for_each(|log| {
                                    if log.topics.len() > 0 {
                                        let topic = log.topics[0];
                                        if topic.eq(&pair_created_hash) {
                                            match factory.clone().decode_event::<PairCreatedFilter>("PairCreated", log.topics.clone(), log.data.clone()) {
                                                Ok(event) => {
                                                    info!(
                                                        "[[** PAIR CREATED **]] from: {} / token_0: {} / token_1: {} / pair: {}",
                                                        utils::to_hex_str(tx.from.as_bytes()),
                                                        utils::to_hex_str(event.token_0.as_bytes()),
                                                        utils::to_hex_str(event.token_1.as_bytes()),
                                                        utils::to_hex_str(event.pair.as_bytes()),
                                                    );
                                                },
                                                Err(e) => error!("{}", TxError::new("pair_created".to_string(), "Decode event".to_string(), log.transaction_hash.unwrap_or_default(), e.to_string())),
                                            }
                                        }

                                        if topic.eq(&ownership_transferred_topic) {
                                            let erc20 = ERC20::new(log.address, client.clone());
                                            match erc20.decode_event::<OwnershipTransferredFilter>("OwnershipTransferred", log.topics.clone(),log.data.clone()) {
                                                Ok(event) => {
                                                    if event.new_owner.is_zero() {
                                                        info!(
                                                            "[[*v OWNERSHIP RENOUNCED v*]] from: {} / to: {} / prev_owner: {} / new_owner: {}",
                                                            utils::to_hex_str(tx.from.as_bytes()),
                                                            utils::to_hex_str(tx.to.as_bytes()),
                                                            utils::to_hex_str(event.previous_owner.as_bytes()),
                                                            utils::to_hex_str(event.new_owner.as_bytes()),
                                                        );
                                                    }
                                                },
                                                Err(e) => error!("{}", TxError::new("ownership_renounced".to_string(), "Decode event".to_string(), log.transaction_hash.unwrap_or_default(), e.to_string())),
                                            }
                                        }

                                        if topic.eq(&trust_swap_deposit_topic) {
                                            let trust_swap = TrustSwapLpLocker::new(log.address, client.clone());
                                            match trust_swap.decode_event::<DepositFilter>("Deposit", log.topics.clone(),log.data.clone()) {
                                                Ok(event) => {
                                                    info!(
                                                        "[[*# TRUST SWAP LP LOCK #*]] from: {} / to: {} / token: {}, withdrawal to: {} / amount: {} / unlock time: {}",
                                                        utils::to_hex_str(tx.from.as_bytes()),
                                                        utils::to_hex_str(tx.to.as_bytes()),
                                                        utils::to_hex_str(event.token_address.as_bytes()),
                                                        utils::to_hex_str(event.withdrawal_address.as_bytes()),
                                                        event.amount,
                                                        event.unlock_time,
                                                    );
                                                },
                                                Err(e) => error!("{}", TxError::new("trust_swap_deposit".to_string(), "Decode event".to_string(), log.transaction_hash.unwrap_or_default(), e.to_string())),
                                            }
                                        }

                                        if topic.eq(&uncx_on_deposit_topic) {
                                            let uncx = UncxLpLocker::new(log.address, client.clone());
                                            match uncx.decode_event::<OnDepositFilter>("OnDeposit", log.topics.clone(),log.data.clone()) {
                                                Ok(event) => {
                                                    info!(
                                                        "[[*# UNCX LP LOCK #*]] from: {} / to: {} / token: {} / user: {} / amount: {} / lock date: {} / unlock date: {}",
                                                        utils::to_hex_str(tx.from.as_bytes()),
                                                        utils::to_hex_str(tx.to.as_bytes()),
                                                        utils::to_hex_str(event.lp_token.as_bytes()),
                                                        utils::to_hex_str(event.user.as_bytes()),
                                                        event.amount,
                                                        event.lock_date,
                                                        event.unlock_date,
                                                    );
                                                },
                                                Err(e) => error!("{}", TxError::new("uncx_on_deposit".to_string(), "Decode event".to_string(), log.transaction_hash.unwrap_or_default(), e.to_string())),
                                            }
                                        }

                                        if topic.eq(&pink_lock_added_topic) {
                                            let uncx = PinkLock::new(log.address, client.clone());
                                            match uncx.decode_event::<LockAddedFilter>("LockAdded", log.topics.clone(),log.data.clone()) {
                                                Ok(event) => {
                                                    info!(
                                                        "[[*# PINK LP LOCK #*]] from: {} / to: {} / token: {} / owner: {} / amount: {} / unlock date: {}",
                                                        utils::to_hex_str(tx.from.as_bytes()),
                                                        utils::to_hex_str(tx.to.as_bytes()),
                                                        utils::to_hex_str(event.token.as_bytes()),
                                                        utils::to_hex_str(event.owner.as_bytes()),
                                                        event.amount,
                                                        event.unlock_date,
                                                    );
                                                },
                                                Err(e) => error!("{}", TxError::new("pink_lock_added".to_string(), "Decode event".to_string(), log.transaction_hash.unwrap_or_default(), e.to_string())),
                                            }
                                        }

                                        if topic.eq(&uniswap_mint_topic) {
                                            let pair = UniswapV2Pair::new(log.address, client.clone());
                                            match pair.decode_event::<MintFilter>("Mint", log.topics.clone(),log.data.clone()) {
                                                Ok(event) => {
                                                    info!(
                                                        "[[!* ADD LIQUIDITY *!]] sender: {} / pair: {} / amount_0: {} / amount_1: {}",
                                                        utils::to_hex_str(event.sender.as_bytes()),
                                                        utils::to_hex_str(log.address.as_bytes()),
                                                        event.amount_0,
                                                        event.amount_1,
                                                    );
                                                },
                                                Err(e) => error!("{}", TxError::new("uniswap_mint".to_string(), "Decode event".to_string(), log.transaction_hash.unwrap_or_default(), e.to_string())),
                                            }
                                        }

                                        if topic.eq(&uniswap_burn_topic) {
                                            let pair = UniswapV2Pair::new(log.address, client.clone());
                                            match pair.decode_event::<BurnFilter>("Burn", log.topics.clone(),log.data.clone()) {
                                                Ok(event) => {
                                                    info!(
                                                        "[[!* REMOVE LIQUIDITY *!]] sender: {} / pair: {} / to: {} / amount_0: {} / amount_1: {}",
                                                        utils::to_hex_str(event.sender.as_bytes()),
                                                        utils::to_hex_str(log.address.as_bytes()),
                                                        utils::to_hex_str(event.to.as_bytes()),
                                                        event.amount_0,
                                                        event.amount_1,
                                                    );
                                                },
                                                Err(e) => error!("{}", TxError::new("uniswap_burn".to_string(), "Decode event".to_string(), log.transaction_hash.unwrap_or_default(), e.to_string())),
                                            }
                                        }

                                        if topic.eq(&uniswap_swap_topic) {
                                            let pair = UniswapV2Pair::new(log.address, client.clone());
                                            match pair.decode_event::<SwapFilter>("Swap", log.topics.clone(),log.data.clone()) {
                                                Ok(event) => {
                                                    debug!(
                                                        "[*- SWAP -*] sender: {} / to: {} / pair: {} / amount_0_in: {} / amount_1_in: {}, amount_0_out: {}, amount_1_out: {}",
                                                        utils::to_hex_str(event.sender.as_bytes()),
                                                        utils::to_hex_str(event.to.as_bytes()),
                                                        utils::to_hex_str(log.address.as_bytes()),
                                                        event.amount_0_in,
                                                        event.amount_1_in,
                                                        event.amount_0_out,
                                                        event.amount_1_out,
                                                    );
                                                },
                                                Err(e) => error!("{}", TxError::new("uniswap_swap".to_string(), "Decode event".to_string(), log.transaction_hash.unwrap_or_default(), e.to_string())),
                                            }
                                        }


                                        if topic.eq(&_transfer_topic) {
                                            let erc20 = ERC20::new(log.address, client.clone());
                                            match erc20.decode_event::<TransferFilter>("Transfer", log.topics.clone(),log.data.clone()) {
                                                Ok(event) => {
                                                    let dead_address = H160::from_str(memers::eth::constants::DEAD_ADDRESS).unwrap();
                                                    if event.to.eq(&dead_address) && !event.from.is_zero() {
                                                        let log_address = log.address.clone();
                                                        let bytecode_client = Arc::new(Provider::<Http>::try_from(env.https_url.as_str()).unwrap());
                                                        let bc = uniswap_v2_pair_bytecode.clone();
                                                        tokio::spawn(async move {
                                                            if is_uniswap_pair(&bytecode_client, log_address, &bc).await {
                                                                info!(
                                                                    "[[*~ LP BURNED ~*]] from: {} - lp token: {} / to: {} / value: {}",
                                                                    utils::to_hex_str(tx.from.as_bytes()),
                                                                    utils::to_hex_str(log_address.as_bytes()),
                                                                    utils::to_hex_str(event.to.as_bytes()),
                                                                    event.value,
                                                                );
                                                            }
                                                        });
                                                    }
                                                },
                                                Err(_) => {},
                                            }
                                        }
                                    }
                                });
                            }
                            Ok(None) => error!(
                                "{:?}",
                                TxError::new(
                                    "block_with_txs".to_string(),
                                    "No transaction receipt found".to_string(),
                                    tx.hash,
                                    "".to_string()
                                )
                            ),
                            Err(e) => error!(
                                "{}",
                                TxError::new(
                                    "block_with_txs".to_string(),
                                    "Failed to get transaction receipt from provider".to_string(),
                                    tx.hash,
                                    e.to_string()
                                )
                            ),
                        };
                    }
                }
                Err(e) => error!("Could not get transactions from block: {}", e),
            }
        }
    });

    // wait until user exists
    let _ = tokio::signal::ctrl_c().await;

    Ok(())
}

async fn is_uniswap_pair(provider: &Provider<Http>, address: Address, bytecode: &str) -> bool {
    match provider.get_code(address, None).await {
        Ok(code) => code.to_string() == bytecode,
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ethers::providers::{Http, Provider};

    #[tokio::test]
    #[ignore = "requires local node"]
    async fn it_should_get_byte_code() {
        let uniswap_v2_pair_bytecode =
            std::fs::read_to_string("./src/abi/uniswap_v2_pair_bytecode.txt").unwrap();
        let provider = Provider::<Http>::try_from("http://localhost:8545").unwrap();
        let address = H160::from_str("0x8bfc25ae2ac1ee299f541fc300d8737ef419066d").unwrap();
        assert!(is_uniswap_pair(&provider, address, &uniswap_v2_pair_bytecode).await);
    }
}
