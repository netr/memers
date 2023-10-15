#![allow(dead_code)]

use std::str::FromStr;
use std::sync::Arc;

use anyhow::Result;
use crossbeam_channel::{bounded, select};
use ethers_contract::EthEvent;
use ethers_core::types::{Address, H160, H256, U256};
use ethers_providers::{Http, Middleware, Provider, Ws};
use log::{debug, error, info};
use memers::constants::Env;
use memers::dex::uniswap::{self};
use memers::utils;
use memers::utils::setup_logger;
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

    let env = Env::new();
    let uniswap_v2_pair_bytecode =
        std::fs::read_to_string("./src/abi/uniswap_v2_pair_bytecode.txt").unwrap();

    let (s, r) = bounded::<uniswap::UniswapV2RouterFuncs>(0);
    let ws_provider = Arc::new(Provider::<Ws>::connect(env.wss_url.as_str()).await.unwrap());
    let http_provider = Arc::new(
        Provider::<Http>::try_from(env.https_url.as_str())
            .expect("could not instantiate HTTP Provider"),
    );

    // run this in a tokio task
    tokio::spawn(async move {
        uniswap::pending_transaction_stream(s, ws_provider, http_provider).await;
    });

    tokio::spawn(async move {
        loop {
            select! {
                recv(r) -> msg => match msg {
                    Ok(msg) => {
                        info!("PENDING... {:?}", msg);
                    },
                    Err(e) => {
                        error!("Error receiving message: {}", e);
                        break;
                    }
                }
            }
        }
    });

    let (s, r) = bounded::<uniswap::UniswapTopic>(0);
    let ws_provider = Arc::new(Provider::<Ws>::connect(env.wss_url.as_str()).await.unwrap());
    let http_provider = Arc::new(
        Provider::<Http>::try_from(env.https_url.as_str())
            .expect("could not instantiate HTTP Provider"),
    );

    tokio::spawn(async move {
        uniswap::transactions_from_block_stream(s, ws_provider, http_provider).await;
    });

    tokio::spawn(async move {
        loop {
            select! {
                recv(r) -> msg => match msg {
                    Ok(msg) => {
                        match msg {
                            uniswap::UniswapTopic::PairCreated(tx, _log, event) => {
                                info!(
                                    "[[** PAIR CREATED **]] from: {} / token_0: {} / token_1: {} / pair: {}",
                                    utils::to_hex_str(tx.from.as_bytes()),
                                    utils::to_hex_str(event.token_0.as_bytes()),
                                    utils::to_hex_str(event.token_1.as_bytes()),
                                    utils::to_hex_str(event.pair.as_bytes()),
                                );
                            },
                            uniswap::UniswapTopic::OwnershipTransferred(tx, _log, event) => {
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
                            uniswap::UniswapTopic::TrustSwapDeposit(tx, _log, event) => {
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
                            uniswap::UniswapTopic::UncxOnDeposit(tx, _log, event) => {
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
                            uniswap::UniswapTopic::PinkLockAdded(tx, _log, event) => {
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
                            uniswap::UniswapTopic::Mint(tx, _log, event) => {
                                info!(
                                    "[[!* ADD LIQUIDITY *!]] sender: {} / pair: {} / amount_0: {} / amount_1: {}",
                                    utils::to_hex_str(event.sender.as_bytes()),
                                    utils::to_hex_str(tx.to.as_bytes()),
                                    event.amount_0,
                                    event.amount_1,
                                );
                            },
                            uniswap::UniswapTopic::Burn(tx, _log, event) => {
                                info!(
                                    "[[!* REMOVE LIQUIDITY *!]] sender: {} / pair: {} / to: {} / amount_0: {} / amount_1: {}",
                                    utils::to_hex_str(event.sender.as_bytes()),
                                    utils::to_hex_str(tx.to.as_bytes()),
                                    utils::to_hex_str(event.to.as_bytes()),
                                    event.amount_0,
                                    event.amount_1,
                                );
                            },
                            uniswap::UniswapTopic::Swap(tx, _log, event) => {
                                debug!(
                                    "[*- SWAP -*] sender: {} / to: {} / pair: {} / amount_0_in: {} / amount_1_in: {}, amount_0_out: {}, amount_1_out: {}",
                                    utils::to_hex_str(event.sender.as_bytes()),
                                    utils::to_hex_str(event.to.as_bytes()),
                                    utils::to_hex_str(tx.to.as_bytes()),
                                    event.amount_0_in,
                                    event.amount_1_in,
                                    event.amount_0_out,
                                    event.amount_1_out,
                                );
                            },
                            uniswap::UniswapTopic::Transfer(tx, _log, event) => {
                                let dead_address = H160::from_str(memers::eth::constants::DEAD_ADDRESS).unwrap();
                                if event.to.eq(&dead_address) && !event.from.is_zero() {
                                    let log_address = _log.address.clone();
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

                            }
                        }
                    },
                    Err(e) => {
                        error!("Error receiving message: {}", e);
                        break;
                    }
                }
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
