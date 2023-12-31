use ethers_core::abi::Abi;
use std::fs;

pub mod erc20;
pub mod pink_lock;
pub mod trust_swap_lp_locker;
pub mod uncx_lp_locker;
pub mod uniswap_v2_factory;
pub mod uniswap_v2_pair;
pub mod uniswap_v2_router;

pub struct ABI {
    pub erc20: Abi,
    pub uniswap_v2_router: Abi,
    pub uniswap_v2_factory: Abi,
}

// TODO: Remove this implementation and use the one from ethers-rs
impl ABI {
    pub fn new() -> Self {
        let erc20_json = fs::read_to_string("src/abi/json/ERC20.json").unwrap();
        let uniswap_v2_router_json =
            fs::read_to_string("src/abi/json/UniswapV2Router.json").unwrap();
        let uniswap_v2_factory_json =
            fs::read_to_string("src/abi/json/UniswapV2Factory.json").unwrap();
        Self {
            erc20: serde_json::from_str(&erc20_json).unwrap(),
            uniswap_v2_router: serde_json::from_str(&uniswap_v2_router_json).unwrap(),
            uniswap_v2_factory: serde_json::from_str(&uniswap_v2_factory_json).unwrap(),
        }
    }
}
