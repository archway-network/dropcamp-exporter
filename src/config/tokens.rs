use std::collections::HashMap;

use serde::Deserialize;

use super::ConfigLoader;

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct TokenInfo {
    pub denom: String,
    pub decimals: u8,
    pub coingecko_id: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct TokenMap(HashMap<String, TokenInfo>);

impl<'de> ConfigLoader<'de> for TokenMap {}
