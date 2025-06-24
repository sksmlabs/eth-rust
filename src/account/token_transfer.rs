use ethers::prelude::*;
use ethers::utils::parse_units;
use ethers::types::transaction::eip2718::TypedTransaction;
use std::sync::Arc;
use eyre::Result;

abigen!(
    ERC20,
    r#"[
        function transfer(address to, uint256 value) external returns (bool)
        function approve(address spender, uint256 value) external returns (bool)
        function transferFrom(address from, address to, uint256 value) external returns (bool)
        function allowance(address owner, address spender) external view returns (uint256)
        function decimals() external view returns (uint8)
    ]"#
);

// Approve Spender
async fn approve_spender<M: Middleware + 'static>(
    client: Arc<M>,
    token_address: Address,
    owner: Address,
    spender: Address,
) -> Result<()> {
    let token = ERC20::new(token_address, client.clone());
    let decimals = token.decimals().call().await?;
    let amount: U256 = parse_units("10", decimals as usize)?.into();

    println!("\n🛂 Approving spender...");
    let contract_call = token.approve(spender, amount);
    let pending_tx = contract_call.send().await?;
    let receipt = pending_tx.await?.expect("Approval failed");
    println!("✅ Approval complete: {:?}", receipt.transaction_hash);

    let allowance = token.allowance(owner, spender).call().await?;
    println!("🔍 Allowance for spender: {}", allowance);
    Ok(())
}

// Transfer Tokens Directly
pub async fn transfer_tokens<M: Middleware + 'static>(
    client: Arc<M>,
    token_address: Address,
    recipient: Address,
) -> Result<()> {
    let token = ERC20::new(token_address, client.clone());
    let provider = client.provider();

    let decimals = token.decimals().call().await?;
    let amount: U256 = parse_units("10", decimals as usize)?.into();

    println!("\n🔁 Estimating gas for token transfer...");

    // Create contract call and get transaction request
    let contract_call = token.transfer(recipient, amount);
    let tx_request = contract_call.tx.clone();

    // Estimate gas
    let gas_estimate = provider.estimate_gas(&tx_request, None).await?;
    let gas_price = provider.get_gas_price().await?;
    let gas_fee = gas_estimate * gas_price;

    // Show estimate
    println!("⛽ Estimated gas: {}", gas_estimate);
    println!("💰 Gas price: {} wei", gas_price);
    println!("🔢 Estimated total gas fee: {} ETH", ethers::utils::format_units(gas_fee, 18)?);

    println!("✅ Token transfer estimation complete.");
    println!("🚀 Ready to send transaction (commented out for safety)");
    
    // Uncomment to actually send the transaction
    // let pending_tx = contract_call.send().await?;
    // let receipt = pending_tx.await?.expect("Transaction failed");
    // println!("✅ Transfer complete: {:?}", receipt.transaction_hash);

    Ok(())
}

// Transfer Tokens From Another User (After Approval)
async fn transfer_tokens_from<M: Middleware + 'static>(
    client: Arc<M>,
    token_address: Address,
    sender: Address,
    recipient: Address,
) -> eyre::Result<()> {
    let token = ERC20::new(token_address, client.clone());
    let provider = client.provider();

    // Set amount
    let decimals = token.decimals().call().await?;
    let amount: U256 = parse_units("10", decimals as usize)?.into();

    println!("\n🔍 Preparing transferFrom transaction...");

    // Step 1: Build ContractCall
    let contract_call = token.transfer_from(sender, recipient, amount);

    // Step 2: Get raw transaction request
    let tx_request = contract_call.tx.clone();

    // Step 3: Estimate gas
    let gas_estimate = provider.estimate_gas(&tx_request, None).await?;
    let gas_price = provider.get_gas_price().await?;
    let gas_fee = gas_estimate * gas_price;

    println!("⛽ Estimated gas: {}", gas_estimate);
    println!("💰 Gas price: {} wei", gas_price);
    println!("🔢 Estimated fee: {} ETH", ethers::utils::format_units(gas_fee, 18)?);

    println!("✅ transferFrom estimation complete.");
    println!("🚀 Ready to send transaction (commented out for safety)");

    // Uncomment to actually send the transaction
    // let pending_tx = contract_call.send().await?;
    // let receipt = pending_tx.await?.expect("Transfer failed");
    // println!("✅ transferFrom successful: {:?}", receipt.transaction_hash);

    Ok(())
}

// Public wrapper for ETH transfer
pub async fn transfer_eth<M: Middleware + 'static>(
    client: Arc<M>,
    from: Address,
    to: Address,
    amount: U256,
) -> Result<()> {
    let provider = client.provider();
    
    println!("\n💸 Preparing ETH transfer...");
    println!("From: {:?}", from);
    println!("To: {:?}", to);
    println!("Amount: {} ETH", ethers::utils::format_units(amount, 18)?);
    
    // Create transaction request using TypedTransaction
    let tx = Eip1559TransactionRequest::new()
        .to(to)
        .value(amount)
        .from(from);
    
    let typed_tx: TypedTransaction = tx.into();
    
    // Estimate gas
    let gas_estimate = provider.estimate_gas(&typed_tx, None).await?;
    let gas_price = provider.get_gas_price().await?;
    let gas_fee = gas_estimate * gas_price;
    
    println!("⛽ Estimated gas: {}", gas_estimate);
    println!("💰 Gas price: {} wei", gas_price);
    println!("🔢 Estimated total gas fee: {} ETH", ethers::utils::format_units(gas_fee, 18)?);
    
    // Check balance
    let balance = provider.get_balance(from, None).await?;
    let total_needed = amount + gas_fee;
    
    if balance < total_needed {
        println!("❌ Insufficient ETH! Need {}, have {}",
            ethers::utils::format_units(total_needed, 18)?,
            ethers::utils::format_units(balance, 18)?
        );
        return Ok(());
    }
    
    println!("✅ Sufficient ETH for transfer and gas.");

    // Uncomment to actually not send the transaction
    println!("🚀 Ready to send transaction (commented out for safety)");
    
    // Uncomment to actually send the transaction
    // let pending_tx = client.send_transaction(typed_tx, None).await?;
    // let receipt = pending_tx.await?.expect("Transaction failed");
    // println!("✅ Transfer complete: {:?}", receipt.transaction_hash);
    
    Ok(())
}

