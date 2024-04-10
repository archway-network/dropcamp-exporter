use std::path::PathBuf;

use config::Config;
use serde::Deserialize;

mod ranking;
mod tokens;

pub use ranking::Ranking;
pub use tokens::TokenMap;

pub trait ConfigLoader<'de>: std::fmt::Debug + Deserialize<'de> + Sized {
    fn load(path: PathBuf) -> anyhow::Result<Self> {
        tracing::debug!(path = %&path.to_string_lossy(), "loading config file");
        let file_source = config::File::from(path);
        let config_loader = Config::builder().add_source(file_source).build()?;
        let config = config_loader.try_deserialize()?;
        tracing::debug!(?config, "config loaded");

        Ok(config)
    }
}
