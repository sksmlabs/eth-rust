use ethers::providers::{Provider, Http, Middleware};
use ethers::types::{Block, H256, BlockNumber};
use std::sync::Arc;
use anyhow::Result;

pub async fn get_latest_block(provider: Arc<Provider<Http>>) -> Result<()> {
    let latest_block: Option<Block<H256>> = provider.get_block(BlockNumber::Latest).await?;
    match latest_block {
    Some(block) => {
        println!("Block Info");
        println!("Latest block number: {:?}", block.number.unwrap_or_default());
        println!("Latest block hash: {:?}", block.hash.unwrap_or_default());
        println!("Latest block transactions: {}", block.transactions.len());
        println!("Latest block timestamp: {}", block.timestamp);
    },
        None => println!("No block found"),
    }
    Ok(())
}
