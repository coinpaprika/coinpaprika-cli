use tabled::{Table, Tabled};
use tabled::settings::Style;

use crate::commands::contracts::{Contract, ContractTicker, ContractHistoryPoint};
use crate::output::{detail_field, format_percent, format_price, format_supply, format_usd, print_coinpaprika_footer, print_detail_table, sanitize, truncate_address};

pub fn print_platforms(platforms: &[String]) {
    println!("Contract Platforms:");
    for p in platforms {
        println!("  {}", sanitize(p));
    }
    print_coinpaprika_footer();
}

#[derive(Tabled)]
struct ContractRow {
    #[tabled(rename = "Address")]
    address: String,
    #[tabled(rename = "Type")]
    contract_type: String,
    #[tabled(rename = "Coin ID")]
    id: String,
    #[tabled(rename = "Active")]
    active: String,
}

pub fn print_contracts_table(contracts: &[Contract]) {
    let rows: Vec<ContractRow> = contracts.iter().map(|c| ContractRow {
        address: c.address.as_deref().map(truncate_address).unwrap_or_else(|| "-".into()),
        contract_type: c.contract_type.clone().unwrap_or_else(|| "-".into()),
        id: c.id.clone().unwrap_or_else(|| "-".into()),
        active: c.active.map(|a| if a { "Yes" } else { "No" }.to_string()).unwrap_or_else(|| "-".into()),
    }).collect();

    let table = Table::new(rows).with(Style::rounded()).to_string();
    println!("{}", crate::output::sanitize(&table));
    print_coinpaprika_footer();
}

pub fn print_contract_ticker(ticker: &ContractTicker) {
    let mut rows: Vec<[String; 2]> = Vec::new();
    detail_field!(rows, "Name", ticker.name.clone().unwrap_or_else(|| "-".into()));
    detail_field!(rows, "Symbol", ticker.symbol.clone().unwrap_or_else(|| "-".into()));
    detail_field!(rows, "Rank", ticker.rank.map(|r| r.to_string()).unwrap_or_else(|| "-".into()));

    if let Some(quotes) = &ticker.quotes {
        if let Some(usd) = quotes.get("USD") {
            detail_field!(rows, "Price (USD)", usd.price.map(format_price).unwrap_or_else(|| "-".into()));
            detail_field!(rows, "Market Cap", usd.market_cap.map(format_usd).unwrap_or_else(|| "-".into()));
            detail_field!(rows, "Volume (24h)", usd.volume_24h.map(format_usd).unwrap_or_else(|| "-".into()));
            detail_field!(rows, "Change (24h)", usd.percent_change_24h.map(format_percent).unwrap_or_else(|| "-".into()));
            detail_field!(rows, "Change (7d)", usd.percent_change_7d.map(format_percent).unwrap_or_else(|| "-".into()));
        }
    }

    detail_field!(rows, "Circulating Supply", ticker.circulating_supply.map(format_supply).unwrap_or_else(|| "-".into()));
    detail_field!(rows, "Total Supply", ticker.total_supply.map(format_supply).unwrap_or_else(|| "-".into()));
    detail_field!(rows, "Last Updated", ticker.last_updated.clone().unwrap_or_else(|| "-".into()));
    print_detail_table(rows);
    print_coinpaprika_footer();
}

#[derive(Tabled)]
struct HistoryRow {
    #[tabled(rename = "Timestamp")]
    timestamp: String,
    #[tabled(rename = "Price")]
    price: String,
    #[tabled(rename = "Volume (24h)")]
    volume: String,
    #[tabled(rename = "Market Cap")]
    market_cap: String,
}

pub fn print_contract_history(data: &[ContractHistoryPoint]) {
    let rows: Vec<HistoryRow> = data.iter().map(|d| HistoryRow {
        timestamp: d.timestamp.clone().unwrap_or_else(|| "-".into()),
        price: d.price.map(format_price).unwrap_or_else(|| "-".into()),
        volume: d.volume_24h.map(format_usd).unwrap_or_else(|| "-".into()),
        market_cap: d.market_cap.map(format_usd).unwrap_or_else(|| "-".into()),
    }).collect();

    let table = Table::new(rows).with(Style::rounded()).to_string();
    println!("{}", crate::output::sanitize(&table));
    print_coinpaprika_footer();
}
