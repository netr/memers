use anyhow::{anyhow, Result};
use ethers_contract::{decode_function_data, BaseContract};
use ethers_core::{
    abi::{FixedBytes, Function},
    types::{Address, Bytes, Uint8, H160, U256},
};
use log::info;

// Type aliases for Uniswap's `swap` return types
type SwapTokensForEth = (U256, U256, Vec<Address>, Address, U256);
type SwapEthForTokens = (U256, Vec<Address>, Address, U256);
type PairSwap = (U256, U256, Address, Bytes);
type AddLiquidity = (Address, Address, U256, U256, U256, U256, Address, U256);
type AddLiquidityETH = (Address, U256, U256, U256, Address, U256);
type CreatePair = (H160, H160);
type RemoveLiquidityEthWithPermit = (
    Address,
    U256,
    U256,
    U256,
    Address,
    U256,
    bool,
    Uint8,
    FixedBytes,
    FixedBytes,
);
type RemoveLiquidityWithPermit = (
    Address,
    Address,
    U256,
    U256,
    U256,
    Address,
    U256,
    bool,
    Uint8,
    FixedBytes,
    FixedBytes,
);
type RemoveLiquidity = (Address, Address, U256, U256, U256, Address, U256);
type RemoveLiquidityETH = (Address, U256, U256, U256, Address, U256);

#[derive(Debug, Clone)]
pub enum UniswapV2RouterFuncs {
    SwapTokensForEth(SwapTokensForEth),
    SwapEthForTokens(SwapEthForTokens),
    PairSwap(PairSwap),
    CreatePair(CreatePair),
    AddLiquidity(AddLiquidity),
    AddLiquidityETH(AddLiquidityETH),
    RemoveLiquidityEthWithPermit(RemoveLiquidityEthWithPermit),
    RemoveLiquidityWithPermit(RemoveLiquidityWithPermit),
    RemoveLiquidity(RemoveLiquidity),
    RemoveLiquidityETH(RemoveLiquidityETH),
}

impl UniswapV2RouterFuncs {
    pub fn from_input(contract: BaseContract, tx_input: &Bytes) -> Result<UniswapV2RouterFuncs> {
        let result = contract.as_ref().functions().find_map(|function| {
            for (prefix, mapper) in FUNCTION_PREFIXES.iter() {
                if function.name.starts_with(prefix) {
                    if let Some(mapped_enum) = mapper(function, tx_input) {
                        return Some(mapped_enum);
                    }
                }
            }
            None
        });
        result.ok_or(anyhow!("No matching function"))
    }
}

const FUNCTION_PREFIXES: &[(&str, fn(&Function, &Bytes) -> Option<UniswapV2RouterFuncs>)] = &[
    ("swap", map_swap_function),
    ("addLiquidity", map_liquidity_function),
    ("removeLiquidity", map_remove_liquidity_function),
];

fn map_swap_function(function: &Function, tx_input: &Bytes) -> Option<UniswapV2RouterFuncs> {
    if let Ok(func) = decode_function_data::<SwapEthForTokens, _>(function, tx_input, true) {
        return Some(UniswapV2RouterFuncs::SwapEthForTokens(func));
    }
    if let Ok(func) = decode_function_data::<SwapTokensForEth, _>(function, tx_input, true) {
        return Some(UniswapV2RouterFuncs::SwapTokensForEth(func));
    }

    None
}

fn map_liquidity_function(function: &Function, tx_input: &Bytes) -> Option<UniswapV2RouterFuncs> {
    if let Ok(func) = decode_function_data::<AddLiquidity, _>(function, tx_input, true) {
        return Some(UniswapV2RouterFuncs::AddLiquidity(func));
    }
    if let Ok(func) = decode_function_data::<AddLiquidityETH, _>(function, tx_input, true) {
        return Some(UniswapV2RouterFuncs::AddLiquidityETH(func));
    }

    None
}

