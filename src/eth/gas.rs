use anyhow::{anyhow, Result};
use ethers_core::types::U256;
use ethers_core::{types::H256, utils::format_units};
use ethers_providers::{Http, Middleware, Provider};
use rand::Rng;
use std::{fmt::Display, ops::Add};

#[derive(Debug, Clone)]
pub struct Gas {
    /// The gas price (gas_price)
    gas_price: U256,
    /// The base fee derived from the [`ethers::types::Block`]
    base_fee: U256,
    /// The maximum fee known as `gas_tip_cap` in EVM
    max_fee: U256,
    /// The maximum priority fee known as `gas_fee_cap` in EVM
    max_prio_fee: U256,
    /// The amount of gas used from the [`ethers::types::TransactionReceipt`]
    gas_used: U256,
    /// The total cost of the transaction in wei
    tx_cost: U256,
}

impl Gas {
    /// Creates a new Gas struct that automagically calculates the transaction cost.
    ///
    /// * `base_fee` - The base fee derived from the [`ethers::types::Block`]
    /// * `gas_used` - The amount of gas used from the [`ethers::types::TransactionReceipt`]
    /// * `max_fee` - The maximum fee known as `gas_tip_cap` in EVM
    /// * `max_prio_fee` - The maximum priority fee known as `gas_fee_cap` in EVM
    pub fn new(gas_used: U256, base_fee: U256, max_fee: U256, max_prio_fee: U256) -> Self {
        let mut gas_price = base_fee.add(max_prio_fee);
        let mut tx_cost = (base_fee + max_prio_fee) * gas_used;
        if max_fee == max_prio_fee {
            gas_price = max_fee;
            tx_cost = gas_price * gas_used;
        }

        Gas {
            gas_price: gas_price,
            base_fee,
            max_fee,
            max_prio_fee,
            gas_used: gas_used,
            tx_cost,
        }
    }

    /// Creates a new Gas struct from a transaction hash.
    ///
    /// * `provider` - The [`ethers_providers::Provider`] to use
    /// * `hash` - The transaction hash
    ///
    /// Returns a [`Result`] containing a [`Gas`] struct.
    ///
    /// # Example
    /// ```no-run
    /// # use ethers::providers::{Http, Middleware, Provider};
    /// # use ethers::types::H256;
    /// # use memers::utils::Gas;
    /// #
    /// #[tokio::main]
    /// async fn main() {
    ///    let provider = Provider::<Http>::connect("https://mainnet.infura.io/v3/").await.unwrap();
    ///    let hash = H256::from_str("0x16423f8d473fd5045bccdf5626a064f45284bd270d7ab7f910ae02d1231c0857").unwrap();
    ///    let gas = Gas::from_h256(provider, hash).await.unwrap();
    ///    println!("{}", gas);
    /// }
    /// ```
    pub async fn from_h256(provider: Provider<Http>, hash: H256) -> Result<Gas> {
        let tx = match provider.get_transaction(hash).await {
            Ok(tx) => match tx {
                Some(tx) => tx,
                None => return Err(anyhow!("Could not get transaction")),
            },
            Err(e) => return Err(anyhow!("Could not get transaction: {}", e)),
        };

        let receipt = match provider.get_transaction_receipt(hash).await {
            Ok(receipt) => match receipt {
                Some(receipt) => receipt,
                None => return Err(anyhow!("Could not get transaction receipt")),
            },
            Err(e) => return Err(anyhow!("Could not get transaction receipt: {}", e)),
        };

        let block = match provider.get_block(receipt.block_number.unwrap()).await {
            Ok(block) => match block {
                Some(block) => block,
                None => return Err(anyhow!("Could not get block")),
            },
            Err(e) => return Err(anyhow!("Could not get block: {}", e)),
        };

        let gas = Gas::new(
            receipt.gas_used.unwrap(),
            block.base_fee_per_gas.unwrap(),
            tx.max_fee_per_gas.unwrap(),
            tx.max_priority_fee_per_gas.unwrap(),
        );

        Ok(gas)
    }

    /// Returns the gas price in wei
    pub fn gas_price(&self) -> U256 {
        self.gas_price
    }

    /// Returns the gas price in gwei
    pub fn gas_price_as_gwei(&self) -> String {
        format_units(self.gas_price, "gwei").unwrap()
    }

    /// Returns the base fee in wei
    pub fn base_fee(&self) -> U256 {
        self.base_fee
    }

    /// Returns the base fee in gwei
    pub fn base_fee_as_gwei(&self) -> String {
        format_units(self.base_fee, "gwei").unwrap()
    }

    /// Returns the maximum fee in wei
    pub fn max_fee(&self) -> U256 {
        self.max_fee
    }

    /// Returns the maximum fee in gwei
    pub fn max_fee_as_gwei(&self) -> String {
        format_units(self.max_fee, "gwei").unwrap()
    }

    /// Returns the maximum priority fee in wei
    pub fn max_priority_fee(&self) -> U256 {
        self.max_prio_fee
    }

    /// Returns the maximum priority fee in gwei
    pub fn max_prio_fee_as_gwei(&self) -> String {
        format_units(self.max_prio_fee, "gwei").unwrap()
    }

    /// Returns the amount of gas used in wei
    pub fn gas_used(&self) -> U256 {
        self.gas_used
    }

    /// Returns the total cost of the transaction in wei
    pub fn tx_cost(&self) -> U256 {
        self.tx_cost
    }

    /// Returns the total cost of the transaction in ether
    pub fn tx_cost_as_eth(&self) -> String {
        format_units(self.tx_cost, "ether").unwrap()
    }
}

