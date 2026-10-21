use tabled::{Table, Tabled};
use tabled::settings::Style;

use crate::commands::exchanges::{Exchange, ExchangeMarket};
use crate::output::{detail_field, format_usd, print_coinpaprika_footer, print_detail_table, truncate};

#[derive(Tabled)]
struct ExchangeRow {
    #[tabled(rename = "Rank")]
    rank: String,
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Currencies")]
    currencies: String,
    #[tabled(rename = "Markets")]
    markets: String,
    #[tabled(rename = "Volume (24h)")]
    volume: String,
    #[tabled(rename = "Confidence")]
    confidence: String,
}

pub fn print_exchanges_table(exchanges: &[Exchange]) {
    let rows: Vec<ExchangeRow> = exchanges.iter().map(|e| {
        let vol = e.quotes.as_ref()
            .and_then(|q| q.get("USD"))
            .and_then(|u| u.adjusted_volume_24h)
            .map(format_usd)
            .unwrap_or_else(|| "-".into());
        ExchangeRow {
            rank: e.adjusted_rank.map(|r| r.to_string()).unwrap_or_else(|| "-".into()),
            name: truncate(&e.name, 25),
            currencies: e.currencies.map(|c| c.to_string()).unwrap_or_else(|| "-".into()),
            markets: e.markets.map(|m| m.to_string()).unwrap_or_else(|| "-".into()),
            volume: vol,
            confidence: e.confidence_score.map(|c| format!("{c:.2}")).unwrap_or_else(|| "-".into()),
        }
    }).collect();

    let table = Table::new(rows).with(Style::rounded()).to_string();
    println!("{}", crate::output::sanitize(&table));
    print_coinpaprika_footer();
}

pub fn print_exchange_detail(exchange: &Exchange) {
    let mut rows: Vec<[String; 2]> = Vec::new();
    detail_field!(rows, "Name", exchange.name.clone());
    detail_field!(rows, "ID", exchange.id.clone());
    detail_field!(rows, "Active", exchange.active.map(|a| if a { "Yes" } else { "No" }.to_string()).unwrap_or_else(|| "-".into()));
    detail_field!(rows, "Rank (Adjusted)", exchange.adjusted_rank.map(|r| r.to_string()).unwrap_or_else(|| "-".into()));
    detail_field!(rows, "Currencies", exchange.currencies.map(|c| c.to_string()).unwrap_or_else(|| "-".into()));
    detail_field!(rows, "Markets", exchange.markets.map(|m| m.to_string()).unwrap_or_else(|| "-".into()));
    detail_field!(rows, "Confidence Score", exchange.confidence_score.map(|c| format!("{c:.3}")).unwrap_or_else(|| "-".into()));

    if let Some(quotes) = &exchange.quotes {
        if let Some(usd) = quotes.get("USD") {
            detail_field!(rows, "Volume (24h)", usd.adjusted_volume_24h.map(format_usd).unwrap_or_else(|| "-".into()));
            detail_field!(rows, "Volume (7d)", usd.adjusted_volume_7d.map(format_usd).unwrap_or_else(|| "-".into()));
            detail_field!(rows, "Volume (30d)", usd.adjusted_volume_30d.map(format_usd).unwrap_or_else(|| "-".into()));
        }
    }

    if let Some(desc) = &exchange.description {
        if !desc.is_empty() {
            detail_field!(rows, "Description", truncate(desc, 200));
        }
    }

    detail_field!(rows, "Last Updated", exchange.last_updated.clone().unwrap_or_else(|| "-".into()));
    print_detail_table(rows);
    print_coinpaprika_footer();
}

#[derive(Tabled)]
struct ExchangeMarketRow {
    #[tabled(rename = "Pair")]
    pair: String,
    #[tabled(rename = "Price")]
    price: String,
    #[tabled(rename = "Volume (24h)")]
    volume: String,
    #[tabled(rename = "Trust")]
    trust: String,
}

pub fn print_exchange_markets_table(markets: &[ExchangeMarket]) {
    let rows: Vec<ExchangeMarketRow> = markets.iter().map(|m| {
        let (price, volume) = m.quotes.as_ref()
            .and_then(|q| q.get("USD"))
            .map(|usd| {
                let p = usd.price.map(crate::output::format_price).unwrap_or_else(|| "-".into());
                let v = usd.volume_24h.map(format_usd).unwrap_or_else(|| "-".into());
                (p, v)
            })
            .unwrap_or_else(|| ("-".into(), "-".into()));

        ExchangeMarketRow {
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
