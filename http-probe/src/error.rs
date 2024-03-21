use std::{error::Error, fmt::Debug};

#[derive(thiserror::Error)]
pub enum HttpProbeError {
    #[error("Http request failed.")]
    RequestError(#[source] reqwest::Error),
    #[error("Reqwest client failure.")]
    ClientError(#[source] reqwest::Error),
    #[error("Failed to read file.")]
    FileError(#[source] std::io::Error),
}

impl Debug for HttpProbeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self)?;
        if let Some(source) = self.source() {
            writeln!(f, "Caused by:\n\t{}", source)?;
        }
        Ok(())
    }
}