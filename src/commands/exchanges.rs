use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::client::ApiClient;
use crate::output::OutputFormat;

#[derive(Debug, Deserialize, Serialize)]
pub struct Exchange {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub active: Option<bool>,
    pub website_status: Option<bool>,
    pub api_status: Option<bool>,
    pub message: Option<String>,
    pub links: Option<HashMap<String, serde_json::Value>>,
    pub markets_data_fetched: Option<bool>,
    pub adjusted_rank: Option<i64>,
    pub reported_rank: Option<i64>,
    pub currencies: Option<i64>,
    pub markets: Option<i64>,
    pub fiats: Option<Vec<serde_json::Value>>,
    pub quotes: Option<HashMap<String, ExchangeQuote>>,
    pub last_updated: Option<String>,
    pub confidence_score: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ExchangeQuote {
    pub reported_volume_24h: Option<f64>,
    pub adjusted_volume_24h: Option<f64>,
    pub reported_volume_7d: Option<f64>,
    pub adjusted_volume_7d: Option<f64>,
    pub reported_volume_30d: Option<f64>,
    pub adjusted_volume_30d: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ExchangeMarket {
    pub pair: Option<String>,
    pub base_currency_id: Option<String>,
    pub base_currency_name: Option<String>,
    pub quote_currency_id: Option<String>,
    pub quote_currency_name: Option<String>,
    pub market_url: Option<String>,
    pub category: Option<String>,
    pub fee_type: Option<String>,
    pub outlier: Option<bool>,
    pub reported_volume_24h_share: Option<f64>,
    pub quotes: Option<HashMap<String, ExchangeMarketQuote>>,
    pub trust_score: Option<String>,
    pub last_updated: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ExchangeMarketQuote {
    pub price: Option<f64>,
    pub volume_24h: Option<f64>,
}

pub async fn execute_list(client: &ApiClient, limit: usize, quotes: &str, output: OutputFormat, raw: bool) -> Result<()> {
    let mut exchanges: Vec<Exchange> = client.coinpaprika_get(
        "/exchanges",
        &[("quotes", quotes)],
    ).await?;
    exchanges.sort_by_key(|e| e.adjusted_rank.unwrap_or(i64::MAX));
    let exchanges: Vec<Exchange> = exchanges.into_iter().take(limit).collect();
    match output {
        OutputFormat::Table => crate::output::exchanges::print_exchanges_table(&exchanges),
        OutputFormat::Json => {
            crate::output::print_json_wrapped(&exchanges, crate::output::ResponseMeta::coinpaprika("/exchanges"), raw)?;
        }
    }
    Ok(())
}

pub async fn execute_detail(client: &ApiClient, exchange_id: &str, quotes: &str, output: OutputFormat, raw: bool) -> Result<()> {
    let id = crate::idpath::path_segment(exchange_id)?;
    let exchange: Exchange = client.coinpaprika_get(
        &format!("/exchanges/{id}"),
        &[("quotes", quotes)],
    ).await?;
    match output {
        OutputFormat::Table => crate::output::exchanges::print_exchange_detail(&exchange),
        OutputFormat::Json => {
            crate::output::print_json_wrapped(&exchange, crate::output::ResponseMeta::coinpaprika(&format!("/exchange/{exchange_id}")), raw)?;
        }
    }
    Ok(())
}

pub async fn execute_markets(client: &ApiClient, exchange_id: &str, limit: usize, quotes: &str, output: OutputFormat, raw: bool) -> Result<()> {
    let id = crate::idpath::path_segment(exchange_id)?;
    let markets: Vec<ExchangeMarket> = client.coinpaprika_get(
        &format!("/exchanges/{id}/markets"),
        &[("quotes", quotes)],
    ).await?;
    let markets: Vec<ExchangeMarket> = markets.into_iter().take(limit).collect();
    match output {
        OutputFormat::Table => crate::output::exchanges::print_exchange_markets_table(&markets),
        OutputFormat::Json => {
            crate::output::print_json_wrapped(&markets, crate::output::ResponseMeta::coinpaprika(&format!("/exchange/{exchange_id}")), raw)?;
        }
    }
    Ok(())
}
