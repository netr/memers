use ethers::{
    prelude::Lazy,
    types::{Address, Bytes},
};
use std::str::FromStr;

pub const WETH_ADDRESS: &str = "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2";
pub const DEAD_ADDRESS: &str = "0x000000000000000000000000000000000000dead";
pub const UNISWAP_V2_FACTORY_ADDRESS: &str = "0x5c69bee701ef814a2b6a3edd4b1652cb9cc5aa6f";
pub const UNISWAP_V2_ROUTER_ADDRESS: &str = "0x7a250d5630b4cf539739df2c5dacb4c659f2488d";

pub fn get_env(key: &str) -> String {
    std::env::var(key).unwrap()
}

#[derive(Debug, Clone)]
pub struct Env {
    pub https_url: String,
    pub wss_url: String,
    pub blocknative_api_key: String,
}

impl Env {
    pub fn new() -> Self {
        Env {
            https_url: get_env("HTTPS_URL"),
            wss_url: get_env("WSS_URL"),
            blocknative_api_key: get_env("BLOCKNATIVE_API_KEY"),
        }
    }
}
