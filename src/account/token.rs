use ethers::prelude::*;
use std::sync::Arc;
use anyhow::Result;


abigen!(
    ERC20,
    r"[
        function balanceOf(address) view returns (uint256)
        function decimals() view returns (uint8)
        function symbol() view returns (string)
    ]"
);

pub async fn get_eth_balance(provider: Arc<Provider<Http>>, address: Address) -> Result<U256> {
    let balance = provider.get_balance(address, None).await?;
    Ok(balance)
}

pub async fn get_erc20_balance(
    provider: Arc<Provider<Http>>,
    token_address: Address,
    address: Address,
) -> Result<(String, f64)> {
    let token = ERC20::new(token_address, provider.clone());
    let balance = token.balance_of(address).call().await?;
    let decimals = token.decimals().call().await?;
    let symbol = token.symbol().call().await?;
    let balance_float = balance.as_u128() as f64 / 10f64.powi(decimals as i32);
    Ok((symbol, balance_float))
} 