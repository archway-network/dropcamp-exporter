use std::path::PathBuf;

use config::Config;
use serde::Deserialize;

mod ranking;

pub use ranking::Ranking;
pub use ranking::RANKING_FILE;

pub fn load<'de, T>(path: PathBuf) -> anyhow::Result<T>
where
    T: std::fmt::Debug + Deserialize<'de>,
{
    tracing::debug!(path = %&path.to_string_lossy(), "loading config file");
    let file_source = config::File::from(path);
    let config_loader = Config::builder().add_source(file_source).build()?;
    let config = config_loader.try_deserialize()?;
    tracing::debug!(?config, "config loaded");

    Ok(config)
}
