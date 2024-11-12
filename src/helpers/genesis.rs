use anyhow::{anyhow, Context, Result};
use indy_vdr::pool::PoolTransactions;
use reqwest;
use reqwest::Client;
use std::fs;
use std::path::Path;
use std::time::Duration;
use tokio::time::timeout;

#[derive(Clone)]
pub enum GenesisSource {
    LocalFile(String),
    Url(String),
}

impl GenesisSource {
    pub fn from_str(source: &str) -> Result<Self> {
        if source.starts_with("http://") || source.starts_with("https://") {
            Ok(GenesisSource::Url(source.to_string()))
        } else if Path::new(source).exists() {
            Ok(GenesisSource::LocalFile(source.to_string()))
        } else {
            Err(anyhow!("Source must be a valid URL or existing file path"))
        }
    }

    pub async fn load_transactions(&self) -> anyhow::Result<PoolTransactions> {
        match self {
            GenesisSource::LocalFile(path) => {
                log::debug!("Loading genesis file from path: {}", path);
                PoolTransactions::from_json_file(path)
                    .with_context(|| format!("Failed to load genesis file from path: {}", path))
            }
            GenesisSource::Url(url) => {
                log::debug!("Fetching genesis file from URL: {}", url);

                let client = Client::builder()
                    .timeout(Duration::from_secs(10))
                    .build()
                    .context("Failed to create HTTP client")?;

                // Wrap the request in a timeout
                let response = timeout(
                    Duration::from_secs(15), // Overall operation timeout
                    client.get(url).send(),
                )
                .await
                .with_context(|| "Connection timed out")??;

                log::debug!("Got response with status: {}", response.status());

                if !response.status().is_success() {
                    return Err(anyhow::anyhow!(
                        "Failed to fetch genesis file: HTTP {} - {}",
                        response.status(),
                        if response.status().as_u16() == 404 {
                            "File not found"
                        } else {
                            "Server error"
                        }
                    ));
                }

                // Wrap content reading in a timeout as well
                let content = timeout(
                    Duration::from_secs(5), // Content reading timeout
                    response.text(),
                )
                .await
                .with_context(|| "Timeout while reading response content")??;

                if content.trim().is_empty() {
                    return Err(anyhow::anyhow!("Genesis file is empty"));
                }

                log::debug!("Received content length: {}", content.len());

                PoolTransactions::from_json(&content).with_context(|| {
                    "Failed to parse genesis transactions - invalid format".to_string()
                })
            }
        }
    }
    // Add a separate method to get the content for viewing
    pub async fn get_content(&self) -> anyhow::Result<String> {
        match self {
            GenesisSource::LocalFile(path) => std::fs::read_to_string(path)
                .with_context(|| format!("Failed to read genesis file from path: {}", path)),
            GenesisSource::Url(url) => {
                let client = Client::builder()
                    .timeout(Duration::from_secs(10))
                    .build()
                    .context("Failed to create HTTP client")?;

                let response =
                    client.get(url).send().await.with_context(|| {
                        format!("Failed to fetch genesis file from URL: {}", url)
                    })?;

                response
                    .text()
                    .await
                    .with_context(|| "Failed to read response content")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_genesis_source_from_str() {
        // Test URL
        let url = "https://example.com/genesis.txt";
        match GenesisSource::from_str(url) {
            Ok(GenesisSource::Url(s)) => assert_eq!(s, url),
            _ => panic!("Failed to parse URL"),
        }

        // Test invalid source
        let invalid = "not/a/real/path";
        assert!(GenesisSource::from_str(invalid).is_err());
    }
}
