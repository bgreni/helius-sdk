use serde::{Deserialize, Serialize};
use reqwest;
use crate::common::dataclass;
use crate::Helius;

pub trait BalancesApi {
    fn balances(&self, address: &str) -> reqwest::Result<BalancesResponse>;
}

impl BalancesApi for Helius {
    fn balances(&self, address: &str) -> reqwest::Result<BalancesResponse> {
        let method = format!("addresses/{address}/balances");
        return self.http_client
            .get(self.get_url_v0(method.as_str()))
            .send()?.error_for_status()?
            .json::<BalancesResponse>();
    }
}

dataclass! {
    #[serde(rename_all="camelCase")]
    pub struct BalancesResponse {
        tokens: Vec<TokenData>,
        native_balance: u64
    }
}


dataclass! {
    #[serde(rename_all="camelCase")]
    pub struct TokenData {
        mint: String,
        amount: u64,
        decimals: u32,
        token_account: String
    }
}