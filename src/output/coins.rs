use tabled::{Table, Tabled};
use tabled::settings::Style;

use crate::commands::coins::{CoinListItem, CoinDetail, CoinEvent, CoinExchange, CoinMarket};
use crate::output::{detail_field, truncate, format_usd, print_coinpaprika_footer, print_detail_table};

#[derive(Tabled)]
struct CoinRow {
    #[tabled(rename = "Rank")]
    rank: String,
    #[tabled(rename = "Symbol")]
    symbol: String,
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Type")]
    coin_type: String,
    #[tabled(rename = "Active")]
    active: String,
}

pub fn print_coins_table(coins: &[CoinListItem]) {
    let rows: Vec<CoinRow> = coins.iter().map(|c| CoinRow {
        rank: c.rank.map(|r| r.to_string()).unwrap_or_else(|| "-".into()),
        symbol: c.symbol.clone(),
        name: truncate(&c.name, 30),
        coin_type: c.coin_type.clone().unwrap_or_else(|| "-".into()),
        active: c.is_active.map(|a| if a { "Yes" } else { "No" }.to_string()).unwrap_or_else(|| "-".into()),
    }).collect();

    let table = Table::new(rows).with(Style::rounded()).to_string();
    println!("{}", crate::output::sanitize(&table));
    print_coinpaprika_footer();
}

pub fn print_coin_detail(coin: &CoinDetail) {
    let mut rows: Vec<[String; 2]> = Vec::new();
    detail_field!(rows, "Name", coin.name.clone());
    detail_field!(rows, "Symbol", coin.symbol.clone());
    detail_field!(rows, "Rank", coin.rank.map(|r| r.to_string()).unwrap_or_else(|| "-".into()));
    detail_field!(rows, "Type", coin.coin_type.clone().unwrap_or_else(|| "-".into()));
    detail_field!(rows, "Active", coin.is_active.map(|a| if a { "Yes" } else { "No" }.to_string()).unwrap_or_else(|| "-".into()));
    detail_field!(rows, "Started", coin.started_at.clone().unwrap_or_else(|| "-".into()));
    detail_field!(rows, "Proof Type", coin.proof_type.clone().unwrap_or_else(|| "-".into()));
    detail_field!(rows, "Hash Algorithm", coin.hash_algorithm.clone().unwrap_or_else(|| "-".into()));
    detail_field!(rows, "Org Structure", coin.org_structure.clone().unwrap_or_else(|| "-".into()));
    detail_field!(rows, "Dev Status", coin.development_status.clone().unwrap_or_else(|| "-".into()));
    detail_field!(rows, "Open Source", coin.open_source.map(|v| if v { "Yes" } else { "No" }.to_string()).unwrap_or_else(|| "-".into()));
    detail_field!(rows, "Hardware Wallet", coin.hardware_wallet.map(|v| if v { "Yes" } else { "No" }.to_string()).unwrap_or_else(|| "-".into()));

    if let Some(tags) = &coin.tags {
        let tag_names: Vec<String> = tags.iter().map(|t| t.name.clone()).collect();
        detail_field!(rows, "Tags", truncate(&tag_names.join(", "), 80));
    }

    if let Some(desc) = &coin.description {
        if !desc.is_empty() {
            detail_field!(rows, "Description", truncate(desc, 200));
        }
    }

    if let Some(wp) = &coin.whitepaper {
        if let Some(link) = &wp.link {
            detail_field!(rows, "Whitepaper", link.clone());
        }
    }

    print_detail_table(rows);
    print_coinpaprika_footer();
}

#[derive(Tabled)]
struct EventRow {
    #[tabled(rename = "Date")]
    date: String,
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Conference")]
    is_conference: String,
}

pub fn print_events_table(events: &[CoinEvent]) {
    let rows: Vec<EventRow> = events.iter().map(|e| EventRow {
        date: e.date.clone().unwrap_or_else(|| "-".into()),
        name: truncate(&e.name.clone().unwrap_or_else(|| "-".into()), 50),
        is_conference: e.is_conference.map(|v| if v { "Yes" } else { "No" }.to_string()).unwrap_or_else(|| "-".into()),
    }).collect();

    let table = Table::new(rows).with(Style::rounded()).to_string();
    println!("{}", crate::output::sanitize(&table));
    print_coinpaprika_footer();
}

#[derive(Tabled)]
struct CoinExchangeRow {
    #[tabled(rename = "Exchange")]
    name: String,
    #[tabled(rename = "Volume Share (24h)")]
    volume_share: String,
}

pub fn print_coin_exchanges_table(exchanges: &[CoinExchange]) {
    let rows: Vec<CoinExchangeRow> = exchanges.iter().map(|e| CoinExchangeRow {
        name: e.name.clone(),
        volume_share: e.adjusted_volume_24h_share.map(|v| format!("{v:.2}%")).unwrap_or_else(|| "-".into()),
    }).collect();

    let table = Table::new(rows).with(Style::rounded()).to_string();
    println!("{}", crate::output::sanitize(&table));
    print_coinpaprika_footer();
}

#[derive(Tabled)]
struct MarketRow {
    #[tabled(rename = "Exchange")]
    exchange: String,
    #[tabled(rename = "Pair")]
    pair: String,
    #[tabled(rename = "Price")]
    price: String,
    #[tabled(rename = "Volume (24h)")]
    volume: String,
    #[tabled(rename = "Trust")]
    trust: String,
}

pub fn print_markets_table(markets: &[CoinMarket]) {
    let rows: Vec<MarketRow> = markets.iter().map(|m| {
        let (price, volume) = m.quotes.as_ref()
            .and_then(|q| q.get("USD"))
            .map(|usd| {
                let p = usd.price.map(crate::output::format_price).unwrap_or_else(|| "-".into());
                let v = usd.volume_24h.map(format_usd).unwrap_or_else(|| "-".into());
                (p, v)
            })
            .unwrap_or_else(|| ("-".into(), "-".into()));

        MarketRow {
            exchange: m.exchange_name.clone().unwrap_or_else(|| "-".into()),
            pair: m.pair.clone().unwrap_or_else(|| "-".into()),
            price,
            volume,
            trust: m.trust_score.clone().unwrap_or_else(|| "-".into()),
        }
    }).collect();

    let table = Table::new(rows).with(Style::rounded()).to_string();
    println!("{}", crate::output::sanitize(&table));
    print_coinpaprika_footer();
}
