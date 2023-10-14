#![allow(dead_code)]

use anyhow::Result;
use memers::constants::Env;
use memers::dex::uniswap;
use memers::eth::transactions::Transaction;
use memers::utils::setup_logger;
use memers::{abi::ABI, constants::UNISWAP_V2_ROUTER_ADDRESS};

use ethers_contract::BaseContract;
use ethers_providers::{Http, Middleware, Provider, StreamExt, Ws};
use log::{debug, error};

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
                    let tx = Transaction::from(tx.to_owned());
                    if !tx.is_to_address(UNISWAP_V2_ROUTER_ADDRESS) {
                        continue;
                    }
                    match uniswap::try_into_uniswap_v2_router(router.clone(), &tx.input) {
                        Some(_) => {}
                        None => {}
                    }
                }
                None => error!("Could not get transaction {}", tx_hash),
            },
            Err(e) => error!("[get_tx] error: {:?} - hash: {}", e, tx_hash),
        }
    }

    Ok(())
}
