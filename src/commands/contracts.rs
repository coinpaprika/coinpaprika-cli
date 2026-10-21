use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::client::ApiClient;
use crate::output::OutputFormat;

#[derive(Debug, Deserialize, Serialize)]
pub struct Contract {
    pub address: Option<String>,
    #[serde(rename = "type")]
    pub contract_type: Option<String>,
    pub id: Option<String>,
    pub active: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ContractTicker {
    pub id: Option<String>,
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub rank: Option<i64>,
    pub circulating_supply: Option<f64>,
    pub total_supply: Option<f64>,
    pub max_supply: Option<f64>,
    pub beta_value: Option<f64>,
    pub first_data_at: Option<String>,
    pub last_updated: Option<String>,
    pub quotes: Option<HashMap<String, ContractQuote>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ContractQuote {
    pub price: Option<f64>,
    pub volume_24h: Option<f64>,
    pub volume_24h_change_24h: Option<f64>,
    pub market_cap: Option<f64>,
    pub market_cap_change_24h: Option<f64>,
    pub percent_change_24h: Option<f64>,
    pub percent_change_7d: Option<f64>,
    pub percent_change_30d: Option<f64>,
    pub percent_change_1y: Option<f64>,
    pub ath_price: Option<f64>,
    pub ath_date: Option<String>,
    pub percent_from_price_ath: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ContractHistoryPoint {
    pub timestamp: Option<String>,
    pub price: Option<f64>,
    pub volume_24h: Option<f64>,
    pub market_cap: Option<f64>,
}

pub async fn execute_platforms(client: &ApiClient, output: OutputFormat, raw: bool) -> Result<()> {
    let platforms: Vec<String> = client.coinpaprika_get("/contracts", &[]).await?;
    match output {
        OutputFormat::Table => crate::output::contracts::print_platforms(&platforms),
        OutputFormat::Json => {
            crate::output::print_json_wrapped(&platforms, crate::output::ResponseMeta::coinpaprika("/contracts"), raw)?;
        }
    }
    Ok(())
}

pub async fn execute_contracts(client: &ApiClient, platform_id: &str, limit: usize, output: OutputFormat, raw: bool) -> Result<()> {
    let id = crate::idpath::path_segment(platform_id)?;
    let contracts: Vec<Contract> = client.coinpaprika_get(&format!("/contracts/{id}"), &[]).await?;
    let contracts: Vec<Contract> = contracts.into_iter().take(limit).collect();
    match output {
        OutputFormat::Table => crate::output::contracts::print_contracts_table(&contracts),
        OutputFormat::Json => {
            crate::output::print_json_wrapped(&contracts, crate::output::ResponseMeta::coinpaprika(&format!("/contracts/{platform_id}")), raw)?;
        }
    }
    Ok(())
}

pub async fn execute_ticker(client: &ApiClient, platform_id: &str, address: &str, output: OutputFormat, raw: bool) -> Result<()> {
    let platform = crate::idpath::path_segment(platform_id)?;
    let addr = crate::idpath::path_segment(address)?;
    let ticker: ContractTicker = client.coinpaprika_get(
        &format!("/contracts/{platform}/{addr}"),
        &[],
    ).await?;
    match output {
        OutputFormat::Table => crate::output::contracts::print_contract_ticker(&ticker),
        OutputFormat::Json => {
            crate::output::print_json_wrapped(&ticker, crate::output::ResponseMeta::coinpaprika(&format!("/contracts/{platform_id}/{address}")), raw)?;
        }
    }
    Ok(())
}

pub async fn execute_history(
    client: &ApiClient,
    platform_id: &str,
    address: &str,
    start: &str,
    end: Option<&str>,
    interval: &str,
    limit: usize,
    output: OutputFormat,
    raw: bool,
) -> Result<()> {
    let limit_str = limit.to_string();
    let mut params: Vec<(&str, &str)> = vec![
        ("start", start),
        ("interval", interval),
        ("limit", &limit_str),
    ];
    if let Some(e) = end {
        params.push(("end", e));
    }

    let platform = crate::idpath::path_segment(platform_id)?;
    let addr = crate::idpath::path_segment(address)?;
    let data: Vec<ContractHistoryPoint> = client.coinpaprika_get(
        &format!("/contracts/{platform}/{addr}/historical"),
        &params,
    ).await?;
    match output {
        OutputFormat::Table => crate::output::contracts::print_contract_history(&data),
        OutputFormat::Json => {
            crate::output::print_json_wrapped(&data, crate::output::ResponseMeta::coinpaprika(&format!("/contracts/{platform_id}/{address}")), raw)?;
        }
    }
    Ok(())
}
