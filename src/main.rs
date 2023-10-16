use std::str::FromStr;
use std::sync::Arc;

use anyhow::Result;
use colored::Colorize;
use crossbeam_channel::{bounded, unbounded};
use ethers_contract::EthEvent;
use ethers_core::types::{Address, H160, U256};
use ethers_providers::{Http, Middleware, Provider, Ws};
use log::{debug, info};
use memers::abi::uniswap_v2_pair::UniswapV2Pair;
use memers::constants::Env;
use memers::dex::uniswap::{self};
use memers::eth::constants::WETH_ADDRESS;
use memers::utils;
use memers::utils::setup_logger;
#[derive(Debug, Clone, EthEvent)]
pub struct PairCreated {
    pub token0: Address,
    pub token1: Address,
    pub pair: Address,
    pub nonce: U256,
}

#[tokio::main]
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
        for msg in r.iter() {
            debug!("PENDING... {:?}", msg);
        }
    });

    let (s, r) = unbounded::<uniswap::UniswapTopic>();
    let ws_provider = Arc::new(Provider::<Ws>::connect(env.wss_url.as_str()).await.unwrap());
    let http_provider = Arc::new(
        Provider::<Http>::try_from(env.https_url.as_str())
            .expect("could not instantiate HTTP Provider"),
    );

    tokio::spawn(async move {
        uniswap::transactions_from_block_stream(s, ws_provider, http_provider).await;
    });

    let weth = H160::from_str(WETH_ADDRESS).unwrap();

    tokio::spawn(async move {
        for msg in r.iter() {
            match msg {
                uniswap::UniswapTopic::ERC20Deployed(_tx, contract) => {
                    info!(
                        "{} name: {} / symbol: {} / decimals: {} / total_supply: {} / from: {} / contract: {}",
                        "CONTRACT_DEPLOYED".blue().bold(),
                        contract.name().white(),
                        contract.symbol(),
                        contract.decimals(),
                        format_units(contract.total_supply(), contract.decimals()).white(),
                        utils::to_hex_str(contract.creator().as_bytes()),
                        utils::to_hex_str(contract.address().as_bytes()),
                    );
                }
                uniswap::UniswapTopic::PairCreated(tx, _log, event) => {
                    let erc20_addr = if event.token_0.ne(&weth) {
                        event.token_0
                    } else {
                        event.token_1
                    };

                    let http_provider =
                        Arc::new(Provider::<Http>::try_from(env.https_url.as_str()).unwrap());
                    if let Ok(contract) = memers::eth::contract::get_erc20_token_vars(
                        erc20_addr,
                        tx.from,
                        http_provider,
                    )
                    .await
                    {
                        info!(
                            "{} pair: {} / name: {} / symbol: {} / decimals: {} / total_supply: {} / from: {} / token: {}",
                            "PAIR_CREATED".cyan().bold(),
                            utils::to_hex_str(event.pair.as_bytes()),
                            contract.name().white(),
                            contract.symbol(),
                            contract.decimals(),
                            format_units(contract.total_supply(), contract.decimals()).white(),
                            utils::to_hex_str(contract.creator().as_bytes()),
                            utils::to_hex_str(contract.address().as_bytes()),
                        );
                    }
                }
                uniswap::UniswapTopic::OwnershipTransferred(tx, _log, event) => {
                    if event.new_owner.is_zero() {
                        info!(
                            "{} from: {} / to: {} / prev_owner: {} / new_owner: {}",
                            "OWNERSHIP_RENOUNCED".red().bold(),
                            utils::to_hex_str(tx.from.as_bytes()),
                            utils::to_hex_str(tx.to.as_bytes()),
                            utils::to_hex_str(event.previous_owner.as_bytes()),
                            utils::to_hex_str(event.new_owner.as_bytes()),
                        );
                    }
                }
                uniswap::UniswapTopic::TrustSwapDeposit(tx, _log, event) => {
                    info!(
                        "{} from: {} / to: {} / token: {}, withdrawal to: {} / amount: {} / unlock time: {}",
                        "TRUSTSWAP_LP_LOCK".yellow().bold(),
                        utils::to_hex_str(tx.from.as_bytes()),
                        utils::to_hex_str(tx.to.as_bytes()),
                        utils::to_hex_str(event.token_address.as_bytes()),
                        utils::to_hex_str(event.withdrawal_address.as_bytes()),
                        event.amount,
                        utc_timestamp_to_date(event.unlock_time.as_u64()),
                    );
                }
                uniswap::UniswapTopic::UncxOnDeposit(tx, _log, event) => {
                    info!(
                        "{} from: {} / to: {} / token: {} / user: {} / amount: {} / lock date: {} / unlock date: {}",
                        "UNCX_LP_LOCK".yellow().bold(),
                        utils::to_hex_str(tx.from.as_bytes()),
                        utils::to_hex_str(tx.to.as_bytes()),
                        utils::to_hex_str(event.lp_token.as_bytes()),
                        utils::to_hex_str(event.user.as_bytes()),
                        event.amount,
                        event.lock_date,
                        utc_timestamp_to_date(event.unlock_date.as_u64()),
                    );
                }
                uniswap::UniswapTopic::PinkLockAdded(tx, _log, event) => {
                    info!(
                        "{} from: {} / to: {} / token: {} / owner: {} / amount: {} / unlock date: {}",
                        "PINK_LP_LOCK".yellow().bold(),
                        utils::to_hex_str(tx.from.as_bytes()),
                        utils::to_hex_str(tx.to.as_bytes()),
                        utils::to_hex_str(event.token.as_bytes()),
                        utils::to_hex_str(event.owner.as_bytes()),
                        event.amount,
                        utc_timestamp_to_date(event.unlock_date.as_u64()),
                    );
                }
                uniswap::UniswapTopic::Mint(tx, log, event) => {
                    let http_provider =
                        Arc::new(Provider::<Http>::try_from(env.https_url.as_str()).unwrap());
                    let pair = UniswapV2Pair::new(log.address, http_provider);
                    let token0 = pair.token_0().await.unwrap_or_default();
                    let token1 = pair.token_1().await.unwrap_or_default();
                    if token0.eq(&weth) || token1.eq(&weth) {
                        info!(
                            "{} sender: {} / pair: {} / amount_0: {} / amount_1: {} / hash: {}",
                            "ADD_LIQUIDITY".white().bold(),
                            utils::to_hex_str(event.sender.as_bytes()),
                            utils::to_hex_str(log.address.as_bytes()),
                            event.amount_0,
                            event.amount_1,
                            utils::to_hex_str(tx.hash.as_bytes()),
                        );
                    }
                }
                uniswap::UniswapTopic::Burn(_tx, log, event) => {
                    let http_provider =
                        Arc::new(Provider::<Http>::try_from(env.https_url.as_str()).unwrap());
                    let pair = UniswapV2Pair::new(log.address, http_provider);
                    let token0 = pair.token_0().await.unwrap_or_default();
                    let token1 = pair.token_1().await.unwrap_or_default();
                    if token0.eq(&weth) || token1.eq(&weth) {
                        info!(
                            "{} sender: {} / pair: {} / to: {} / amount_0: {} / amount_1: {}",
                            "REMOVE_LIQUIDITY".magenta().bold(),
                            utils::to_hex_str(event.sender.as_bytes()),
                            utils::to_hex_str(log.address.as_bytes()),
                            utils::to_hex_str(event.to.as_bytes()),
                            event.amount_0,
                            event.amount_1,
                        );
                    }
                }
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
                }
                uniswap::UniswapTopic::Transfer(tx, log, event) => {
                    let dead_address =
                        H160::from_str(memers::eth::constants::DEAD_ADDRESS).unwrap();
                    if event.to.eq(&dead_address) && !event.from.is_zero() {
                        let http_provider =
                            Arc::new(Provider::<Http>::try_from(env.https_url.as_str()).unwrap());
                        if is_uniswap_pair(
                            http_provider,
                            log.address,
                            &uniswap_v2_pair_bytecode.clone(),
                        )
                        .await
                        {
                            info!(
                                "{} from: {} - lp token: {} / to: {} / value: {} / hash: {}",
                                "LP_BURNED".yellow().bold(),
                                utils::to_hex_str(tx.from.as_bytes()),
                                utils::to_hex_str(log.address.as_bytes()),
                                utils::to_hex_str(event.to.as_bytes()),
                                event.value,
                                utils::to_hex_str(tx.hash.as_bytes()),
                            );
                        }
                    }
                }
            }
        }
    });

    // wait until user exists
    let _ = tokio::signal::ctrl_c().await;

    Ok(())
}

