use anyhow::{bail, Result};
use reqwest::StatusCode;

pub struct ApiClient {
    http: reqwest::Client,
    coinpaprika_base: String,
    api_key: Option<String>,
}

impl ApiClient {
    pub fn new(api_key: Option<String>) -> Self {
        let coinpaprika_base = if api_key.is_some() {
            "https://api-pro.coinpaprika.com/v1".to_string()
        } else {
            "https://api.coinpaprika.com/v1".to_string()
        };

        Self {
            http: reqwest::Client::builder()
                .user_agent(concat!("coinpaprika-cli/", env!("CARGO_PKG_VERSION")))
                .build()
                .expect("failed to build HTTP client"),
            coinpaprika_base,
            api_key,
        }
    }

    pub async fn coinpaprika_get<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        params: &[(&str, &str)],
    ) -> Result<T> {
        let url = format!("{}{}", self.coinpaprika_base, path);
        let mut req = self.http.get(&url);

        if let Some(key) = &self.api_key {
            req = req.header("Authorization", key.as_str());
        }

        if !params.is_empty() {
            req = req.query(params);
        }

        let resp = req.send().await?;
        let status = resp.status();

        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            match status {
                StatusCode::PAYMENT_REQUIRED => {
                    let has_key = self.api_key.is_some();
                    if has_key {
                        bail!(
                            "This endpoint requires a higher-tier plan.\n\n\
                             Check your plan:   coinpaprika-cli key-info\n\
                             Upgrade:           https://coinpaprika.com/api/pricing\n\n\
                             Available plans:\n\
                             \x20 Starter    — Historical data, ticker history, changelog\n\
                             \x20 Business   — All Starter features + ID mappings, priority support\n\
                             \x20 Enterprise — Custom limits, dedicated support"
                        );
                    } else {
                        bail!(
                            "Free tier rate limit reached, or this endpoint requires a paid plan.\n\n\
                             If you just hit the rate limit (20,000 calls/mo), wait and retry.\n\
                             Run coinpaprika-cli plans to see free tier limits.\n\n\
                             To unlock higher limits and all endpoints:\n\
                             \x20 Get your API key:  https://coinpaprika.com/api/pricing\n\
                             \x20 Set your key:      coinpaprika-cli config set-key <YOUR_KEY>"
                        );
                    }
                }
                StatusCode::TOO_MANY_REQUESTS => {
                    bail!("Rate limit exceeded. Wait a moment and try again.");
                }
                StatusCode::FORBIDDEN => {
                    bail!("Invalid API key. Check your key with `coinpaprika-cli config show`");
                }
                StatusCode::NOT_FOUND => {
                    bail!("Not found. Check the ID format (e.g., btc-bitcoin for Bitcoin). API response: {body}");
                }
                s if s.is_server_error() => {
                    bail!("CoinPaprika API is temporarily unavailable. Try again shortly. ({status})");
                }
                _ => {
                    bail!("CoinPaprika API error {status}: {body}");
                }
            }
        }

        Ok(resp.json().await?)
    }
}
