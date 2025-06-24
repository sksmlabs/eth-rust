use ethers::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;
use anyhow::Result;
use crate::account::token::ERC20;
use crate::contracts::{USDT_CONTRACT, USDC_CONTRACT, DAI_CONTRACT, WBTC_CONTRACT, LINK_CONTRACT, CSUSDL_CONTRACT};

// Example mapping for popular tokens on Ethereum mainnet
fn token_address_map() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    map.insert("USDT", USDT_CONTRACT);
    map.insert("USDC", USDC_CONTRACT);
    map.insert("DAI",  DAI_CONTRACT);
    map.insert("WBTC", WBTC_CONTRACT);
    map.insert("LINK", LINK_CONTRACT);
    map.insert("CSUSDL", CSUSDL_CONTRACT);
    map
}

pub async fn get_token_balances(
    provider: Arc<Provider<Http>>,
    address: Address,
    symbols: &[&str],
) -> Result<Vec<(String, f64)>> {
    let map = token_address_map();
    let mut balances = Vec::new();

    // If symbols is empty, use all keys from the map
    let tokens_to_check: Vec<&str> = if symbols.is_empty() {
        map.keys().copied().collect()
    } else {
        symbols.to_vec()
    };

    

    for symbol in tokens_to_check {
        if symbol == "ETH" {
            let balance = provider.get_balance(address, None).await?;
            balances.push(("ETH".to_string(), balance.as_u128() as f64 / 10f64.powi(18 as i32)));
            continue;
        }
        if let Some(&token_addr) = map.get(symbol) {
            let token_address: Address = token_addr.parse()?;
            let token = ERC20::new(token_address, provider.clone());
            let balance = token.balance_of(address).call().await?;
            let decimals = token.decimals().call().await?;
            let balance_float = balance.as_u128() as f64 / 10f64.powi(decimals as i32);
            balances.push((symbol.to_string(), balance_float));
        } else {
            balances.push((symbol.to_string(), 0.0)); // or return an error if preferred
        }
    }
    Ok(balances)
} 