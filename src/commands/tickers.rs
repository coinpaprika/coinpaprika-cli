use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::client::ApiClient;
use crate::output::OutputFormat;

#[derive(Debug, Deserialize, Serialize)]
pub struct Ticker {
    pub id: String,
    pub name: String,
    pub symbol: String,
    pub rank: Option<i64>,
    pub circulating_supply: Option<f64>,
    pub total_supply: Option<f64>,
    pub max_supply: Option<f64>,
    pub beta_value: Option<f64>,
    pub first_data_at: Option<String>,
    pub last_updated: Option<String>,
    pub quotes: Option<HashMap<String, TickerQuote>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TickerQuote {
    pub price: Option<f64>,
    pub volume_24h: Option<f64>,
    pub volume_24h_change_24h: Option<f64>,
    pub market_cap: Option<f64>,
    pub market_cap_change_24h: Option<f64>,
    pub percent_change_15m: Option<f64>,
    pub percent_change_30m: Option<f64>,
    pub percent_change_1h: Option<f64>,
    pub percent_change_6h: Option<f64>,
    pub percent_change_12h: Option<f64>,
    pub percent_change_24h: Option<f64>,
    pub percent_change_7d: Option<f64>,
    pub percent_change_30d: Option<f64>,
    pub percent_change_1y: Option<f64>,
    pub ath_price: Option<f64>,
    pub ath_date: Option<String>,
    pub percent_from_price_ath: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TickerHistoryPoint {
    pub timestamp: Option<String>,
    pub price: Option<f64>,
    pub volume_24h: Option<f64>,
    pub market_cap: Option<f64>,
}

pub async fn execute_list(client: &ApiClient, limit: usize, quotes: &str, output: OutputFormat, raw: bool) -> Result<()> {
    let tickers: Vec<Ticker> = client.coinpaprika_get(
        "/tickers",
        &[("quotes", quotes), ("limit", &limit.to_string())],
    ).await?;
    match output {
        OutputFormat::Table => crate::output::tickers::print_tickers_table(&tickers),
        OutputFormat::Json => {
            crate::output::print_json_wrapped(&tickers, crate::output::ResponseMeta::coinpaprika("/tickers"), raw)?;
        }
    }
    Ok(())
}

pub async fn execute_detail(client: &ApiClient, coin_id: &str, quotes: &str, output: OutputFormat, raw: bool) -> Result<()> {
    let id = crate::idpath::path_segment(coin_id)?;
    let ticker: Ticker = client.coinpaprika_get(
        &format!("/tickers/{id}"),
        &[("quotes", quotes)],
    ).await?;
    match output {
        OutputFormat::Table => crate::output::tickers::print_ticker_detail(&ticker),
        OutputFormat::Json => {
            crate::output::print_json_wrapped(&ticker, crate::output::ResponseMeta::coinpaprika(&format!("/coin/{coin_id}")), raw)?;
        }
    }
    Ok(())
}

pub async fn execute_history(
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
    let history: Vec<TickerHistoryPoint> = client.coinpaprika_get(
        &format!("/tickers/{id}/historical"),
        &params,
    ).await?;
    match output {
        OutputFormat::Table => crate::output::tickers::print_history_table(&history),
        OutputFormat::Json => {
            crate::output::print_json_wrapped(&history, crate::output::ResponseMeta::coinpaprika(&format!("/coin/{coin_id}")), raw)?;
        }
    }
    Ok(())
}
