# 📦 eth-rust

A Rust playground for interacting with the Ethereum blockchain.  The project showcases how to:

* Create and sign transactions with a local wallet
* Query ERC-20 balances and transfer ETH or tokens
* Read decentralised price-feeds via Chainlink
* Inspect on-chain liquidity pools (Uniswap v3 & Balancer)
* Listen to the live pending-transaction stream
* Prototype simple arbitrage logic

All functionality is built on top of the excellent [`ethers-rs`](https://github.com/gakonst/ethers-rs) library.

---

## 🗂  Project structure

```
eth-rust/
├── src/
│   ├── account/          # Wallet, balances & transfers
│   ├── arbitrage/        # Experimental cross-DEX arb logic
│   ├── balancer/         # Balancer-specific helpers
│   ├── block/            # Latest-block utilities
│   ├── chainlink/        # Chainlink price-feeds
│   ├── curve/            # (placeholder) Curve-finance helpers
│   ├── uniswap/          # Uniswap v3 helpers
│   ├── chains.rs         # RPC / WS endpoints per network
│   ├── constants.rs      # Hard-coded keys & endpoints (for demo only!)
│   └── main.rs           # CLI entry-point
└── Cargo.toml
```

---

## Constants File
```
    pub const RPC_URL_ETHEREUM: &str = "http-rpc-ethereum";
    pub const RPC_URL_SEPOLIA: &str = "http-rpc-sepolia";
    pub const WS_URL_ETHEREUM: &str = "websocket-rpc-ethereum";
    pub const WS_URL_SEPOLIA: &str = "websocket-rpc-sepolia";   
    pub const ACCOUNT_PRIVATE_KEY: &str = "your-server-private-key";
    pub const RECIPIENT_ADDRESS: &str = "recipient-address"; 
```

---

## 🚀 Quick start

1. **Install Rust** (stable toolchain)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. **Clone & build**

```bash
git clone <this-repo> && cd eth-rust
cargo build --release
```

3. **(Optional) Configure secrets**

The repo ships with hard-coded RPC endpoints and a *test* private key in `src/constants.rs`.  **Never use real funds with this key!**   Replace the values with your own:

```rust
// src/constants.rs
pub const RPC_URL_ETHEREUM: &str = "https://your-node";
pub const WS_URL_ETHEREUM:  &str = "wss://your-node";
// …
pub const ACCOUNT_PRIVATE_KEY: &str = "<0x…>";
```

Alternatively, wire the constants to environment variables – the code is already using `dotenv`.

4. **Run a demo command**

```bash
# Show the latest Ethereum block number
cargo run -- block

# Fetch the latest Chainlink ETH/USD price
cargo run -- chainlink

# Inspect the WETH/USDC 0.05% Uniswap v3 pool
cargo run -- pool_uniswap

# Show wallet balances (ETH only; see `account_balances` in main.rs)
cargo run -- account_balances
```

The CLI accepts multiple keywords at once, e.g.

```bash
cargo run -- block chainlink account_balances
```

---

## 🛠  Features in detail

• **Wallet / Account (`src/account`)**  
  – Generate a `LocalWallet` from a private-key hex string.  
  – Retrieve ETH & ERC-20 balances (`token_balances.rs`).  
  – Transfer ETH or tokens to another address (`token_transfer.rs`).

• **Chainlink (`src/chainlink`)**  
  – Reads the ETH/USD AggregatorV3 feed (`0x5f4e…8419`).  
  – Provides the latest price, timestamp and description helpers.

• **Uniswap v3 (`src/uniswap`)**  
  – Wraps the pool ABI (via `abigen!`).  
  – Fetches `token0`, `token1`, `fee`, `liquidity`, `slot0` in one go.  
  – Verifies byte-code to ensure the address is indeed a pool.

• **Balancer (`src/balancer`)**  
  – Mirrors the Uniswap helper but for Balancer pools.

• **Streaming / Blocks (`src/block`)**  
  – Get the current block or subscribe (via WebSockets) to pending TXs.

• **Arbitrage (`src/arbitrage`)**  
  – Experimental module combining two pools to spot rate diffs.

---

## 📑 Generating ABIs

Whenever you add a new contract interface, throw its JSON ABI into the corresponding module and use 

```rust
abigen!(MyContract, "./abi/MyContract.json");
```

The generated type-safe bindings Just Work™ with ethers-rs.

---

## 🔒 Security & Disclaimer

*This repository is for educational and experimental purposes only.*  It hard-codes private keys and RPC URLs, does not optimise gas, and performs minimal error-handling.  **Do not** deploy or run it against mainnet with accounts holding real value.

---

## 📜 License

Dual-licensed under either

*Apache License, Version 2.0* or *MIT license* at your option.

See the `LICENSE-*` files for details. 