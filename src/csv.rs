use std::marker::PhantomData;
use std::path::PathBuf;

use anyhow::Result;

use tokio::fs::File;
use tokio::io::{self, AsyncWriteExt, BufWriter};
use tokio::sync::mpsc;

const DEFAULT_BUFFER_SIZE: usize = 1024;
const SEPARATOR: &str = ";";

pub type Header = Vec<&'static str>;
pub type Row = Vec<String>;

pub trait Item {
    fn header() -> Header;

    fn rows(self) -> Vec<Row>;
}

pub struct Writer<T: Item> {
    tx: mpsc::Sender<Row>,
    _phantom: PhantomData<T>,
}

impl<T: Item> Writer<T> {
    pub async fn create(path: PathBuf) -> Result<Self> {
        let writer = Self {
            tx: writer_channel(path).await?,
            _phantom: PhantomData,
        };
        writer.write_header().await?;

        Ok(writer)
    }

    async fn write_header(&self) -> Result<()> {
        let header = T::header().iter().map(|s| s.to_string()).collect();
        self.tx.send(header).await?;
        Ok(())
    }

    pub async fn write(&self, item: T) -> Result<()> {
        for row in item.rows() {
            self.tx.send(row).await?;
        }
        Ok(())
    }
}

async fn writer_channel(path: PathBuf) -> Result<mpsc::Sender<Row>> {
    let (tx, mut rx) = mpsc::channel::<Vec<String>>(DEFAULT_BUFFER_SIZE);
    let file = File::create(path).await?;

    tokio::spawn(async move {
        let mut writer = BufWriter::new(file);
        while let Some(row) = rx.recv().await {
            let buffer = row.join(SEPARATOR);
            tracing::trace!(?row, "writing row to file");
            writer.write_all(buffer.as_bytes()).await?;
            writer.write_all(b"\n").await?;
            writer.flush().await?;
        }

        writer.shutdown().await?;

        Ok::<_, io::Error>(())
    });

    Ok(tx)
}
