use crate::link::Link;
use crate::resources::account::AccountFlags;
use crate::resources::Asset;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AssetStat {
    #[serde(rename = "_links")]
    pub links: AssetStatLinks,
    #[serde(flatten)]
    pub asset: Asset,
    pub paging_token: String,
    pub num_accounts: i32,
    pub num_claimable_balances: i32,
    pub num_liquidity_pools: i32,
    pub amount: String,
    pub accounts: AssetStatAccounts,
    pub claimable_balances_amount: String,
    pub liquidity_pools_amount: String,
    pub balances: AssetStatBalances,
    pub flags: AccountFlags,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AssetStatLinks {
    pub toml: Link,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AssetStatAccounts {
    pub authorized: i32,
    pub authorized_to_maintain_liabilities: i32,
    pub unauthorized: i32,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AssetStatBalances {
    pub authorized: String,
    pub authorized_to_maintain_liabilities: String,
    pub unauthorized: String,
}
