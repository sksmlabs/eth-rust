use ethers::prelude::*;
use std::sync::Arc;
use anyhow::Result;
use crate::contracts::ETH_USD_PRICE_FEED;

// AggregatorV3Interface ABI for price feeds
abigen!(
    AggregatorV3Interface,
    r#"[
        function decimals() external view returns (uint8)
        function description() external view returns (string)
        function version() external view returns (uint256)
        function getRoundData(uint80 _roundId) external view returns (uint80 roundId, int256 answer, uint256 startedAt, uint256 updatedAt, uint80 answeredInRound)
        function latestRoundData() external view returns (uint80 roundId, int256 answer, uint256 startedAt, uint256 updatedAt, uint80 answeredInRound)
    ]"#
);

pub struct PriceFeed {
    contract: AggregatorV3Interface<Provider<Http>>,
}

impl PriceFeed {
    pub fn new(provider: Arc<Provider<Http>>) -> Self {
        let address: Address = ETH_USD_PRICE_FEED.parse().expect("Invalid address");
        let contract = AggregatorV3Interface::new(address, provider);
        Self { contract }
    }

    pub async fn get_latest_price(&self) -> Result<f64> {
        let (_, price, _, _, _) = self.contract.latest_round_data().call().await?;
        let decimals = self.contract.decimals().call().await?;
        
        // Convert price to float, accounting for decimals
        let price_float = price.as_u128() as f64 / 10f64.powi(decimals as i32);
        
        Ok(price_float)
    }

    pub async fn get_price_with_timestamp(&self) -> Result<(f64, u64)> {
        let (_, price, _, timestamp, _) = self.contract.latest_round_data().call().await?;
        let decimals = self.contract.decimals().call().await?;
        
        // Convert price to float, accounting for decimals
        let price_float = price.as_u128() as f64 / 10f64.powi(decimals as i32);
        
        Ok((price_float, timestamp.as_u64()))
    }

    pub async fn get_description(&self) -> Result<String> {
        let description = self.contract.description().call().await?;
        Ok(description)
    }
}