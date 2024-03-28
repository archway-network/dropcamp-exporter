use anyhow::*;
use chrono::prelude::*;
use tendermint::block::{Header, Height};

#[derive(Clone, Debug)]
pub struct Block {
    pub height: Height,
    pub time: DateTime<Utc>,
}

impl TryFrom<Header> for Block {
    type Error = anyhow::Error;

    fn try_from(header: Header) -> Result<Self, Self::Error> {
        let height = header.height;

        let secs = header.time.unix_timestamp();
        let time = DateTime::from_timestamp(secs, 0).ok_or(anyhow!(
            "invalid timestamp in block header: {}",
            header.time
        ))?;

        Ok(Self { height, time })
    }
}
