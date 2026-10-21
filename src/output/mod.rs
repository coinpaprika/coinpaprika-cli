use anyhow::Result;
use chrono::Utc;
use serde::Serialize;
use tabled::settings::object::Columns;
use tabled::settings::{Modify, Style, Width};
use tabled::Table;

#[derive(Clone, Copy, Debug, clap::ValueEnum)]
pub enum OutputFormat {
    Table,
    Json,
}

// --- Attribution / _meta wrapper ---

#[derive(Serialize)]
pub struct WrappedResponse<'a, T: Serialize> {
    pub data: &'a T,
    pub _meta: ResponseMeta,
}

#[derive(Serialize)]
pub struct ResponseMeta {
    pub source: String,
    pub url: String,
    pub api_docs: String,
    pub attribution: String,
    pub timestamp: String,
}

impl ResponseMeta {
    pub fn coinpaprika(entity_path: &str) -> Self {
        Self {
            source: "CoinPaprika".into(),
            url: format!("https://coinpaprika.com{entity_path}"),
            api_docs: "https://api.coinpaprika.com".into(),
            attribution: "Powered by CoinPaprika · Free crypto market data".into(),
            timestamp: Utc::now().to_rfc3339(),
        }
    }
}

// --- Shared output helpers ---

pub fn truncate(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        return s.to_string();
    }
    let mut truncated: String = s.chars().take(max.saturating_sub(1)).collect();
    truncated.push('\u{2026}');
    truncated
}

pub fn truncate_address(addr: &str) -> String {
    if addr.len() <= 13 {
        return addr.to_string();
    }
    format!("{}...{}", &addr[..6], &addr[addr.len() - 4..])
}

pub fn format_usd(n: f64) -> String {
    if n.abs() >= 1_000_000_000_000.0 {
        format!("${:.1}T", n / 1_000_000_000_000.0)
    } else if n.abs() >= 1_000_000_000.0 {
        format!("${:.1}B", n / 1_000_000_000.0)
    } else if n.abs() >= 1_000_000.0 {
        format!("${:.1}M", n / 1_000_000.0)
    } else if n.abs() >= 1_000.0 {
        format!("${:.1}K", n / 1_000.0)
    } else {
        format!("${n:.2}")
    }
}

pub fn format_price(n: f64) -> String {
    if n >= 1.0 {
        format!("${n:.2}")
    } else if n >= 0.01 {
        format!("${n:.4}")
    } else {
        format!("${n:.8}")
    }
}

pub fn format_percent(n: f64) -> String {
    if n >= 0.0 {
        format!("+{n:.2}%")
    } else {
        format!("{n:.2}%")
    }
}

pub fn format_supply(n: f64) -> String {
    if n >= 1_000_000_000.0 {
        format!("{:.1}B", n / 1_000_000_000.0)
    } else if n >= 1_000_000.0 {
        format!("{:.1}M", n / 1_000_000.0)
    } else if n >= 1_000.0 {
        format!("{:.1}K", n / 1_000.0)
    } else {
        format!("{n:.0}")
    }
}

#[allow(dead_code)]
pub fn print_json<T: Serialize>(data: &T) -> Result<()> {
    println!("{}", serde_json::to_string_pretty(data)?);
    Ok(())
}

pub fn print_json_wrapped<T: Serialize>(data: &T, meta: ResponseMeta, raw: bool) -> Result<()> {
    if raw {
        println!("{}", serde_json::to_string_pretty(data)?);
    } else {
        let wrapped = WrappedResponse { data, _meta: meta };
        println!("{}", serde_json::to_string_pretty(&wrapped)?);
    }
    Ok(())
}

/// Strip C0 control characters (including ESC, 0x00-0x1F), DEL (0x7F) and
/// C1 control characters (0x80-0x9F) from text that came from the API
/// before it reaches the terminal.
///
/// Table rendering (tabled, with default features) only rewrites tabs; it
/// does not know about terminal escape sequences. Without this, API text
/// such as a coin description, a search result name, or a non-2xx response
/// body could carry a spoofed OSC 8 hyperlink, a terminal title change, or
/// hidden text straight into the user's terminal. Every API-derived string
/// that reaches table or plain println!/eprintln! output goes through this
/// one helper; JSON output is unaffected because serde_json already
/// escapes control characters.
pub fn sanitize(s: &str) -> String {
    s.chars()
        .filter(|c| {
            let code = *c as u32;
            !(code <= 0x1F || code == 0x7F || (0x80..=0x9F).contains(&code))
        })
        .collect()
}

