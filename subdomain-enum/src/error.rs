use std::{error::Error, fmt::Debug};

#[derive(thiserror::Error)]
pub enum SubdomainError {
    #[error("Subdomain finding request failed.")]
   RequestError(#[from] reqwest::Error),
}

impl Debug for SubdomainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self)?;
        if let Some(source) = self.source() {
            writeln!(f, "Caused by:\n\t{}", source)?;
        }
        Ok(())
    }
}