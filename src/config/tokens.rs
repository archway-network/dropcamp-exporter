use std::collections::HashMap;

use anyhow::Result;
use serde::Deserialize;

use crate::coin::Coin;

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct TokenInfo {
    pub denom: String,
    pub decimals: u8,
    pub coingecko_id: Option<String>,
}

impl TokenInfo {
    pub fn create_coin(&self, coin: cosmos_sdk_proto::cosmos::base::v1beta1::Coin) -> Result<Coin> {
        Coin::build(
            self.denom.clone(),
            coin.amount,
            self.decimals,
            self.coingecko_id.clone(),
        )
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct TokenMap(HashMap<String, TokenInfo>);

impl TokenMap {
    pub fn get(&self, denom: &String) -> Option<&TokenInfo> {
        self.0.get(denom)
    }

    pub fn create_coin(
        &self,
        coin: cosmos_sdk_proto::cosmos::base::v1beta1::Coin,
    ) -> Result<Option<Coin>> {
        self.get(&coin.denom)
            .map(|info| info.create_coin(coin))
            .transpose()
    }
}
