use ethers::prelude::*;
use ethers::providers::{Provider, Http};
use std::sync::Arc;
use anyhow::Result;

abigen!(
    BalancerPool,
    r#"[
        function getSwapFee() external view returns (uint256)
        function getBalance(address token) external view returns (uint256)
        function getFinalTokens() external view returns (address[] memory)
    ]"#
);

abigen!(
    IERC20,
    r#"[
        function symbol() external view returns (string)
        function decimals() external view returns (uint8)
        function balanceOf(address owner) external view returns (uint256)
    ]"#
);

pub struct PoolInfo {
    pub fee: U256,
    pub token_0_addr: Address,
    pub token_1_addr: Address,
    pub token_0_symbol: String,
    pub token_1_symbol: String,
    pub token_0_balance: f64,
    pub token_1_balance: f64,
}

pub struct PoolBalancer {
    contract: BalancerPool<Provider<Http>>,
    provider: Arc<Provider<Http>>,
    address: Address,
    tokens: Vec<Address>,
    info: Option<PoolInfo>,
}

impl PoolBalancer {
    // modify this to supply the pool address in args
    pub async fn new(provider: Arc<Provider<Http>>, pool_address: Address) -> Result<Self> {
        let contract = BalancerPool::new(pool_address, provider.clone());

        let tokens = contract.get_final_tokens().call().await?;

        Ok(Self { 
            contract,
            provider,
            address: pool_address,
            tokens,
            info: None,
        })
    }

    pub async fn verify_contract(&self) -> Result<()> {
        println!("\nVerifying contract at address: {:?}", self.address);
        
        // Check if contract exists by getting its code
        match self.provider.get_code(self.address, None).await {
            Ok(code) => {
                if code.is_empty() {
                    return Err(anyhow::anyhow!("No contract code found at address {:?}", self.address));
                }
                println!("Contract code found. Code size: {} bytes", code.len());
            }
            Err(e) => {
                return Err(anyhow::anyhow!("Failed to get contract code: {}", e));
            }
        }
        Ok(())
    }

    pub async fn get_pool_info(&mut self) -> Result<()> {        
        // First verify the contract exists
        self.verify_contract().await?;

        // Get swap fee
        let swap_fee = self.contract.get_swap_fee().call().await?;

        let mut pool_info = PoolInfo {
            fee: swap_fee,
            token_0_addr: self.tokens[0],
            token_1_addr: self.tokens[1],
            token_0_symbol: String::new(),
            token_1_symbol: String::new(),
            token_0_balance: 0.0,
            token_1_balance: 0.0,
        };

        for (index, token) in self.tokens.iter().enumerate() {
            let token_contract = IERC20::new(*token, self.provider.clone());
            let token_symbol = token_contract.symbol().call().await?;
            let token_decimals = token_contract.decimals().call().await?;   
            let balance = self.contract.get_balance(*token).call().await?;
            let human_balance = balance.as_u128() as f64 / 10f64.powi(token_decimals as i32);

            // Modify this to support more than 2 tokens
            if index == 0 && self.tokens.len() == 2 {
                pool_info.token_0_addr = *token;
                pool_info.token_0_symbol = token_symbol;
                pool_info.token_0_balance = human_balance;
            } else if index == 1 && self.tokens.len() == 2 {
                pool_info.token_1_symbol = token_symbol;
                pool_info.token_1_balance = human_balance;
            }
        }
        
         // Display pool information
         println!("\n🪣  Balancer Pool Info:");
         println!("-------------------------------------");
         println!("Fee: {}", pool_info.fee);
         println!("{}: {:.6}", pool_info.token_0_symbol, pool_info.token_0_balance);
         println!("{}: {:.6}", pool_info.token_1_symbol, pool_info.token_1_balance);

        self.info = Some(pool_info);

        Ok(())
    }

    pub fn get_info(&self) -> Option<&PoolInfo> {
        self.info.as_ref()
    }
}