fn map_remove_liquidity_function(
    function: &Function,
    tx_input: &Bytes,
) -> Option<UniswapV2RouterFuncs> {
    if let Ok(func) =
        decode_function_data::<RemoveLiquidityEthWithPermit, _>(function, tx_input, true)
    {
        return Some(UniswapV2RouterFuncs::RemoveLiquidityEthWithPermit(func));
    }
    if let Ok(func) = decode_function_data::<RemoveLiquidityWithPermit, _>(function, tx_input, true)
    {
        return Some(UniswapV2RouterFuncs::RemoveLiquidityWithPermit(func));
    }
    if let Ok(func) = decode_function_data::<RemoveLiquidity, _>(function, tx_input, true) {
        return Some(UniswapV2RouterFuncs::RemoveLiquidity(func));
    }
    if let Ok(func) = decode_function_data::<RemoveLiquidityETH, _>(function, tx_input, true) {
        return Some(UniswapV2RouterFuncs::RemoveLiquidityETH(func));
    }

    None
}

pub fn try_into_uniswap_v2_router(
    contract: BaseContract,
    input: &Bytes,
) -> Option<UniswapV2RouterFuncs> {
    if let Ok(router_func) = UniswapV2RouterFuncs::from_input(contract, input) {
        info!("{:?}", router_func);
        return Some(router_func);
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::abi::ABI;

    use super::*;
    use ethers::utils::hex;

    #[test]
    fn test_try_into_uniswap_v2_router_add_liquidity() {
        let abis = ABI::new();
        let router = BaseContract::from(abis.uniswap_v2_router.clone());

        let input = Bytes::from(
            hex::decode(
                "0xe8e33700000000000000000000000000dac17f958d2ee523a2206206994597c13d831ec7000000000000000000000000f81df93ab37d5b1396139f294418b2741143b28000000000000000000000000000000000000000000000000000000000000f42400000000000000000000000000000000000000000000000000de0b6b3a764000000000000000000000000000000000000000000000000000000000000000f42400000000000000000000000000000000000000000000000000de0b6b3a764000000000000000000000000000045b5a5a7d305ade28ffbc3dffe72a2d3dde642d900000000000000000000000000000000000000000000000000000000652a4ddf",
            )
            .unwrap(),
        );
        match try_into_uniswap_v2_router(router, &input) {
            Some(UniswapV2RouterFuncs::AddLiquidity(_)) => {}
            Some(_) => panic!("wrong function match"),
            None => panic!("no function match"),
        };
    }

    #[test]
    fn test_try_into_uniswap_v2_router_swap_tokens_for() {
        let abis = ABI::new();
        let router = BaseContract::from(abis.uniswap_v2_router.clone());

        let input = Bytes::from(
            hex::decode(
                "0x791ac94700000000000000000000000000000000000000000000000000003575423f722000000000000000000000000000000000000000000000000000e2350b5a1ceb1500000000000000000000000000000000000000000000000000000000000000a000000000000000000000000044d291716765164631b5cf41647f0a6ec3602a8800000000000000000000000000000000000000000000000000000000652a4bc20000000000000000000000000000000000000000000000000000000000000002000000000000000000000000fc4b4ec763722b71eb1d729749b447a9645f5f30000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2",
            )
            .unwrap(),
        );
        match try_into_uniswap_v2_router(router, &input) {
            Some(UniswapV2RouterFuncs::SwapTokensForEth(_)) => {}
            Some(_) => panic!("wrong function match"),
            None => panic!("no function match"),
        };
    }

    #[test]
    fn test_try_into_uniswap_v2_router_swap_eth_for() {
        let abis = ABI::new();
        let router = BaseContract::from(abis.uniswap_v2_router.clone());

        let input = Bytes::from(
            hex::decode(
                "0xb6f9de950000000000000000000000000000000000000000000000000de0b6b3a764000000000000000000000000000000000000000000000000000000000000000000800000000000000000000000003e88f4e2e4f78acd018a463b69eb1ed8d011111a0000000000000000000000000000000000000000000000000000018b2d37bf480000000000000000000000000000000000000000000000000000000000000002000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc20000000000000000000000000b9ae6b1d4f0eeed904d1cef68b9bd47499f3fff",
            )
            .unwrap(),
        );
        match try_into_uniswap_v2_router(router, &input) {
            Some(UniswapV2RouterFuncs::SwapEthForTokens(_)) => {}
            Some(_) => panic!("wrong function match"),
            None => panic!("no function match"),
        };
    }

    #[test]
    fn test_try_into_uniswap_v2_router_add_liquidity_eth() {
        let abis = ABI::new();
        let router = BaseContract::from(abis.uniswap_v2_router.clone());

        let input = Bytes::from(
            hex::decode(
                "0xf305d7190000000000000000000000000cfeb7799cc72838e92c28902bac5c163ca2be9b0000000000000000000000000000000000000000204fce5e3e250261100000000000000000000000000000000000000000000000204fce5e3e25026110000000000000000000000000000000000000000000000000000001f399b1438a100000000000000000000000000000639c24bdc36a8f015d36f172ecd961ea5d1cd73100000000000000000000000000000000000000000000000000000000652a4b9f",
            )
            .unwrap(),
        );
        match try_into_uniswap_v2_router(router, &input) {
            Some(UniswapV2RouterFuncs::AddLiquidityETH(_)) => {}
            Some(_) => panic!("wrong function match"),
            None => panic!("no function match"),
        };
    }

    #[test]
    fn test_try_into_uniswap_v2_router_remove_liquidity_eth_with_permit() {
        let abis = ABI::new();
        let router = BaseContract::from(abis.uniswap_v2_router.clone());

        let input = Bytes::from(
            hex::decode(
                "0xded9382a000000000000000000000000960c4b7d00140e544cc78f3f20ac1bf519162b48000000000000000000000000000000000000000000000000918d2ce7e4af4ce500000000000000000000000000000000000000000000000034419ea35fa83d4e0000000000000000000000000000000000000000000000016ef9a53e10e2f54c000000000000000000000000a1913435e4e6d96324406b16694e259c72d5859900000000000000000000000000000000000000000000000000000000652a61770000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001ccace7af2f336001a508aedaaca8aa791a111c0c89d79b0e3a1845b50a4e48c9b43b51372492a62d8d32b6150c3ed675432e4cc6cfddc76a8ac5181c8e559b616",
            )
            .unwrap(),
        );
        match try_into_uniswap_v2_router(router, &input) {
            Some(UniswapV2RouterFuncs::RemoveLiquidityEthWithPermit(_)) => {}
            Some(_) => panic!("wrong function match"),
            None => panic!("no function match"),
        };
    }

    #[test]
    fn test_try_into_uniswap_v2_router_remove_liquidity_with_permit() {
        let abis = ABI::new();
        let router = BaseContract::from(abis.uniswap_v2_router.clone());

        let input = Bytes::from(
            hex::decode(
                "0x2195995c0000000000000000000000006a3c3db39454416fc9e08da0fe10649fdedca83d000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2000000000000000000000000000000000000000000000000003b4c438798341c000000000000000000000000000000000000000000000000006b7e60c2801256000000000000000000000000000000000000000000000000001cf0b155b310b2000000000000000000000000ef190aa4ad08dad97d9575484e37645f26c4ce1900000000000000000000000000000000000000000000000000000000652a71d30000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001b7a0c521e2d2ef3308c9a61ef55a1f93ff2b237aaa5e460132f87a6c9b00cd58818d0b575fb865bee7f7e47a1fe147f2168017345f2b7852908656b28e5941414",
            )
            .unwrap(),
        );
        match try_into_uniswap_v2_router(router, &input) {
            Some(UniswapV2RouterFuncs::RemoveLiquidityWithPermit(_)) => {}
            Some(_) => panic!("wrong function match"),
            None => panic!("no function match"),
        };
    }
}