async fn is_uniswap_pair(provider: Arc<Provider<Http>>, address: Address, bytecode: &str) -> bool {
    match provider.get_code(address, None).await {
        Ok(code) => code.to_string() == bytecode.to_string(),
        Err(_) => false,
    }
}

fn utc_timestamp_to_date(timestamp: u64) -> chrono::NaiveDateTime {
    chrono::NaiveDateTime::from_timestamp_opt(timestamp as i64, 0).unwrap()
}

fn format_units(value: U256, decimals: u8) -> String {
    let val = ethers_core::utils::format_units(value, decimals as i32).unwrap_or_default();
    let (left, right) = val.split_once('.').unwrap_or((val.as_str(), ""));

    let left = left
        .chars()
        .rev()
        .enumerate()
        .fold(String::new(), |mut acc, (i, c)| {
            if i != 0 && i % 3 == 0 {
                acc.push('_');
            }
            acc.push(c);
            acc
        })
        .chars()
        .rev()
        .collect::<String>();

    if right.chars().all(|c| c == '0') {
        return left;
    }

    format!("{}.{}", left, right)
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
        let provider = Arc::new(Provider::<Http>::try_from("http://localhost:8545").unwrap());
        let address = H160::from_str("0x8bfc25ae2ac1ee299f541fc300d8737ef419066d").unwrap();
        assert!(is_uniswap_pair(provider, address, &uniswap_v2_pair_bytecode).await);
    }

    #[test]
    fn it_should_convert_timestamp_to_date() {
        let timestamp = 1709269020;
        let date = utc_timestamp_to_date(timestamp);
        assert_eq!(
            date.format("%Y-%m-%d %H:%M:%S").to_string(),
            "2024-03-01 04:57:00"
        );
    }

    #[test]
    fn format_units_with_trailing_numbers() {
        let number = U256::from_dec_str("1395633240001230456000").unwrap();
        let text = format_units(number, 9);
        assert_eq!(text, "1_395_633_240_001.230456000");
    }

    #[test]
    fn format_units_trim_zeros_from_end() {
        let number = U256::from_dec_str("50000000000000000000000000").unwrap();
        let text = format_units(number, 18);
        assert_eq!(text, "50_000_000");
    }
}
