#![allow(dead_code)]

use std::str::FromStr;
use std::sync::Arc;

use anyhow::Result;
use ethers_core::types::{Address, H256, U256, H160};
use memers::abi::erc20::{ERC20, OwnershipTransferredFilter};
use memers::abi::uniswap_v2_factory::{UniswapV2Factory, PairCreatedFilter};
use memers::constants::Env;
use memers::dex::uniswap;
use memers::eth::constants::{UNISWAP_V2_ROUTER_ADDRESS, UNISWAP_V2_FACTORY_ADDRESS};
use memers::{eth, utils};
use memers::eth::transactions::Transaction;
use memers::utils::{setup_logger, DistinctStore};
use memers::abi::ABI;

use ethers_contract::{BaseContract, EthEvent};
use ethers_providers::{Http, Middleware, Provider, StreamExt, Ws};
use log::{debug, error, info, warn};
#[derive(Debug, Clone, EthEvent)]
pub struct PairCreated {
    pub token0: Address,
    pub token1: Address,
    pub pair: Address,
    pub nonce: U256
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
                    None => warn!("{}", TxError::new("get_tx".to_string(), "Transaction not found".to_string(), tx_hash, "".to_string())),
                },
                Err(e) => error!("{}", TxError::new("get_tx".to_string(), "Failed getting transaction from provider".to_string(), tx_hash, e.to_string())),
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
    let factory = UniswapV2Factory::new(H160::from_str(UNISWAP_V2_FACTORY_ADDRESS).unwrap(), client.clone());

    let event = factory.event::<PairCreatedFilter>();
    let _watcher = event.stream().await.unwrap();

    let pair_created_hash = H256::from_str(uniswap::PAIR_CREATED_TOPIC).unwrap();
    let _transfer_topic = H256::from_str(memers::eth::constants::TRANSFER_TOPIC).unwrap();
    let ownership_transferred_topic = H256::from_str(memers::eth::constants::OWNERSHIP_TRANSFERRED_TOPIC).unwrap();

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
                                                        "[[** PAIR CREATED **]] from: {} / to: {} / address: {} / topics: {:?} / data: {:?}",
                                                        utils::to_hex_str(tx.from.as_bytes()), 
                                                        utils::to_hex_str(tx.to.as_bytes()), 
                                                        utils::to_hex_str(log.address.as_bytes()), 
                                                        log.topics, 
                                                        log.data,
                                                    ); 
                                                    info!("[[** PAIR CREATED **]] event: {:?}", event)
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
                                                            "[[** OWNERSHIP RENOUNCED **]] from: {} / to: {} / address: {} / topics: {:?} / data: {:?}",
                                                            utils::to_hex_str(tx.from.as_bytes()), 
                                                            utils::to_hex_str(tx.to.as_bytes()), 
                                                            utils::to_hex_str(log.address.as_bytes()), 
                                                            log.topics, 
                                                            log.data,
                                                        ); 
                                                    }
                                                },
                                                Err(e) => error!("{}", TxError::new("ownership_renounced".to_string(), "Decode event".to_string(), log.transaction_hash.unwrap_or_default(), e.to_string())),
                                            }
                                        }
                                    } 
                                });
                            }
                            Ok(None) => error!("{:?}", TxError::new("block_with_txs".to_string(), "No transaction receipt found".to_string(), tx.hash, "".to_string())),
                            Err(e) => error!("{}", TxError::new("block_with_txs".to_string(), "Failed to get transaction receipt from provider".to_string(), tx.hash, e.to_string())),
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