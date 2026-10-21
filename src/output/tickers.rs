use tabled::{Table, Tabled};
use tabled::settings::Style;

use crate::commands::tickers::{Ticker, TickerHistoryPoint};
use crate::output::{detail_field, format_percent, format_price, format_supply, format_usd, print_coinpaprika_footer, print_detail_table};

#[derive(Tabled)]
struct TickerRow {
    #[tabled(rename = "Rank")]
    rank: String,
    #[tabled(rename = "Symbol")]
    symbol: String,
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Price")]
    price: String,
    #[tabled(rename = "24h Change")]
    change_24h: String,
    #[tabled(rename = "Market Cap")]
    market_cap: String,
    #[tabled(rename = "Volume (24h)")]
    volume: String,
}

pub fn print_tickers_table(tickers: &[Ticker]) {
    let rows: Vec<TickerRow> = tickers.iter().map(|t| {
        let usd = t.quotes.as_ref().and_then(|q| q.get("USD"));
        TickerRow {
            rank: t.rank.map(|r| r.to_string()).unwrap_or_else(|| "-".into()),
            symbol: t.symbol.clone(),
            name: crate::output::truncate(&t.name, 20),
            price: usd.and_then(|u| u.price).map(format_price).unwrap_or_else(|| "-".into()),
            change_24h: usd.and_then(|u| u.percent_change_24h).map(format_percent).unwrap_or_else(|| "-".into()),
            market_cap: usd.and_then(|u| u.market_cap).map(format_usd).unwrap_or_else(|| "-".into()),
            volume: usd.and_then(|u| u.volume_24h).map(format_usd).unwrap_or_else(|| "-".into()),
        }
    }).collect();

    let table = Table::new(rows).with(Style::rounded()).to_string();
    println!("{}", crate::output::sanitize(&table));
    print_coinpaprika_footer();
}

pub fn print_ticker_detail(ticker: &Ticker) {
    let mut rows: Vec<[String; 2]> = Vec::new();
    detail_field!(rows, "Name", ticker.name.clone());
    detail_field!(rows, "Symbol", ticker.symbol.clone());
    detail_field!(rows, "Rank", ticker.rank.map(|r| r.to_string()).unwrap_or_else(|| "-".into()));

    if let Some(quotes) = &ticker.quotes {
        if let Some(usd) = quotes.get("USD") {
            detail_field!(rows, "Price (USD)", usd.price.map(format_price).unwrap_or_else(|| "-".into()));
            detail_field!(rows, "Market Cap", usd.market_cap.map(format_usd).unwrap_or_else(|| "-".into()));
            detail_field!(rows, "Volume (24h)", usd.volume_24h.map(format_usd).unwrap_or_else(|| "-".into()));
            detail_field!(rows, "Change (1h)", usd.percent_change_1h.map(format_percent).unwrap_or_else(|| "-".into()));
            detail_field!(rows, "Change (24h)", usd.percent_change_24h.map(format_percent).unwrap_or_else(|| "-".into()));
            detail_field!(rows, "Change (7d)", usd.percent_change_7d.map(format_percent).unwrap_or_else(|| "-".into()));
            detail_field!(rows, "Change (30d)", usd.percent_change_30d.map(format_percent).unwrap_or_else(|| "-".into()));
            detail_field!(rows, "ATH Price", usd.ath_price.map(format_price).unwrap_or_else(|| "-".into()));
            detail_field!(rows, "ATH Date", usd.ath_date.clone().unwrap_or_else(|| "-".into()));
            detail_field!(rows, "% From ATH", usd.percent_from_price_ath.map(format_percent).unwrap_or_else(|| "-".into()));
        }
    }

    detail_field!(rows, "Circulating Supply", ticker.circulating_supply.map(format_supply).unwrap_or_else(|| "-".into()));
    detail_field!(rows, "Total Supply", ticker.total_supply.map(format_supply).unwrap_or_else(|| "-".into()));
    detail_field!(rows, "Max Supply", ticker.max_supply.map(format_supply).unwrap_or_else(|| "-".into()));
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

pub fn print_history_table(history: &[TickerHistoryPoint]) {
    let rows: Vec<HistoryRow> = history.iter().map(|h| HistoryRow {
        timestamp: h.timestamp.clone().unwrap_or_else(|| "-".into()),
        price: h.price.map(format_price).unwrap_or_else(|| "-".into()),
        volume: h.volume_24h.map(format_usd).unwrap_or_else(|| "-".into()),
        market_cap: h.market_cap.map(format_usd).unwrap_or_else(|| "-".into()),
    }).collect();

    let table = Table::new(rows).with(Style::rounded()).to_string();
    println!("{}", crate::output::sanitize(&table));
    print_coinpaprika_footer();
}
