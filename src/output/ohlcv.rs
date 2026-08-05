use tabled::{Table, Tabled};
use tabled::settings::Style;

use crate::commands::ohlcv::OhlcvData;
use crate::output::{format_price, format_usd, print_coinpaprika_footer};

#[derive(Tabled)]
struct OhlcvRow {
    #[tabled(rename = "Date")]
    date: String,
    #[tabled(rename = "Open")]
    open: String,
    #[tabled(rename = "High")]
    high: String,
    #[tabled(rename = "Low")]
    low: String,
    #[tabled(rename = "Close")]
    close: String,
    #[tabled(rename = "Volume")]
    volume: String,
}

pub fn print_ohlcv_table(data: &[OhlcvData]) {
    let rows: Vec<OhlcvRow> = data.iter().map(|d| OhlcvRow {
        date: d.time_open.as_deref().unwrap_or("-").chars().take(10).collect(),
        open: d.open.map(format_price).unwrap_or_else(|| "-".into()),
        high: d.high.map(format_price).unwrap_or_else(|| "-".into()),
        low: d.low.map(format_price).unwrap_or_else(|| "-".into()),
        close: d.close.map(format_price).unwrap_or_else(|| "-".into()),
        volume: d.volume.map(format_usd).unwrap_or_else(|| "-".into()),
    }).collect();

    let table = Table::new(rows).with(Style::rounded()).to_string();
    println!("{table}");
    print_coinpaprika_footer();
}
