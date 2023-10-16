use std::sync::Arc;

use ethers_core::types::{Address, U256};
use ethers_providers::{Http, Provider};

use crate::abi::erc20::ERC20;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct ERC20Token {
    address: Address,
    creator: Address,
    name: String,
    symbol: String,
    decimals: u8,
    total_supply: U256,
}

impl ERC20Token {
    pub fn new(
        address: Address,
        creator: Address,
        name: String,
        symbol: String,
        decimals: u8,
        total_supply: U256,
    ) -> Self {
        ERC20Token {
            address,
            creator,
            name,
            symbol,
            decimals,
            total_supply,
        }
    }

    pub fn address(&self) -> Address {
        self.address.clone()
    }

    pub fn creator(&self) -> Address {
        self.creator.clone()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn symbol(&self) -> String {
        self.symbol.clone()
    }

    pub fn decimals(&self) -> u8 {
        self.decimals
    }

    pub fn total_supply(&self) -> U256 {
        self.total_supply
    }
}

pub async fn get_erc20_token_vars(
    erc20_address: Address,
    creator_address: Address,
    provider: Arc<Provider<Http>>,
) -> Result<ERC20Token> {
    let erc20 = ERC20::new(erc20_address, provider);
    let name = erc20.name().await?;
    let symbol = erc20.symbol().await?;
    let decimals = erc20.decimals().await?;
    let total_supply = erc20.total_supply().await?;

    Ok(ERC20Token::new(
        erc20_address,
        creator_address,
        name,
        symbol,
        decimals,
        total_supply,
    ))
}
