use crate::uniswap::pool::PoolUniswap;
use crate::balancer::pool::PoolBalancer;
use crate::contracts::{UNISWAP_ETHEREUM_WETH_USDC, BALANCER_ETHEREUM_BCoW_50WETH_50USDC};
use ethers::prelude::*;
use std::sync::Arc;
use anyhow::Result;

fn compute_optimal_arbitrage(
    x1: f64, // X in Pool 1
    y1: f64, // Y in Pool 1
    x2: f64, // X in Pool 2
    y2: f64, // Y in Pool 2
    exchange_rate_pool_1: f64, // Exchange rate of Pool 1
    exchange_rate_pool_2: f64, // Exchange rate of Pool 2
    gamma1: f64, // fee multiplier in Pool A (e.g. 0.997)
    gamma2: f64, // fee multiplier in Pool B (e.g. 0.9975)
) -> Option<f64> {
    

    // x * y = k
    // γ = 1 - fee
    // exchange rate of pool 1: x1 / y1
    // exchange rate of pool 2: x2 / y2

    // optimize equation based on exchange rate of pool 1 and pool 2
    if exchange_rate_pool_1 < exchange_rate_pool_2 {
        println!("Obtain x out from pool_A");
    
        let numerator = x2 * y1 * gamma2 * gamma1;
        let denominator = y2 * x1;
        if denominator == 0.0 {
            return None;
        }

        let sqrt_term = (numerator / denominator).sqrt();
        let delta = sqrt_term * x1 - (x1 / gamma2);
        if delta > 0.0 {
            return Some(delta);
        }
    } else {
         // Δ = x out from pool_B - x in to pool_A
        // Pool A: (x1 + γ1 * Δx1) * (y1 - Δy1) = x1 * y1
        // Δy1 = (Δx1 * γ1 * y1) / (x1 + Δx1 * γ1)
        // Pool B: (x2 - Δx2) * (y2 + γ2 * Δy2) = x2 * y2
        // Δy1 = Δy2 | because that’s what you're actually depositing into Pool B
        // Δx2 = (Δy1 * γ2 * x2) / (y2 + Δy1 * γ2)

        println!("Obtain x out from pool_B");

        let numerator = x1 * y2 * gamma1 * gamma2;
        let denominator = y1 * x2;
        if denominator == 0.0 {
            return None;
        }
        let sqrt_term = (numerator / denominator).sqrt();
        let delta = sqrt_term * x2 - (x2/ gamma1);
        if delta > 0.0 {
            return Some(delta);
        }
                 
    }

    return None;
}

pub async fn call_arbitrage(provider: Arc<Provider<Http>>) -> Result<()> {
    println!("Arbitraging...");

    // Get Uniswap pool info
    let mut pool_1 = PoolUniswap::new(provider.clone(), UNISWAP_ETHEREUM_WETH_USDC.parse().expect("invalid address"));
    pool_1.get_pool_info().await?;
    println!("\n");

    // Get Balancer pool info
    let mut pool_2 = PoolBalancer::new(provider.clone(), BALANCER_ETHEREUM_BCoW_50WETH_50USDC.parse().expect("invalid address")).await?;
    pool_2.get_pool_info().await?;
    println!("\n");

    let exchage_rate_pool_1 = pool_1.get_info().unwrap().token_0_balance / pool_1.get_info().unwrap().token_1_balance;
    let exchage_rate_pool_2 = pool_2.get_info().unwrap().token_0_balance / pool_2.get_info().unwrap().token_1_balance;

    println!("Exchage rate pool 1: {}", exchage_rate_pool_1);
    println!("Exchage rate pool 2: {}", exchage_rate_pool_2);

    let pool_1_token_0_balance = pool_1.get_info().unwrap().token_0_balance;
    let pool_1_token_1_balance = pool_1.get_info().unwrap().token_1_balance;
    let pool_2_token_0_balance = pool_2.get_info().unwrap().token_0_balance;
    let pool_2_token_1_balance = pool_2.get_info().unwrap().token_1_balance;

    if let Some(arbitrage) = compute_optimal_arbitrage(
        pool_1_token_0_balance,
        pool_1_token_1_balance,
        pool_2_token_0_balance,
        pool_2_token_1_balance,
        exchage_rate_pool_1,
        exchage_rate_pool_2,
        0.997, 
        0.9975
    ) {
        println!("Arbitrage: {:?}", arbitrage);
    } else {
        println!("No arbitrage opportunity found");
    }

    Ok(())
}