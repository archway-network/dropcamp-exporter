use std::str::FromStr;

use anyhow::{anyhow, Result};
use bigdecimal::{BigDecimal, ToPrimitive};
use num::BigUint;

#[derive(Clone, Debug)]
pub struct Coin {
    pub denom: String,
    pub amount: BigDecimal,
    pub coingecko_id: Option<String>,
}

impl Coin {
    pub fn build(
        denom: String,
        amount: String,
        decimals: u8,
        coingecko_id: Option<String>,
    ) -> Result<Self> {
        let amount = BigUint::from_str(amount.as_str())?;

        Ok(Self {
            denom,
            amount: BigDecimal::new(amount.into(), decimals.into()),
            coingecko_id,
        })
    }

    pub fn total_value<T>(&self, price: T) -> Result<f64>
    where
        T: TryInto<BigDecimal>,
        anyhow::Error: From<T::Error>,
    {
        let total = &self.amount * price.try_into()?;
        total
            .to_f64()
            .ok_or(anyhow!("failed to convert value to f64: {}", total))
    }

    pub fn with_scale(&self, scale: i64) -> Self {
        Self {
            denom: self.denom.clone(),
            amount: self.amount.with_scale(scale),
            coingecko_id: self.coingecko_id.clone(),
        }
    }
}

impl std::fmt::Display for Coin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}{}", self.amount, self.denom))
    }
}
