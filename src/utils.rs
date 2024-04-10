use std::str::FromStr;

use anyhow::Result;
use bigdecimal::{num_bigint::BigUint, BigDecimal};

pub fn to_bigdecimal(amount: &str) -> Result<BigDecimal> {
    let amount = BigUint::from_str(amount)?;
    Ok(BigDecimal::new(amount.into(), 18))
}
