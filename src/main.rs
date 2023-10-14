#![allow(dead_code)]

use std::str::FromStr;
use std::sync::Arc;

use anyhow::Result;
use ethers_core::abi::Uint;
use ethers_core::types::{Address, H256, U256, H160};
use memers::abi::uniswap_v2_factory::UniswapV2Factory;
use memers::constants::{Env, UNISWAP_V2_FACTORY_ADDRESS};
use memers::dex::uniswap;
use memers::eth;
use memers::eth::transactions::Transaction;
use memers::utils::setup_logger;
use memers::{abi::ABI, constants::UNISWAP_V2_ROUTER_ADDRESS};

use ethers_contract::{BaseContract, EthEvent, abigen, MultiAbigen};
use ethers::contract::Abigen;
use ethers_providers::{Http, Middleware, Provider, StreamExt, Ws};
use log::{debug, error, info};
#[derive(Debug, Clone, EthEvent)]
pub struct PairCreated {
    pub token0: Address,
    pub token1: Address,
    pub pair: Address,
    pub nonce: U256
}

#[tokio::main]
#[allow(unreachable_code)]
async fn main() -> Result<()> {
    setup_logger()?;
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
    });

    // let http_provider = Provider::<Http>::try_from(env.https_url.as_str())
    //     .expect("could not instantiate HTTP Provider");

    // let abis = ABI::new();
    // let router = BaseContract::from(abis.uniswap_v2_router.clone());

    let block_http_provider = Provider::<Http>::try_from(env.https_url.as_str()).unwrap();
    let block_ws_provider = Provider::<Ws>::connect(env.wss_url.as_str()).await?;
    let abis = ABI::new();
    let client = Arc::new(Provider::<Http>::try_from(env.https_url.as_str()).unwrap());
    let factory = UniswapV2Factory::new(H160::from_str(UNISWAP_V2_FACTORY_ADDRESS).unwrap(), client);
    // abigen!(uniswap_factory, abis.uniswap_v2_factory.clone());

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
                                        if topic
                                            .eq(&H256::from_str(uniswap::PAIR_CREATED_TOPIC)
                                                .unwrap())
                                        {
                                            let event = factory.clone().decode_event::<PairCreated>("PairCreated", log.topics.clone(), tx.input.clone()).unwrap();                                                    

                                            info!(
                                                "[[** PAIR CREATED **]] from: {} / to: {} / address: {:?} / topics: {:?} / data: {:?}",
                                                tx.from, tx.to, log.address, log.topics, log.data
                                            ); 
                                            info!("[[** PAIR CREATED **]] event: {:?}", event)
                                        }
                                    } 
                                });
                            }
                            Ok(None) => error!("Could not get receipt for tx {}", tx.hash),
                            Err(e) => error!("Could not get receipt for tx {}: {}", tx.hash, e),
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
