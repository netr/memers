#![allow(dead_code)]

mod abi;
mod blocknative;
mod constants;
mod dex;
mod utils;

use crate::abi::ABI;
use crate::constants::Env;
use crate::utils::setup_logger;
use anyhow::Result;

use ethers_contract::BaseContract;
use ethers_providers::{Http, Middleware, Provider, StreamExt, Ws};
use log::{debug, error};
use memers::eth;

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

    let abis = ABI::new();
    let router = BaseContract::from(abis.uniswap_v2_router.clone());

    let mut stream = ws_provider.subscribe_pending_txs().await?;
    while let Some(tx_hash) = stream.next().await {
        match http_provider.get_transaction(tx_hash).await {
            Ok(tx) => match tx {
                Some(tx) => {
                    let tx = eth::transactions::Transaction::from(tx.to_owned());
                    if !tx.is_uniswap_v2_router() {
                        continue;
                    }
                    dex::uniswap::try_into_uniswap_v2_router(router.clone(), &tx.input);
                }
                None => error!("Could not get transaction {}", tx_hash),
            },
            Err(e) => error!("[get_tx] error: {:?} - hash: {}", e, tx_hash),
        }
    }

    Ok(())
}
