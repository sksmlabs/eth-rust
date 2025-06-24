mod block;
mod constants;
mod contracts;
mod chains;
mod account;
mod chainlink;
mod uniswap;
mod balancer;
mod curve;
mod arbitrage;

use anyhow::Result;
use ethers::prelude::*;
use ethers::providers::{Provider, Http, Ws, StreamExt};
use ethers::types::{U256};
use ethers::providers::Middleware;
use ethers::middleware::SignerMiddleware;
use std::convert::TryFrom;
use std::sync::Arc;

// modules
use constants::{ACCOUNT_PRIVATE_KEY, RECIPIENT_ADDRESS};
use chains::{CHAIN_ETHEREUM};
use account::token_balances::get_token_balances;
use account::token_transfer::transfer_eth;
use chainlink::price::PriceFeed;
use uniswap::pool::PoolUniswap;
use balancer::pool::PoolBalancer;
use contracts::{UNISWAP_ETHEREUM_WETH_USDC, BALANCER_ETHEREUM_BCoW_50WETH_50USDC};
use arbitrage::index::call_arbitrage;
use block::index::get_latest_block;

#[tokio::main]
async fn main() -> Result<()> {
    // modify this code such that we shall be giving an argument to cargo run to select the function to run
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: cargo run -- <function_name>");
        return Ok(());
    }

    // Chain to use
    let CHAIN = CHAIN_ETHEREUM;
    
    // Connect to Ethereum provider
    let provider = Arc::new(Provider::<Http>::try_from(CHAIN.rpc_url)?);

    // Fetch latest block number
    if args.contains(&"block".to_string()) {
        get_latest_block(provider.clone()).await?;
    }

    // Note: Transaction subscription requires WebSocket provider, not HTTP
    if args.contains(&"subscribe".to_string()) {
        println!("Connecting to WebSocket for transaction subscription...");
        
        match Provider::<Ws>::connect(CHAIN.ws_url).await {
            Ok(ws_provider) => {
                let mut stream = ws_provider.subscribe_pending_txs().await?;
                println!("Listening for pending transactions... (Press Ctrl+C to stop)");
                
                let mut count = 0;
                while let Some(tx_hash) = stream.next().await {
                    println!("New pending transaction #{}: {:?}", count + 1, tx_hash);
                    count += 1;
                    
                    // Limit to first 10 transactions for demo purposes
                    if count >= 100 {
                        println!("Received 10 transactions, stopping subscription.");
                        break;
                    }
                }
            }
            Err(e) => {
                println!("Failed to connect to WebSocket: {}", e);
                println!("Make sure the WebSocket URL is correct and accessible.");
            }
        }
    }

    // Create wallet from private key using ethers LocalWallet with correct chain ID
    let private_key = ACCOUNT_PRIVATE_KEY.trim_start_matches("0x");
    let private_key_bytes = hex::decode(private_key)?;
    let wallet = LocalWallet::from_bytes(&private_key_bytes)?.with_chain_id(CHAIN.chain_id);
    let address = wallet.address();
    let client = Arc::new(SignerMiddleware::new(provider.clone(), wallet));
    println!("\n");
    println!("Wallet from private key:");
    println!("Address: {:?}", address);

    // List of token symbols to check
    if args.contains(&"account_balances".to_string()) {
        let symbols = ["ETH",];
        let balances = get_token_balances(provider.clone(), address, &symbols).await?;
        for (symbol, balance) in balances {
            println!("\n");
            println!("Wallet Balance");
            println!("{} Balance: {}", symbol, balance);
        }
    }
    
    // Fetch ETH/USD price from Chainlink
    if args.contains(&"chainlink".to_string()) {
        let price_feed = PriceFeed::new(provider.clone());
        let (price, timestamp) = price_feed.get_price_with_timestamp().await?;
        println!("\n");
        println!("Chainlink ETH/USD Price Feed:");
        println!("Description: {}", price_feed.get_description().await?);
        println!("Latest Price: ${:.2}", price);
        println!("Last Updated: {}", timestamp);
    }
   
    // Get Uniswap pool info
    if args.contains(&"pool_uniswap".to_string()) {
        let mut pool = PoolUniswap::new(provider.clone(), UNISWAP_ETHEREUM_WETH_USDC.parse().expect("invalid address"));
        pool.get_pool_info().await?;
        println!("\n");
    }

    // Get Balancer pool info
    if args.contains(&"pool_balancer".to_string()) {
        let mut pool = PoolBalancer::new(provider.clone(), BALANCER_ETHEREUM_BCoW_50WETH_50USDC.parse().expect("invalid address")).await?;
        pool.get_pool_info().await?;
        println!("\n");
    }

    // Arbitrage
    if args.contains(&"arbitrage".to_string()) {
        call_arbitrage(provider.clone()).await?;
        println!("\n");
    }

    // Transfer tokens
    if args.contains(&"account_transfer".to_string()) {
        let recipient = RECIPIENT_ADDRESS.parse()?;
        // Transfer ETH
        let amount = U256::from(1000000000000000u64); // 0.001 ETH in wei
        println!("\n");
        transfer_eth(client.clone(), address, recipient, amount).await.map_err(|e| anyhow::anyhow!(e))?;
    }

    Ok(())
}