pub fn print_detail_table(rows: Vec<[String; 2]>) {
    let table = Table::from_iter(rows)
        .with(Style::rounded())
        .with(Modify::new(Columns::first()).with(Width::wrap(20)))
        .with(Modify::new(Columns::last()).with(Width::wrap(80)))
        .to_string();
    println!("{}", sanitize(&table));
}

pub fn print_coinpaprika_footer() {
    println!(
        "\n Data: CoinPaprika (coinpaprika.com) \u{00b7} Free API: api.coinpaprika.com"
    );
}

macro_rules! detail_field {
    ($rows:expr, $label:expr, $val:expr) => {
        $rows.push([$label.into(), $val]);
    };
}

pub(crate) use detail_field;

pub mod global;
pub mod coins;
pub mod tickers;
pub mod ohlcv;
pub mod exchanges;
pub mod tags;
pub mod people;
pub mod search;
pub mod convert;
pub mod contracts;
pub mod api_management;
pub mod config;
pub mod status;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncate_short_string_unchanged() {
        assert_eq!(truncate("hello", 10), "hello");
    }

    #[test]
    fn truncate_long_string_appends_ellipsis() {
        let result = truncate("hello world", 5);
        assert_eq!(result.chars().count(), 5);
        assert!(result.ends_with('\u{2026}'));
    }

    #[test]
    fn format_usd_billions() {
        assert_eq!(format_usd(1_500_000_000.0), "$1.5B");
    }

    #[test]
    fn format_usd_millions() {
        assert_eq!(format_usd(12_400_000.0), "$12.4M");
    }

    #[test]
    fn format_usd_thousands() {
        assert_eq!(format_usd(5_200.0), "$5.2K");
    }

    #[test]
    fn format_usd_small() {
        assert_eq!(format_usd(42.5), "$42.50");
    }

    #[test]
    fn format_percent_positive() {
        assert_eq!(format_percent(2.34), "+2.34%");
    }

    #[test]
    fn format_percent_negative() {
        assert_eq!(format_percent(-1.23), "-1.23%");
    }

    #[test]
    fn format_price_large() {
        assert_eq!(format_price(97234.56), "$97234.56");
    }

    #[test]
    fn format_price_small() {
        assert_eq!(format_price(0.001234), "$0.00123400");
    }

    #[test]
    fn truncate_address_short() {
        assert_eq!(truncate_address("0x1234567890abcdef"), "0x1234...cdef");
    }

    #[test]
    fn sanitize_strips_escape_sequences() {
        // ESC ] 8 ; ; url BEL ... ESC ] 8 ; ; BEL is an OSC 8 hyperlink.
        let malicious = "A\u{1b}]8;;https://example.invalid\u{7}B\u{1b}]8;;\u{7}C";
        let cleaned = sanitize(malicious);
        assert_eq!(cleaned, "A]8;;https://example.invalidB]8;;C");
        assert!(!cleaned.contains('\u{1b}'));
        assert!(!cleaned.contains('\u{7}'));
    }

    #[test]
    fn sanitize_strips_c0_del_and_c1() {
        let s = "a\u{0}b\u{1f}c\u{7f}d\u{80}e\u{9f}f";
        assert_eq!(sanitize(s), "abcdef");
    }

    #[test]
    fn sanitize_keeps_ordinary_text_unchanged() {
        let s = "Bitcoin is a decentralized digital currency, 2009.";
        assert_eq!(sanitize(s), s);
    }

    #[test]
    fn sanitize_keeps_unicode_text() {
        let s = "satoshi-nakamoto \u{00e9}\u{00e8} \u{4e2d}\u{6587}";
        assert_eq!(sanitize(s), s);
    }
}
