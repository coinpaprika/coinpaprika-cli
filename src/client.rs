use anyhow::{bail, Result};
use reqwest::StatusCode;

pub struct ApiClient {
    http: reqwest::Client,
    coinpaprika_base: String,
    api_key: Option<String>,
}

/// Build the full request URL from `base` and `path` and verify that
/// parsing it did not rewrite the path we asked for.
///
/// `path` is expected to already consist of literal route segments and
/// values validated by `idpath::path_segment`. Parsing can still legally
/// disagree with that expectation (a parser bug, a future refactor that
/// forgets to validate an argument, or a value that validation missed) and
/// when it does, `url` would send the request to a path other than the one
/// the caller intended, carrying the Authorization header with it. Refuse
/// in that case instead of guessing.
fn build_verified_url(base: &str, path: &str) -> Result<reqwest::Url> {
    let url = format!("{base}{path}");
    let parsed = reqwest::Url::parse(&url)
        .map_err(|e| anyhow::anyhow!("built an invalid request URL: {e}"))?;
    let base_path = reqwest::Url::parse(base)
        .map(|u| u.path().to_string())
        .unwrap_or_default();
    let expected_path = format!("{base_path}{path}");
    if parsed.path() != expected_path {
        bail!(
            "refusing to send request: the URL path changed from {expected_path} to {} while building it",
            parsed.path()
        );
    }
    Ok(parsed)
}

impl ApiClient {
    pub fn new(api_key: Option<String>) -> Self {
        let coinpaprika_base = if api_key.is_some() {
            "https://api-pro.coinpaprika.com/v1".to_string()
        } else {
            "https://api.coinpaprika.com/v1".to_string()
        };

        let ua = format!(
            "coinpaprika-cli/{} ({}/{})",
            env!("CARGO_PKG_VERSION"),
            std::env::consts::OS,
            std::env::consts::ARCH,
        );
        Self {
            http: reqwest::Client::builder()
                .user_agent(&ua)
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
        let verified_url = build_verified_url(&self.coinpaprika_base, path)?;
        let mut req = self.http.get(verified_url);

        if let Some(key) = &self.api_key {
            req = req.header("Authorization", key.as_str());
        }

        if !params.is_empty() {
            req = req.query(params);
        }

        let resp = req.send().await?;
        let status = resp.status();

        if !status.is_success() {
            let body = crate::output::sanitize(&resp.text().await.unwrap_or_default());
            match status {
                StatusCode::PAYMENT_REQUIRED => {
                    let has_key = self.api_key.is_some();
                    if has_key {
                        bail!(
                            "This endpoint requires a higher-tier plan.\n\n\
                             Check your plan:   coinpaprika-cli key-info\n\
                             Upgrade:           https://coinpaprika.com/api/pricing\n\n\
                             Available plans:\n\
                             \x20 Starter    : Historical data, ticker history, changelog\n\
                             \x20 Business   : All Starter features + ID mappings, priority support\n\
                             \x20 Enterprise : Custom limits, dedicated support"
                        );
                    } else {
                        bail!(
                            "Free tier rate limit reached, or this endpoint requires a paid plan.\n\n\
                             If you just hit the rate limit (20,000 calls/mo), wait and retry.\n\
                             Run coinpaprika-cli plans to see free tier limits.\n\n\
                             To unlock higher limits and all endpoints:\n\
                             \x20 Get your API key:  https://coinpaprika.com/api/pricing\n\
                             \x20 Set your key:      coinpaprika-cli onboard\n\
                             \x20 (or set the COINPAPRIKA_API_KEY environment variable)"
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

#[cfg(test)]
mod tests {
    use super::*;

    const BASE: &str = "https://api-pro.coinpaprika.com/v1";

    #[test]
    fn accepts_a_clean_validated_path() {
        let url = build_verified_url(BASE, "/coins/btc-bitcoin/events").unwrap();
        assert_eq!(url.path(), "/v1/coins/btc-bitcoin/events");
    }

    #[test]
    fn rejects_a_path_whose_dot_segments_the_url_parser_would_collapse() {
        // This is the exact shape the old code built directly from an
        // unvalidated id via format!("/coins/{coin_id}"): the url crate
        // normalizes ".." segments while parsing, before any request is
        // sent, moving the request to a different path than the one the
        // command intended.
        let moved_by_dot_segments = "/coins/../../global";
        let result = build_verified_url(BASE, moved_by_dot_segments);
        assert!(result.is_err(), "expected the rewritten path to be rejected");
    }

    #[test]
    fn rejects_a_path_with_a_bare_query_delimiter() {
        // An id containing an unencoded '?' would inject query parameters
        // ahead of the CLI's own ones; idpath::path_segment rejects this,
        // but prove the independent guard also catches it.
        let path_with_injected_query = "/tickers/btc-bitcoin?quotes=EVIL";
        let result = build_verified_url(BASE, path_with_injected_query);
        // '?' is a valid URL delimiter, so url::Url::parse succeeds and
        // reparses it as a query string; the path it reports therefore no
        // longer equals the literal path string we asked for.
        assert!(result.is_err());
    }
}
