use ethers::prelude::*;
use std::sync::Arc;
use anyhow::Result;

// UniswapV3Pool ABI fragment based on official Uniswap V3 interfaces
abigen!(
    UniswapV3Pool,
    r#"[
        function token0() external view returns (address)
        function token1() external view returns (address)
        function fee() external view returns (uint24)
        function liquidity() external view returns (uint128)
        function factory() external view returns (address)
    ]"#
);

abigen!(
    IERC20,
    r#"[
        function symbol() external view returns (string)
        function decimals() external view returns (uint8)
        function balanceOf(address) external view returns (uint256)
    ]"#
);

pub struct PoolInfo {
    pub fee: u32,
    pub liquidity: u128,
    pub factory: Address,
    pub token_0_addr: Address,
    pub token_1_addr: Address,
    pub token_0_symbol: String,
    pub token_1_symbol: String,
    pub token_0_balance: f64,
    pub token_1_balance: f64,
}

pub struct PoolUniswap {
    contract: UniswapV3Pool<Provider<Http>>,
    provider: Arc<Provider<Http>>,
    address: Address,
    info: Option<PoolInfo>,
}

impl PoolUniswap {
    pub fn new(provider: Arc<Provider<Http>>, pool_address: Address) -> Self {
        let contract = UniswapV3Pool::new(pool_address, provider.clone());
        Self { 
            contract,
            provider,
            address: pool_address,
            info: None,
        }
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
        println!("Fetching Uniswap V3 Pool Info from: {}", self.address);
        
        // First verify the contract exists
        self.verify_contract().await?;
        
        // Get pool basic info
        let token_0_addr = self.contract.token_0().call().await?;
        let token_1_addr = self.contract.token_1().call().await?;
        let factory = self.contract.factory().call().await?;
        let fee = self.contract.fee().call().await?;
        let liquidity = self.contract.liquidity().call().await?;

        // Create token contracts
        let token_0 = IERC20::new(token_0_addr, self.provider.clone());
        let token_1 = IERC20::new(token_1_addr, self.provider.clone());

        // Get token info separately to avoid borrowing issues
        let symbol_0 = token_0.symbol().call().await?;
        let symbol_1 = token_1.symbol().call().await?;
        
        let decimals_0 = token_0.decimals().call().await?;
        let decimals_1 = token_1.decimals().call().await?;
        
        let balance_0 = token_0.balance_of(self.address).call().await?;
        let balance_1 = token_1.balance_of(self.address).call().await?;

        // Convert U256 to f64 properly
        let human_balance_0 = balance_0.as_u128() as f64 / 10f64.powi(decimals_0 as i32);
        let human_balance_1 = balance_1.as_u128() as f64 / 10f64.powi(decimals_1 as i32);

        // Display pool information
        println!("\n🪣  Uniswap V3 Pool Info:");
        println!("-------------------------------------");
        println!("Fee: {}", fee);
        println!("Liquidity: {}", liquidity);
        println!("Factory: {}", factory);
        println!("Token0: {}", token_0_addr);
        println!("Token1: {}", token_1_addr);
        println!("\n🧠 Uniswap V3 Pool Token Balances:");
        println!("-------------------------------------");
        println!("{}: {:.6}", symbol_0, human_balance_0);
        println!("{}: {:.6}", symbol_1, human_balance_1);

        //return all the values
        let pool_info = PoolInfo {
            fee,
            liquidity,
            factory,
            token_0_addr,
            token_1_addr,
            token_0_symbol: symbol_0,
            token_1_symbol: symbol_1,
            token_0_balance: human_balance_0,
            token_1_balance: human_balance_1,
        };      

        self.info = Some(pool_info);

        Ok(())
    }

    pub fn get_info(&self) -> Option<&PoolInfo> {
        self.info.as_ref()
    }
}

