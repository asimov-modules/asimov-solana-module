// This is free and unencumbered software released into the public domain.

#![allow(unused)]

use asimov_module::prelude::{FromStr, String};
use core::error::Error;

#[derive(Clone, Debug, Default)]
pub struct BlockUrl {
    pub network: String,
    pub slot: u64,
}

impl FromStr for BlockUrl {
    type Err = url::ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        url::Url::parse(input).map(|url| {
            let network = url.host_str().unwrap_or("mainnet-beta").to_owned();
            let path = url.path().strip_prefix('/').expect("should contain a path");
            let slot: u64 = path.parse().expect("should be a valid slot number");
            Self { network, slot }
        })
    }
}

#[cfg(feature = "std")]
impl BlockUrl {
    pub fn fetch(&self) -> Result<String, Box<dyn Error>> {
        let request_url = format!("https://api.{}.solana.com", self.network);
        let body = format!(
            r#"{{"jsonrpc":"2.0","id":1,"method":"getBlock","params":[{slot},{opts}]}}"#,
            slot = self.slot,
            opts = r#"{"encoding":"json","maxSupportedTransactionVersion":0,"transactionDetails":"full","rewards":false}"#,
        );
        let mut response = ureq::post(&request_url)
            .header("Content-Type", "application/json")
            .send(body.as_str())?;
        let response_body = response.body_mut().read_to_string()?;
        Ok(response_body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_mainnet_url() {
        let block: BlockUrl = "solana://mainnet-beta/100000000".parse().unwrap();
        assert_eq!(block.network, "mainnet-beta");
        assert_eq!(block.slot, 100_000_000);
    }

    #[test]
    fn parse_devnet_url() {
        let block: BlockUrl = "solana://devnet/42".parse().unwrap();
        assert_eq!(block.network, "devnet");
        assert_eq!(block.slot, 42);
    }

    #[test]
    fn parse_testnet_url() {
        let block: BlockUrl = "solana://testnet/1".parse().unwrap();
        assert_eq!(block.network, "testnet");
        assert_eq!(block.slot, 1);
    }

    #[test]
    fn parse_invalid_url_returns_error() {
        assert!("not-a-url".parse::<BlockUrl>().is_err());
    }

    #[test]
    fn default_block_url() {
        let block = BlockUrl::default();
        assert_eq!(block.network, "");
        assert_eq!(block.slot, 0);
    }

    #[test]
    fn rpc_url_format() {
        let block: BlockUrl = "solana://mainnet-beta/1".parse().unwrap();
        assert_eq!(
            format!("https://api.{}.solana.com", block.network),
            "https://api.mainnet-beta.solana.com"
        );

        let block: BlockUrl = "solana://devnet/1".parse().unwrap();
        assert_eq!(
            format!("https://api.{}.solana.com", block.network),
            "https://api.devnet.solana.com"
        );
    }
}
