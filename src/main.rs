#![allow(dead_code, unused_imports)]

mod blocknative;
mod constants;
mod utils;

use crate::blocknative::GasPrediction;
use crate::constants::Env;
use crate::utils::{convert_utc_to_local_str, setup_logger, Gas};
use anyhow::{anyhow, Error, Result};
use chrono::prelude::DateTime;
use chrono::Utc;
use ethers::prelude::H256;
use ethers_core::types::{Block, H160};
use ethers_core::utils::{format_ether, format_units};
use ethers_providers::{Http, Middleware, Provider, StreamExt, Ws};
use log::{debug, info};
use std::convert::TryFrom;
use std::ops::Add;
use std::str::FromStr;
use std::time::{Duration, UNIX_EPOCH};

#[tokio::main]
#[allow(unreachable_code)]
async fn main() -> Result<()> {
    setup_logger()?;
    dotenvy::dotenv().ok();
    debug!("Starting up memers");

    let env = Env::new();

    let http_provider = Provider::<Http>::try_from(env.https_url.as_str())
        .expect("could not instantiate HTTP Provider");
    let ws_provider = Provider::<Ws>::connect(env.wss_url.as_str()).await?;

    let response = blocknative::get_block_prices(env.blocknative_api_key.as_str()).await?;
    let prediction = GasPrediction::from(response);

    info!("Base Fee: {}", prediction.base_fee);
    info!("Gas Price: {:?}", prediction.gas_price);
    info!("Max Prio Fee: {:?}", prediction.max_prio_fee);
    info!("Max fee: {:?}", prediction.max_fee);
    info!("Next Block Base Fee: {}", prediction.next_base_fee);
    return Ok(());

    let mut stream = ws_provider.subscribe_blocks().await?;
    while let Some(block) = stream.next().await {
        let start_time = std::time::Instant::now();
        let total_tx =
            get_transactions_from_block(http_provider.clone(), block.number.unwrap()).await;

        info!(
            "Block Number: {} - Gas Used: {} - Gas Limit: {} - Timestamp: {} - Txs: {} -  Time Taken: {:?}",
            block.number.unwrap(),
            block.gas_used,
            block.gas_limit,
            convert_utc_to_local_str(block.timestamp.as_u64()),
            total_tx,
            start_time.elapsed()
        );
    }

    Ok(())
}

async fn get_transactions_from_block(
    provider: Provider<Http>,
    block_number: ethers_core::types::U64,
) -> usize {
    let mut count = 0;
    match provider.get_block_with_txs(block_number).await {
        Ok(block) => match block {
            Some(block) => {
                count += block.transactions.len();
                for tx in block.transactions {
                    let gas_price = tx.gas_price.unwrap_or(ethers_core::types::U256::default());
                    let max_fee = tx
                        .max_fee_per_gas
                        .unwrap_or(ethers_core::types::U256::default());
                    let max_priority_fee = tx
                        .max_priority_fee_per_gas
                        .unwrap_or(ethers_core::types::U256::default());

                    debug!(
                        "Tx Hash: {} - From: {} - To: {} - Value: {} - Gas Price: {} - Gas Max Fee: {} - Gas Prio Fee: {}",
                        tx.hash,
                        tx.from,
                        tx.to.unwrap_or(H160::default()),
                        format_ether(tx.value),
                        format_units(gas_price, "gwei").unwrap(),
                        format_units(max_fee, "gwei").unwrap(),
                        format_units(max_priority_fee, "gwei").unwrap(),
                    );
                }
            }
            None => todo!(),
        },
        Err(_) => todo!(),
    }

    count
}
