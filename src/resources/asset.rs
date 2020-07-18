use crate::resources::account::AccountFlags;
use crate::resources::{Asset, Link};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AssetStat {
    #[serde(rename = "_links")]
    pub links: AssetStatLinks,
    #[serde(flatten)]
    pub asset: Asset,
    pub paging_token: String,
    pub amount: String,
    pub num_accounts: i32,
    pub flags: AccountFlags,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AssetStatLinks {
    pub toml: Link,
}