impl Display for Gas {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "price: {} gwei, base fee: {} gwei, max fee: {} gwei, max prio fee: {} gwei, gas used: {} wei, tx cost: {} eth",
            self.gas_price_as_gwei(),
            self.base_fee_as_gwei(),
            self.max_fee_as_gwei(),
            self.max_prio_fee_as_gwei(),
            self.gas_used(),
            self.tx_cost_as_eth()
        )
    }
}

/// Returns the next block base fee using the naive algorithm.
/// https://github.com/solidquant/mev-templates/blob/main/rust/src/utils.rs#L42
pub fn naive_next_block_base_fee(gas_used: U256, gas_limit: U256, base_fee_per_gas: U256) -> U256 {
    let gas_used = gas_used;

    let mut target_gas_used = gas_limit / 2;
    target_gas_used = if target_gas_used == U256::zero() {
        U256::one()
    } else {
        target_gas_used
    };

    let new_base_fee = {
        if gas_used > target_gas_used {
            base_fee_per_gas
                + ((base_fee_per_gas * (gas_used - target_gas_used)) / target_gas_used)
                    / U256::from(8u64)
        } else {
            base_fee_per_gas
                - ((base_fee_per_gas * (target_gas_used - gas_used)) / target_gas_used)
                    / U256::from(8u64)
        }
    };

    let seed = rand::thread_rng().gen_range(0..9);
    new_base_fee + seed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_display_gas_properly() {
        // Is this the right way to test this? I'm not sure. Is it even worth testing? Seems redundant in a way.
        // This tests the underlying *_converted() functions, but not the Display trait itself.
        // If anything, this can be used for documentation purposes.
        let base_fee = U256::from(8681938922u64);
        let gas_used = U256::from(182197u64);
        let max_fee = U256::from(9121299698u64);
        let max_prio_fee = U256::from(50000000u64);

        let expected_display = "price: 8.731938922 gwei, base fee: 8.681938922 gwei, max fee: 9.121299698 gwei, max prio fee: 0.050000000 gwei, gas used: 182197 wei, tx cost: 0.001590933075771634 eth";
        let expected_debug = "Gas { gas_price: 8731938922, base_fee: 8681938922, max_fee: 9121299698, max_prio_fee: 50000000, gas_used: 182197, tx_cost: 1590933075771634 }";

        let actual = Gas::new(gas_used, base_fee, max_fee, max_prio_fee);
        assert_eq!(expected_display, actual.to_string());
        assert_eq!(expected_display, format!("{}", actual));
        assert_eq!(expected_debug, format!("{:?}", actual));
    }

    #[test]
    fn it_should_calculate_the_correct_transaction_cost() {
        let base_fee = U256::from(8681938922u64);
        let gas_used = U256::from(182197u64);
        let max_fee = U256::from(9121299698u64);
        let max_prio_fee = U256::from(50000000u64);

        let expected = Gas {
            gas_price: base_fee + max_prio_fee,
            base_fee,
            max_fee,
            max_prio_fee,
            gas_used,
            tx_cost: U256::from(1590933075771634u64),
        };

        let actual = Gas::new(gas_used, base_fee, max_fee, max_prio_fee);
        assert_eq!(expected.gas_price, actual.gas_price);
        assert_eq!(expected.base_fee, actual.base_fee);
        assert_eq!(expected.max_fee, actual.max_fee);
        assert_eq!(expected.max_prio_fee, actual.max_prio_fee);
        assert_eq!(expected.gas_used, actual.gas_used);
        assert_eq!(expected.tx_cost, actual.tx_cost);
    }

    #[test]
    fn it_should_calculate_the_correct_transaction_cost_when_max_fee_equals_max_priority_fee() {
        let base_fee = U256::from(8681938922u64);
        let gas_used = U256::from(182197u64);
        let max_fee = U256::from(50000000u64);
        let max_prio_fee = U256::from(50000000u64);

        let expected = Gas {
            gas_price: max_fee,
            base_fee,
            max_fee,
            max_prio_fee,
            gas_used,
            tx_cost: U256::from(9109850000000u64),
        };

        let actual = Gas::new(gas_used, base_fee, max_fee, max_prio_fee);
        assert_eq!(expected.gas_price, actual.gas_price);
        assert_eq!(expected.max_fee, actual.max_fee);
        assert_eq!(expected.max_prio_fee, actual.max_prio_fee);
        assert_eq!(expected.gas_used, actual.gas_used);
        assert_eq!(expected.tx_cost, actual.tx_cost);
    }

    #[test]
    fn it_should_calculate_the_correct_transaction_cost_when_max_fee_equals_gas_price() {
        let base_fee = U256::from(8681938922u64);
        let gas_used = U256::from(182197u64);
        let max_fee = U256::from(8681938922u64);
        let max_prio_fee = U256::from(50000000u64);

        let expected = Gas {
            gas_price: base_fee + max_prio_fee,
            base_fee,
            max_fee,
            max_prio_fee,
            gas_used,
            tx_cost: U256::from(1590933075771634u64),
        };

        let actual = Gas::new(gas_used, base_fee, max_fee, max_prio_fee);
        assert_eq!(expected.gas_price, actual.gas_price);
        assert_eq!(expected.max_fee, actual.max_fee);
        assert_eq!(expected.max_prio_fee, actual.max_prio_fee);
        assert_eq!(expected.gas_used, actual.gas_used);
        assert_eq!(expected.tx_cost, actual.tx_cost);
    }
}
