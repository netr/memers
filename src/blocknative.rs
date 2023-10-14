use anyhow::{anyhow, Result};
use ethers_core::types::U256;
use reqwest::header;
use serde::{Deserialize, Serialize};

pub struct GasPrediction {
    pub base_fee: U256,
    pub next_base_fee: U256,
    pub gas_price: U256,
    pub max_fee: U256,
    pub max_prio_fee: U256,
}

impl From<BlockPrices> for GasPrediction {
    fn from(block_prices: BlockPrices) -> Self {
        let base_fee = block_prices.base_fee().unwrap_or_default();
        let next_base_fee = block_prices.next_base_fee().unwrap_or_default();
        let estimated_price = block_prices.estimated_price().unwrap_or_default();

        GasPrediction {
            base_fee,
            next_base_fee,
            gas_price: estimated_price.price_to_u256(),
            max_fee: estimated_price.max_fee_to_u256(),
            max_prio_fee: estimated_price.max_prio_fee_to_u256(),
        }
    }
}

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

    pub fn base_fee(&self) -> Result<U256> {
        let estimated = self
            .block_prices
            .iter()
            .find(|x| x.estimated_prices.len() > 0)
            .ok_or(anyhow!("No estimated gas price found"))?;

        // convert base fee into wei
        let base_fee = estimated.base_fee_per_gas;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_create_a_gas_prediction_from_block_prices() {
        let mock_block_prices = BlockPrices {
            system: "ethereum".to_string(),
            network: "main".to_string(),
            unit: "gwei".to_string(),
            max_price: 1000,
            current_block_number: 123456,
            ms_since_last_block: 1000,
            block_prices: vec![BlockPrice {
                block_number: 123456,
                estimated_transaction_count: 100,
                base_fee_per_gas: 69.0,
                estimated_prices: vec![EstimatedPrice {
                    confidence: 95,
                    price: 87,
                    max_priority_fee_per_gas: 1111.0,
                    max_fee_per_gas: 100.0,
                }],
            }],
            estimated_base_fees: vec![EstimatedBaseFee {
                pending_1: vec![Pending1 {
                    confidence: 95,
                    base_fee: 7.0,
                }],
                pending_2: vec![Pending2 {
                    confidence: 95,
                    base_fee: 100.0,
                }],
                pending_3: vec![Pending3 {
                    confidence: 95,
                    base_fee: 100.0,
                }],
                pending_4: vec![Pending4 {
                    confidence: 95,
                    base_fee: 100.0,
                }],
                pending_5: vec![Pending5 {
                    confidence: 95,
                    base_fee: 100.0,
                }],
            }],
        };

        let gas_prediction = GasPrediction::from(mock_block_prices);
        assert_eq!(gas_prediction.base_fee, U256::from(69_000_000_000u64));
        assert_eq!(gas_prediction.next_base_fee, U256::from(7_000_000_000u64));
        assert_eq!(gas_prediction.gas_price, U256::from(87_000_000_000u64));
        assert_eq!(gas_prediction.max_fee, U256::from(100_000_000_000u64));
        assert_eq!(
            gas_prediction.max_prio_fee,
            U256::from(1_111_000_000_000u64)
        );
    }
}
