use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::client::ApiClient;
use crate::output::OutputFormat;

#[derive(Debug, Deserialize, Serialize)]
pub struct CoinListItem {
    pub id: String,
    pub name: String,
    pub symbol: String,
    pub rank: Option<i64>,
    pub is_new: Option<bool>,
    pub is_active: Option<bool>,
    #[serde(rename = "type")]
    pub coin_type: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CoinDetail {
    pub id: String,
    pub name: String,
    pub symbol: String,
    pub rank: Option<i64>,
    pub is_new: Option<bool>,
    pub is_active: Option<bool>,
    #[serde(rename = "type")]
    pub coin_type: Option<String>,
    pub logo: Option<String>,
    pub description: Option<String>,
    pub open_source: Option<bool>,
    pub started_at: Option<String>,
    pub development_status: Option<String>,
    pub hardware_wallet: Option<bool>,
    pub proof_type: Option<String>,
    pub org_structure: Option<String>,
    pub hash_algorithm: Option<String>,
    pub tags: Option<Vec<CoinTag>>,
    pub team: Option<Vec<TeamMember>>,
    pub links: Option<HashMap<String, serde_json::Value>>,
    pub whitepaper: Option<Whitepaper>,
    pub first_data_at: Option<String>,
    pub last_data_at: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CoinTag {
    pub id: String,
    pub name: String,
    pub coin_counter: Option<i64>,
    pub ico_counter: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TeamMember {
    pub id: String,
    pub name: String,
    pub position: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Whitepaper {
    pub link: Option<String>,
    pub thumbnail: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CoinEvent {
    pub id: Option<String>,
    pub date: Option<String>,
    pub date_to: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_conference: Option<bool>,
    pub link: Option<String>,
    pub proof_image_link: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CoinExchange {
    pub id: String,
    pub name: String,
    pub adjusted_volume_24h_share: Option<f64>,
    pub fiats: Option<Vec<FiatCurrency>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FiatCurrency {
    pub name: Option<String>,
    pub symbol: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CoinMarket {
    pub exchange_id: Option<String>,
    pub exchange_name: Option<String>,
    pub pair: Option<String>,
    pub base_currency_id: Option<String>,
    pub base_currency_name: Option<String>,
    pub quote_currency_id: Option<String>,
    pub quote_currency_name: Option<String>,
    pub market_url: Option<String>,
    pub category: Option<String>,
    pub fee_type: Option<String>,
    pub outlier: Option<bool>,
    pub adjusted_volume_24h_share: Option<f64>,
    pub quotes: Option<HashMap<String, MarketQuote>>,
    pub trust_score: Option<String>,
    pub last_updated: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MarketQuote {
    pub price: Option<f64>,
    pub volume_24h: Option<f64>,
}

pub async fn execute_list(client: &ApiClient, limit: usize, output: OutputFormat, raw: bool) -> Result<()> {
    let coins: Vec<CoinListItem> = client.coinpaprika_get("/coins", &[]).await?;
    let coins: Vec<CoinListItem> = coins.into_iter().take(limit).collect();
    match output {
        OutputFormat::Table => crate::output::coins::print_coins_table(&coins),
        OutputFormat::Json => {
            crate::output::print_json_wrapped(&coins, crate::output::ResponseMeta::coinpaprika("/coins"), raw)?;
        }
    }
    Ok(())
}

pub async fn execute_detail(client: &ApiClient, coin_id: &str, output: OutputFormat, raw: bool) -> Result<()> {
    let id = crate::idpath::path_segment(coin_id)?;
    let coin: CoinDetail = client.coinpaprika_get(&format!("/coins/{id}"), &[]).await?;
    match output {
        OutputFormat::Table => crate::output::coins::print_coin_detail(&coin),
        OutputFormat::Json => {
            crate::output::print_json_wrapped(&coin, crate::output::ResponseMeta::coinpaprika(&format!("/coin/{coin_id}")), raw)?;
        }
    }
    Ok(())
}

pub async fn execute_events(client: &ApiClient, coin_id: &str, limit: usize, output: OutputFormat, raw: bool) -> Result<()> {
    let id = crate::idpath::path_segment(coin_id)?;
    let events: Vec<CoinEvent> = client.coinpaprika_get(&format!("/coins/{id}/events"), &[]).await?;
    let events: Vec<CoinEvent> = events.into_iter().take(limit).collect();
    match output {
        OutputFormat::Table => crate::output::coins::print_events_table(&events),
        OutputFormat::Json => {
            crate::output::print_json_wrapped(&events, crate::output::ResponseMeta::coinpaprika(&format!("/coin/{coin_id}")), raw)?;
        }
    }
    Ok(())
}

pub async fn execute_exchanges(client: &ApiClient, coin_id: &str, limit: usize, output: OutputFormat, raw: bool) -> Result<()> {
    let id = crate::idpath::path_segment(coin_id)?;
    let exchanges: Vec<CoinExchange> = client.coinpaprika_get(&format!("/coins/{id}/exchanges"), &[]).await?;
    let exchanges: Vec<CoinExchange> = exchanges.into_iter().take(limit).collect();
    match output {
        OutputFormat::Table => crate::output::coins::print_coin_exchanges_table(&exchanges),
        OutputFormat::Json => {
            crate::output::print_json_wrapped(&exchanges, crate::output::ResponseMeta::coinpaprika(&format!("/coin/{coin_id}")), raw)?;
        }
    }
    Ok(())
}

pub async fn execute_markets(client: &ApiClient, coin_id: &str, quotes: &str, limit: usize, output: OutputFormat, raw: bool) -> Result<()> {
    let id = crate::idpath::path_segment(coin_id)?;
    let markets: Vec<CoinMarket> = client.coinpaprika_get(
        &format!("/coins/{id}/markets"),
        &[("quotes", quotes)],
    ).await?;
    let markets: Vec<CoinMarket> = markets.into_iter().take(limit).collect();
    match output {
        OutputFormat::Table => crate::output::coins::print_markets_table(&markets),
        OutputFormat::Json => {
            crate::output::print_json_wrapped(&markets, crate::output::ResponseMeta::coinpaprika(&format!("/coin/{coin_id}")), raw)?;
        }
    }
    Ok(())
}
