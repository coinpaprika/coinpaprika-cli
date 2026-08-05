use crate::commands::global::GlobalData;
use crate::output::{detail_field, format_percent, format_usd, print_coinpaprika_footer, print_detail_table};

pub fn print_global(data: &GlobalData) {
    let mut rows: Vec<[String; 2]> = Vec::new();
    detail_field!(rows, "Market Cap", data.market_cap_usd.map(format_usd).unwrap_or_else(|| "-".into()));
    detail_field!(rows, "Volume (24h)", data.volume_24h_usd.map(format_usd).unwrap_or_else(|| "-".into()));
    detail_field!(rows, "BTC Dominance", data.bitcoin_dominance_percentage.map(|v| format!("{v:.1}%")).unwrap_or_else(|| "-".into()));
    detail_field!(rows, "Cryptocurrencies", data.cryptocurrencies_number.map(|v| v.to_string()).unwrap_or_else(|| "-".into()));
    detail_field!(rows, "Market Cap ATH", data.market_cap_ath_value.map(format_usd).unwrap_or_else(|| "-".into()));
    detail_field!(rows, "Market Cap ATH Date", data.market_cap_ath_date.clone().unwrap_or_else(|| "-".into()));
    detail_field!(rows, "Market Cap 24h Change", data.market_cap_change_24h.map(format_percent).unwrap_or_else(|| "-".into()));
    detail_field!(rows, "Volume 24h Change", data.volume_24h_change_24h.map(format_percent).unwrap_or_else(|| "-".into()));
    print_detail_table(rows);
    print_coinpaprika_footer();
}
