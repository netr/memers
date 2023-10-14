use anyhow::{anyhow, Result};
use ethers_core::types::U256;
use reqwest::header;
use serde::{Deserialize, Serialize};

pub async fn get_block_prices(api_key: &str) -> Result<BlockPrices> {
    let mut headers = header::HeaderMap::new();
    headers.insert("Authorization", header::HeaderValue::from_str(api_key)?);

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    let body = client
        .get("https://api.blocknative.com/gasprices/blockprices")
        .send()
        .await?
        .json::<BlockPrices>()
        .await?;

    Ok(body)
}

impl BlockPrices {
    pub fn next_base_fee(&self) -> Result<U256> {
        let estimated = self
            .estimated_base_fees
            .iter()
            .find(|x| x.pending_1.len() > 0)
            .ok_or(anyhow!("No estimated base fee found"))?;

        // convert base fee into wei
        let base_fee = estimated.pending_1[0].base_fee;
        let base_fee = base_fee * 1e9;

        Ok(U256::from(base_fee as u64))
    }

    pub fn estimated_price(&self) -> Result<EstimatedPrice> {
        let estimated = self
            .block_prices
            .iter()
            .find(|x| x.estimated_prices.len() > 0)
            .ok_or(anyhow!("No estimated gas price found"))?;

        Ok(estimated.estimated_prices[0].clone())
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockPrices {
    pub system: String,
    pub network: String,
    pub unit: String,
    pub max_price: i64,
    pub current_block_number: i64,
    pub ms_since_last_block: i64,
    pub block_prices: Vec<BlockPrice>,
    pub estimated_base_fees: Vec<EstimatedBaseFee>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockPrice {
    pub block_number: i64,
    pub estimated_transaction_count: i64,
    pub base_fee_per_gas: f64,
    pub estimated_prices: Vec<EstimatedPrice>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EstimatedPrice {
    pub confidence: i64,
    pub price: i64,
    pub max_priority_fee_per_gas: f64,
    pub max_fee_per_gas: f64,
}

impl EstimatedPrice {
    pub fn price_to_u256(&self) -> U256 {
        let price = self.price as f64;
        let price = price * 1e9;

        U256::from(price as u64)
    }

    pub fn max_prio_fee_to_u256(&self) -> U256 {
        let fee = self.max_priority_fee_per_gas as f64;
        let fee = fee * 1e9;

        U256::from(fee as u64)
    }

    pub fn max_fee_to_u256(&self) -> U256 {
        let fee = self.max_fee_per_gas as f64;
        let fee = fee * 1e9;

        U256::from(fee as u64)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EstimatedBaseFee {
    #[serde(rename = "pending+1")]
    #[serde(default)]
    pub pending_1: Vec<Pending1>,
    #[serde(rename = "pending+2")]
    #[serde(default)]
    pub pending_2: Vec<Pending2>,
    #[serde(rename = "pending+3")]
    #[serde(default)]
    pub pending_3: Vec<Pending3>,
    #[serde(rename = "pending+4")]
    #[serde(default)]
    pub pending_4: Vec<Pending4>,
    #[serde(rename = "pending+5")]
    #[serde(default)]
    pub pending_5: Vec<Pending5>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pending1 {
    pub confidence: i64,
    pub base_fee: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pending2 {
    pub confidence: i64,
    pub base_fee: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pending3 {
    pub confidence: i64,
    pub base_fee: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pending4 {
    pub confidence: i64,
    pub base_fee: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pending5 {
    pub confidence: i64,
    pub base_fee: f64,
}
