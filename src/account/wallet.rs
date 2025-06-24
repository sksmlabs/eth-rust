// TODO: TO BE REMOVED SINCE NO LONGER USED

use ethers::prelude::*;
use anyhow::Result;

#[derive(Debug)]
pub struct Wallet {
    pub address: Address,
    pub private_key: LocalWallet,
}

impl Clone for Wallet {
    fn clone(&self) -> Self {
        // Clone by recreating from the private key bytes
        let private_key_bytes = self.private_key.signer().to_bytes();
        let private_key = LocalWallet::from_bytes(&private_key_bytes).unwrap();
        
        Self {
            address: self.address,
            private_key,
        }
    }
}

impl Wallet {
    /// Creates a new wallet from a private key
    pub fn from_private_key(private_key: &str) -> Result<Self> {
        let private_key = private_key.trim_start_matches("0x");
        let private_key = hex::decode(private_key)?;
        let private_key = LocalWallet::from_bytes(&private_key)?;
        let address = private_key.address();

        Ok(Self {
            address,
            private_key,
        })
    }

    /// Returns the wallet's address as a string
    pub fn address_str(&self) -> String {
        format!("{:?}", self.address)
    }

    /// Returns the wallet's private key as a hex string
    pub fn private_key_str(&self) -> String {
        format!("0x{}", hex::encode(self.private_key.signer().to_bytes()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_private_key() {
        // Test with a known private key
        let private_key = "0000000000000000000000000000000000000000000000000000000000000001";
        let wallet = Wallet::from_private_key(private_key).unwrap();
        assert_eq!(wallet.address_str(), "0x7E5F4552091A69125d5DfCb7b8C2659029395Bdf");
    }
} 