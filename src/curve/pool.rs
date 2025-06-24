use ethers::prelude::*;
use ethers::providers::{Provider, Http};
use std::sync::Arc;

abigen!(
    CurvePool,
    r#"[
        function coins(uint256 i) external view returns (address)
        function balances(uint256 i) external view returns (uint256)
        function A() external view returns (uint256)
    ]"#
);

pub struct PoolCurve {
    contract: CurvePool<Provider<Http>>,
    provider: Arc<Provider<Http>>,
    address: Address,
}

impl PoolCurve {
    pub fn new(provider: Arc<Provider<Http>>, pool_address: Address) -> Self {
        let contract = CurvePool::new(pool_address, provider.clone());
        Self { 
            contract,
            provider,
            address: pool_address,
        }
    }
}