use crate::constants::{RPC_URL_ETHEREUM, RPC_URL_SEPOLIA, WS_URL_ETHEREUM, WS_URL_SEPOLIA};

pub struct Chain {
    pub rpc_url: &'static str,
    pub ws_url: &'static str,
    pub chain_id: u64,
}

pub const CHAIN_ETHEREUM: Chain = Chain {
    rpc_url: RPC_URL_ETHEREUM,
    ws_url: WS_URL_ETHEREUM,
    chain_id: 1,
};

pub const CHAIN_SEPOLIA: Chain = Chain {
    rpc_url: RPC_URL_SEPOLIA,
    ws_url: WS_URL_SEPOLIA,
    chain_id: 11155111,
};