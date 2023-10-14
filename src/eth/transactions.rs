use std::str::FromStr;

use anyhow::{anyhow, Result};
use ethers::prelude::H256;
use ethers_core::types::{Address, BlockId, Bytes, H160, U256, U64};
use ethers_providers::{Http, Middleware, Provider};

pub async fn get_transactions_from_block(
    provider: Provider<Http>,
    block_hash_or_number: BlockId,
) -> Result<BlockWithTransactions> {
    match provider.get_block_with_txs(block_hash_or_number).await {
        Ok(block) => match block {
            Some(block) => {
                let mut block_with_tx = BlockWithTransactions {
                    block_number: block.number.unwrap_or_default(),
                    block_hash: block.hash.unwrap_or_default(),
                    transactions: vec![],
                };

                let txs = block
                    .transactions
                    .iter()
                    .map(|tx| Transaction::from(tx.to_owned()))
                    .collect::<Vec<Transaction>>();

                block_with_tx.transactions = txs;
                return Ok(block_with_tx);
            }
            None => {
                return Err(anyhow!("Could not get block with transactions"));
            }
        },
        Err(e) => {
            return Err(anyhow!("Could not get block with transactions, {}", e));
        }
    }
}

#[derive(Debug, Clone)]
pub struct BlockWithTransactions {
    pub block_number: U64,
    pub block_hash: H256,
    pub transactions: Vec<Transaction>,
}

#[derive(Debug, Clone)]
pub struct Transaction {
    pub hash: H256,
    pub from: H160,
    pub to: H160,
    pub value: U256,
    pub gas_price: U256,
    pub max_fee_per_gas: U256,
    pub max_priority_fee_per_gas: U256,
    pub input: Bytes,
}

impl Transaction {
    pub fn is_to_address(&self, address: &str) -> bool {
        self.to == Address::from_str(address).expect("could not parse transaction's to address")
    }
}

impl From<ethers::types::Transaction> for Transaction {
    fn from(tx: ethers::types::Transaction) -> Self {
        let gas_price = tx.gas_price.unwrap_or(U256::default());
        let max_fee = tx.max_fee_per_gas.unwrap_or(U256::default());
        let max_priority_fee = tx.max_priority_fee_per_gas.unwrap_or(U256::default());

        Transaction {
            hash: tx.hash,
            from: tx.from,
            to: tx.to.unwrap_or(H160::default()),
            value: tx.value,
            gas_price,
            max_fee_per_gas: max_fee,
            max_priority_fee_per_gas: max_priority_fee,
            input: tx.input,
        }
    }
}
