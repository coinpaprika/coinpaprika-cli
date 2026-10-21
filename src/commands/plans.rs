use anyhow::Result;
use serde::Serialize;

use crate::output::OutputFormat;

#[derive(Debug, Serialize)]
struct FreeTier {
    rate_limit: &'static str,
    update_frequency: &'static str,
    endpoints: &'static str,
    assets: &'static str,
    use_case: &'static str,
    daily_ohlcv_history: &'static str,
    hourly_ohlcv: &'static str,
    ohlcv_intervals: &'static str,
    ticker_history: bool,
    circulating_supply: bool,
    api_id_mappings: bool,
    websockets: bool,
    redistribution: bool,
    sla: bool,
}

#[derive(Debug, Serialize)]
struct PlansInfo {
    free_tier: FreeTier,
    paid_plans_url: &'static str,
    set_key_command: &'static str,
}

pub fn execute(output: OutputFormat, raw: bool) -> Result<()> {
    let info = PlansInfo {
        free_tier: FreeTier {
            rate_limit: "20,000 calls/month",
            update_frequency: "~10 minutes",
            endpoints: "25+",
            assets: "2,000",
            use_case: "Personal",
            daily_ohlcv_history: "up to 1 year back",
            hourly_ohlcv: "last 24 hours",
            ohlcv_intervals: "24h only",
            ticker_history: false,
            circulating_supply: false,
            api_id_mappings: false,
            websockets: false,
            redistribution: false,
            sla: false,
        },
        paid_plans_url: "https://coinpaprika.com/api/pricing",
        set_key_command: "coinpaprika-cli onboard",
    };

    match output {
        OutputFormat::Table => {
            println!();
            println!("  ─── Free tier ($0/mo, no API key needed) ───");
            println!();
            println!("  Rate limits");
            println!("    20,000 calls/month");
            println!("    Data updates every ~10 minutes");
            println!();
            println!("  Coverage");
            println!("    25+ endpoints");
            println!("    2,000 assets");
            println!("    Personal use");
            println!();
            println!("  Historical data");
            println!("    Daily OHLCV:    up to 1 year back");
            println!("    Hourly OHLCV:   last 24 hours");
            println!("    OHLCV interval: 24h only");
            println!("    5-min / ticker history: not available");
            println!();
            println!("  Not included");
            println!("    Circulating supply");
            println!("    API ID mappings");
            println!("    WebSockets");
            println!("    Redistribution rights");
            println!("    SLA / dedicated infrastructure");
            println!();
            println!("  ─── Need more? ───");
            println!();
            println!("  Paid plans add: full history, 5-min intervals, circulating supply,");
            println!("  higher limits, WebSockets, commercial use, and priority support.");
            println!();
            println!("  See current pricing:  https://coinpaprika.com/api/pricing");
            println!("  Set your API key:     coinpaprika-cli onboard");
            println!();
        }
        OutputFormat::Json => {
            crate::output::print_json_wrapped(&info, crate::output::ResponseMeta::coinpaprika("/plans"), raw)?;
        }
    }

    Ok(())
}
