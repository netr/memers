use ethers_contract::{decode_function_data, BaseContract};
use ethers_core::{
    abi::FixedBytes,
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
    pub fn from_input(contract: BaseContract, tx_input: &Bytes) -> Option<UniswapV2RouterFuncs> {
        let funcs = contract
            .as_ref()
            .functions()
            .filter(|function| {
                function.name.starts_with("swap")
                    || function.name.starts_with("addLiquidity")
                    || function.name.starts_with("removeLiquidity")
            })
            .into_iter()
            .map(|function| {
                if function.name.starts_with("swapETH") || function.name.starts_with("swapExactETH")
                {
                    if let Ok(func) =
                        decode_function_data::<SwapEthForTokens, _>(function, tx_input, true)
                    {
                        return Some(UniswapV2RouterFuncs::SwapEthForTokens(func));
                    }
                }
                if function.name.starts_with("swap") {
                    if let Ok(func) =
                        decode_function_data::<SwapTokensForEth, _>(function, tx_input, true)
                    {
                        return Some(UniswapV2RouterFuncs::SwapTokensForEth(func));
                    }
                }
                if function.name.starts_with("addLiquidity") {
                    if let Ok(func) =
                        decode_function_data::<AddLiquidity, _>(function, tx_input, true)
                    {
                        return Some(UniswapV2RouterFuncs::AddLiquidity(func));
                    }
                    if let Ok(func) =
                        decode_function_data::<AddLiquidityETH, _>(function, tx_input, true)
                    {
                        return Some(UniswapV2RouterFuncs::AddLiquidityETH(func));
                    }
                }
                if function.name.starts_with("removeLiquidity") {
                    if let Ok(func) = decode_function_data::<RemoveLiquidityEthWithPermit, _>(
                        function, tx_input, true,
                    ) {
                        return Some(UniswapV2RouterFuncs::RemoveLiquidityEthWithPermit(func));
                    }
                    if let Ok(func) = decode_function_data::<RemoveLiquidityWithPermit, _>(
                        function, tx_input, true,
                    ) {
                        return Some(UniswapV2RouterFuncs::RemoveLiquidityWithPermit(func));
                    }
                    if let Ok(func) =
                        decode_function_data::<RemoveLiquidity, _>(function, tx_input, true)
                    {
                        return Some(UniswapV2RouterFuncs::RemoveLiquidity(func));
                    }
                    if let Ok(func) =
                        decode_function_data::<RemoveLiquidityETH, _>(function, tx_input, true)
                    {
                        return Some(UniswapV2RouterFuncs::RemoveLiquidityETH(func));
                    }
                }

                None
            });

        if let Some(res) = funcs.filter(|func| func.is_some()).next() {
            return res;
        }

        None
    }
}

pub fn try_into_uniswap_v2_router(
    contract: BaseContract,
    input: &Bytes,
) -> Option<UniswapV2RouterFuncs> {
    if let Some(router_func) = UniswapV2RouterFuncs::from_input(contract, input) {
        info!("{:#?}", router_func);
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
                "0x2195995c000000000000000000000000a408808f0960f811cd6c4cbf4af6d61f18ad8703000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc20000000000000000000000000000000000000000000000000175678b9fe819180000000000000000000000000000000000000000000000000024667f1e6a9cde0000000000000000000000000000000000000000000000000409ad26cd209907000000000000000000000000913bb6b20f04f6b36e2e835a25e1e065554435a400000000000000000000000000000000000000000000000000000000652a64ef0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001b0a56ec18f4ea0440863ad8c7f59da990aaa5108f341147c92b9e26d9fbf87ea3200a590729eeee982dbb7b4b5a83b0000cfa54f30361ccaf5d71fa98d22301d4",
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
