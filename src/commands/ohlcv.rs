use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::client::ApiClient;
use crate::output::OutputFormat;

#[derive(Debug, Deserialize, Serialize)]
pub struct OhlcvData {
    pub time_open: Option<String>,
    pub time_close: Option<String>,
    pub open: Option<f64>,
    pub high: Option<f64>,
    pub low: Option<f64>,
    pub close: Option<f64>,
    pub volume: Option<f64>,
    pub market_cap: Option<f64>,
}

pub async fn execute_historical(
    client: &ApiClient,
    coin_id: &str,
    start: &str,
    end: Option<&str>,
    interval: &str,
    limit: usize,
    quote: &str,
    output: OutputFormat,
    raw: bool,
) -> Result<()> {
    let limit_str = limit.to_string();
    let mut params: Vec<(&str, &str)> = vec![
        ("start", start),
        ("interval", interval),
        ("limit", &limit_str),
        ("quote", quote),
    ];
    if let Some(e) = end {
        params.push(("end", e));
    }

    let id = crate::idpath::path_segment(coin_id)?;
    let data: Vec<OhlcvData> = client.coinpaprika_get(
        &format!("/coins/{id}/ohlcv/historical"),
        &params,
    ).await?;
    match output {
        OutputFormat::Table => crate::output::ohlcv::print_ohlcv_table(&data),
        OutputFormat::Json => {
            crate::output::print_json_wrapped(&data, crate::output::ResponseMeta::coinpaprika(&format!("/coin/{coin_id}")), raw)?;
        }
    }
    Ok(())
}

pub async fn execute_latest(client: &ApiClient, coin_id: &str, quote: &str, output: OutputFormat, raw: bool) -> Result<()> {
    let id = crate::idpath::path_segment(coin_id)?;
    let data: Vec<OhlcvData> = client.coinpaprika_get(
        &format!("/coins/{id}/ohlcv/latest"),
        &[("quote", quote)],
    ).await?;
    match output {
        OutputFormat::Table => crate::output::ohlcv::print_ohlcv_table(&data),
        OutputFormat::Json => {
            crate::output::print_json_wrapped(&data, crate::output::ResponseMeta::coinpaprika(&format!("/coin/{coin_id}")), raw)?;
        }
    }
    Ok(())
}

pub async fn execute_today(client: &ApiClient, coin_id: &str, quote: &str, output: OutputFormat, raw: bool) -> Result<()> {
    let id = crate::idpath::path_segment(coin_id)?;
    let data: Vec<OhlcvData> = client.coinpaprika_get(
        &format!("/coins/{id}/ohlcv/today"),
        &[("quote", quote)],
    ).await?;
    match output {
        OutputFormat::Table => crate::output::ohlcv::print_ohlcv_table(&data),
        OutputFormat::Json => {
            crate::output::print_json_wrapped(&data, crate::output::ResponseMeta::coinpaprika(&format!("/coin/{coin_id}")), raw)?;
        }
    }
    Ok(())
}
