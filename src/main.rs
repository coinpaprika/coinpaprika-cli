mod client;
mod config;
mod commands;
mod output;
mod shell;

use clap::{Parser, Subcommand};
use output::OutputFormat;
use std::process::ExitCode;

#[derive(Parser)]
#[command(
    name = "coinpaprika-cli",
    version,
    about = "coinpaprika-cli — Crypto market data for developers and AI agents",
    long_about = "coinpaprika-cli — Crypto market data for developers and AI agents\n\n\
                   8,000+ coins · Real-time prices · OHLCV · Exchanges · Market data\n\n\
                   Free tier: 20,000 calls/mo, no API key needed\n\
                   Paid plans: full history, 5-min intervals, higher limits\n\n\
                   Quick start:  coinpaprika-cli onboard\n\
                   Free vs paid: coinpaprika-cli plans\n\
                   API docs:     https://api.coinpaprika.com\n\
                   Pricing:      https://coinpaprika.com/api/pricing"
)]
pub(crate) struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Output format: table or json
    #[arg(short, long, global = true, default_value = "table")]
    pub(crate) output: OutputFormat,

    /// CoinPaprika API key (overrides env var and config file)
    #[arg(long, global = true)]
    api_key: Option<String>,

    /// JSON output without _meta wrapper (for scripts/piping)
    #[arg(long, global = true, default_value = "false")]
    pub(crate) raw: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Global crypto market overview (market cap, volume, BTC dominance)
    Global,

    /// List all coins
    #[command(after_help = "EXAMPLES:\n  coinpaprika-cli coins --limit 10\n  coinpaprika-cli coins --output json")]
    Coins {
        /// Maximum number of results
        #[arg(long, default_value = "100")]
        limit: usize,
    },

    /// Get detailed info about a specific coin
    #[command(after_help = "EXAMPLES:\n  coinpaprika-cli coin btc-bitcoin\n  coinpaprika-cli coin eth-ethereum --output json")]
    Coin {
        /// Coin ID (e.g., btc-bitcoin, eth-ethereum)
        coin_id: String,
    },

    /// Get events for a coin
    #[command(name = "coin-events", after_help = "EXAMPLES:\n  coinpaprika-cli coin-events btc-bitcoin\n  coinpaprika-cli coin-events eth-ethereum --limit 5")]
    CoinEvents {
        /// Coin ID (e.g., btc-bitcoin)
        coin_id: String,
        /// Maximum number of results
        #[arg(long, default_value = "50")]
        limit: usize,
    },

    /// Get exchanges where a coin is traded
    #[command(name = "coin-exchanges", after_help = "EXAMPLES:\n  coinpaprika-cli coin-exchanges btc-bitcoin --limit 10")]
    CoinExchanges {
        /// Coin ID (e.g., btc-bitcoin)
        coin_id: String,
        /// Maximum number of results
        #[arg(long, default_value = "50")]
        limit: usize,
    },

    /// Get markets for a coin
    #[command(name = "coin-markets", after_help = "EXAMPLES:\n  coinpaprika-cli coin-markets btc-bitcoin --quotes USD,BTC")]
    CoinMarkets {
        /// Coin ID (e.g., btc-bitcoin)
        coin_id: String,
        /// Currency quotes, comma-separated
        #[arg(long, default_value = "USD")]
        quotes: String,
        /// Maximum number of results
        #[arg(long, default_value = "50")]
        limit: usize,
    },

    /// List tickers (real-time price data for all coins)
    #[command(after_help = "EXAMPLES:\n  coinpaprika-cli tickers --limit 20\n  coinpaprika-cli tickers --quotes USD,BTC")]
    Tickers {
        /// Maximum number of results
        #[arg(long, default_value = "50")]
        limit: usize,
        /// Currency quotes, comma-separated
        #[arg(long, default_value = "USD")]
        quotes: String,
    },

    /// Get real-time price data for a specific coin
    #[command(after_help = "EXAMPLES:\n  coinpaprika-cli ticker btc-bitcoin\n  coinpaprika-cli ticker eth-ethereum --quotes USD,BTC\n  coinpaprika-cli ticker btc-bitcoin --output json")]
    Ticker {
        /// Coin ID in format symbol-name (e.g., btc-bitcoin, eth-ethereum)
        coin_id: String,
        /// Currency quotes, comma-separated
        #[arg(long, default_value = "USD")]
        quotes: String,
    },

    /// Get historical ticker data for a coin [PAID: Starter+]
    #[command(name = "ticker-history", after_help = "EXAMPLES:\n  coinpaprika-cli ticker-history btc-bitcoin --start 2024-01-01\n  coinpaprika-cli ticker-history eth-ethereum --start 2024-01-01 --interval 24h --limit 30")]
    TickerHistory {
        /// Coin ID (e.g., btc-bitcoin)
        coin_id: String,
        /// Start date (ISO format, e.g., 2024-01-01)
        #[arg(long)]
        start: String,
        /// End date (ISO format)
        #[arg(long)]
        end: Option<String>,
        /// Interval (5m, 1h, 24h, 7d, 30d, etc.)
        #[arg(long, default_value = "24h")]
        interval: String,
        /// Maximum number of results
        #[arg(long, default_value = "50")]
        limit: usize,
        /// Quote currency
        #[arg(long, default_value = "usd")]
        quote: String,
    },

    /// Get historical OHLCV data for a coin [PAID: Starter+]
    #[command(after_help = "EXAMPLES:\n  coinpaprika-cli ohlcv btc-bitcoin --start 2024-01-01\n  coinpaprika-cli ohlcv eth-ethereum --start 2024-01-01 --interval 24h --limit 30")]
    Ohlcv {
        /// Coin ID (e.g., btc-bitcoin)
        coin_id: String,
        /// Start date (ISO format, e.g., 2024-01-01)
        #[arg(long)]
        start: String,
        /// End date (ISO format)
        #[arg(long)]
        end: Option<String>,
        /// Interval (5m, 15m, 30m, 1h, 6h, 12h, 24h)
        #[arg(long, default_value = "24h")]
        interval: String,
        /// Maximum number of results
        #[arg(long, default_value = "50")]
        limit: usize,
        /// Quote currency
        #[arg(long, default_value = "usd")]
        quote: String,
    },

    /// Get OHLCV data for the last full day
    #[command(name = "ohlcv-latest", after_help = "EXAMPLES:\n  coinpaprika-cli ohlcv-latest btc-bitcoin")]
    OhlcvLatest {
        /// Coin ID (e.g., btc-bitcoin)
        coin_id: String,
        /// Quote currency
        #[arg(long, default_value = "usd")]
        quote: String,
    },

    /// Get OHLCV data for today (incomplete day)
    #[command(name = "ohlcv-today", after_help = "EXAMPLES:\n  coinpaprika-cli ohlcv-today btc-bitcoin")]
    OhlcvToday {
        /// Coin ID (e.g., btc-bitcoin)
        coin_id: String,
        /// Quote currency
        #[arg(long, default_value = "usd")]
        quote: String,
    },

    /// List exchanges
    #[command(after_help = "EXAMPLES:\n  coinpaprika-cli exchanges --limit 10")]
    Exchanges {
        /// Maximum number of results
        #[arg(long, default_value = "50")]
        limit: usize,
        /// Currency quotes, comma-separated
        #[arg(long, default_value = "USD")]
        quotes: String,
    },

    /// Get detailed info about an exchange
    #[command(after_help = "EXAMPLES:\n  coinpaprika-cli exchange binance")]
    Exchange {
        /// Exchange ID (e.g., binance, coinbase)
        exchange_id: String,
        /// Currency quotes, comma-separated
        #[arg(long, default_value = "USD")]
        quotes: String,
    },

    /// Get markets on an exchange
    #[command(name = "exchange-markets", after_help = "EXAMPLES:\n  coinpaprika-cli exchange-markets binance --limit 10")]
    ExchangeMarkets {
        /// Exchange ID (e.g., binance)
        exchange_id: String,
        /// Maximum number of results
        #[arg(long, default_value = "50")]
        limit: usize,
        /// Currency quotes, comma-separated
        #[arg(long, default_value = "USD")]
        quotes: String,
    },

    /// List tags/categories
    #[command(after_help = "EXAMPLES:\n  coinpaprika-cli tags --limit 20")]
    Tags {
        /// Maximum number of results
        #[arg(long, default_value = "50")]
        limit: usize,
    },

    /// Get details about a tag
    #[command(after_help = "EXAMPLES:\n  coinpaprika-cli tag cryptocurrency")]
    Tag {
        /// Tag ID (e.g., cryptocurrency, defi)
        tag_id: String,
    },

    /// Get details about a person in crypto
    #[command(after_help = "EXAMPLES:\n  coinpaprika-cli person vitalik-buterin")]
    Person {
        /// Person ID (e.g., vitalik-buterin, satoshi-nakamoto)
        person_id: String,
    },

    /// Search for coins, exchanges, people, and tags
    #[command(after_help = "EXAMPLES:\n  coinpaprika-cli search bitcoin\n  coinpaprika-cli search ethereum --categories currencies,exchanges --limit 5")]
    Search {
        /// Search query
        query: String,
        /// Categories to search (currencies,exchanges,icos,people,tags)
        #[arg(long)]
        categories: Option<String>,
        /// Maximum number of results per category
        #[arg(long, default_value = "10")]
        limit: usize,
        /// Search modifier (e.g., symbol_search)
        #[arg(long)]
        modifier: Option<String>,
    },

    /// Convert between two currencies
    #[command(after_help = "EXAMPLES:\n  coinpaprika-cli convert btc-bitcoin eth-ethereum\n  coinpaprika-cli convert btc-bitcoin usd-us-dollars --amount 0.5")]
    Convert {
        /// Base currency ID (e.g., btc-bitcoin)
        base_id: String,
        /// Quote currency ID (e.g., eth-ethereum, usd-us-dollars)
        quote_id: String,
        /// Amount to convert
        #[arg(long, default_value = "1", allow_hyphen_values = true)]
        amount: f64,
    },

    /// List contract platforms
    #[command(after_help = "EXAMPLES:\n  coinpaprika-cli platforms")]
    Platforms,

    /// List contracts on a platform
    #[command(after_help = "EXAMPLES:\n  coinpaprika-cli contracts eth-ethereum --limit 20")]
    Contracts {
        /// Platform ID (e.g., eth-ethereum)
        platform_id: String,
        /// Maximum number of results
        #[arg(long, default_value = "50")]
        limit: usize,
    },

    /// Get ticker data by contract address
    #[command(name = "contract-ticker", after_help = "EXAMPLES:\n  coinpaprika-cli contract-ticker eth-ethereum 0xdac17f958d2ee523a2206206994597c13d831ec7")]
    ContractTicker {
        /// Platform ID (e.g., eth-ethereum)
        platform_id: String,
        /// Contract address
        address: String,
    },

    /// Get historical ticker data by contract [PAID: Starter+]
    #[command(name = "contract-history", after_help = "EXAMPLES:\n  coinpaprika-cli contract-history eth-ethereum 0xdac17f958d2ee523a2206206994597c13d831ec7 --start 2024-01-01")]
    ContractHistory {
        /// Platform ID (e.g., eth-ethereum)
        platform_id: String,
        /// Contract address
        address: String,
        /// Start date (ISO format)
        #[arg(long)]
        start: String,
        /// End date (ISO format)
        #[arg(long)]
        end: Option<String>,
        /// Interval
        #[arg(long, default_value = "24h")]
        interval: String,
        /// Maximum number of results
        #[arg(long, default_value = "50")]
        limit: usize,
    },

    /// Get API key info [PAID: requires API key]
    #[command(name = "key-info")]
    KeyInfo,

    /// Get ID mappings across platforms [PAID: Business+]
    Mappings,

    /// Get changelog of coin ID changes [PAID: Starter+]
    Changelog {
        /// Maximum number of results
        #[arg(long, default_value = "50")]
        limit: usize,
        /// Page number
        #[arg(long, default_value = "1")]
        page: usize,
    },

    /// Manage CLI configuration
    #[command(subcommand)]
    Config(commands::config::ConfigCommand),

    /// Check CoinPaprika API health status and response time
    Status,

    /// Check for CLI updates (compares with latest GitHub release)
    #[command(name = "check-update")]
    CheckUpdate,

    /// Get ready-to-paste attribution snippets for CoinPaprika
    Attribution,

    /// Interactive shell mode (REPL)
    Shell,

    /// Show free tier details and paid plan overview
    Plans,

    /// Interactive setup wizard (configure API key)
    Onboard {
        /// API key to save (skips interactive prompts)
        #[arg(long)]
        key: Option<String>,
    },
}

pub(crate) fn run(cli: Cli) -> std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<()>> + Send>> {
    Box::pin(run_inner(cli))
}

async fn run_inner(cli: Cli) -> anyhow::Result<()> {
    let api_key = config::resolve_api_key(cli.api_key.as_deref());
    let has_api_key = api_key.is_some();
    let client = client::ApiClient::new(api_key);
    let output = cli.output;
    let raw = cli.raw;

    match cli.command {
        Commands::Global => commands::global::execute(&client, output, raw).await,
        Commands::Coins { limit } => commands::coins::execute_list(&client, limit, output, raw).await,
        Commands::Coin { coin_id } => commands::coins::execute_detail(&client, &coin_id, output, raw).await,
        Commands::CoinEvents { coin_id, limit } => commands::coins::execute_events(&client, &coin_id, limit, output, raw).await,
        Commands::CoinExchanges { coin_id, limit } => commands::coins::execute_exchanges(&client, &coin_id, limit, output, raw).await,
        Commands::CoinMarkets { coin_id, quotes, limit } => commands::coins::execute_markets(&client, &coin_id, &quotes, limit, output, raw).await,
        Commands::Tickers { limit, quotes } => commands::tickers::execute_list(&client, limit, &quotes, output, raw).await,
        Commands::Ticker { coin_id, quotes } => commands::tickers::execute_detail(&client, &coin_id, &quotes, output, raw).await,
        Commands::TickerHistory { coin_id, start, end, interval, limit, quote } => {
            commands::tickers::execute_history(&client, &coin_id, &start, end.as_deref(), &interval, limit, &quote, output, raw).await
        }
        Commands::Ohlcv { coin_id, start, end, interval, limit, quote } => {
            commands::ohlcv::execute_historical(&client, &coin_id, &start, end.as_deref(), &interval, limit, &quote, output, raw).await
        }
        Commands::OhlcvLatest { coin_id, quote } => commands::ohlcv::execute_latest(&client, &coin_id, &quote, output, raw).await,
        Commands::OhlcvToday { coin_id, quote } => commands::ohlcv::execute_today(&client, &coin_id, &quote, output, raw).await,
        Commands::Exchanges { limit, quotes } => commands::exchanges::execute_list(&client, limit, &quotes, output, raw).await,
        Commands::Exchange { exchange_id, quotes } => commands::exchanges::execute_detail(&client, &exchange_id, &quotes, output, raw).await,
        Commands::ExchangeMarkets { exchange_id, limit, quotes } => commands::exchanges::execute_markets(&client, &exchange_id, limit, &quotes, output, raw).await,
        Commands::Tags { limit } => commands::tags::execute_list(&client, limit, output, raw).await,
        Commands::Tag { tag_id } => commands::tags::execute_detail(&client, &tag_id, output, raw).await,
        Commands::Person { person_id } => commands::people::execute(&client, &person_id, output, raw).await,
        Commands::Search { query, categories, limit, modifier } => {
            commands::search::execute(&client, &query, categories.as_deref(), limit, modifier.as_deref(), output, raw).await
        }
        Commands::Convert { base_id, quote_id, amount } => commands::convert::execute(&client, &base_id, &quote_id, amount, output, raw).await,
        Commands::Platforms => commands::contracts::execute_platforms(&client, output, raw).await,
        Commands::Contracts { platform_id, limit } => commands::contracts::execute_contracts(&client, &platform_id, limit, output, raw).await,
        Commands::ContractTicker { platform_id, address } => commands::contracts::execute_ticker(&client, &platform_id, &address, output, raw).await,
        Commands::ContractHistory { platform_id, address, start, end, interval, limit } => {
            commands::contracts::execute_history(&client, &platform_id, &address, &start, end.as_deref(), &interval, limit, output, raw).await
        }
        Commands::KeyInfo => commands::api_management::execute_key_info(&client, has_api_key, output, raw).await,
        Commands::Mappings => commands::api_management::execute_mappings(&client, output, raw).await,
        Commands::Changelog { limit, page } => commands::api_management::execute_changelog(&client, limit, page, output, raw).await,
        Commands::Config(cmd) => commands::config::execute(cmd, output, raw).await,
        Commands::Status => commands::status::execute(&client, output, raw).await,
        Commands::CheckUpdate => commands::version::execute(output, raw).await,
        Commands::Attribution => commands::attribution::execute(output, raw),
        Commands::Shell => {
            shell::run_shell().await;
            Ok(())
        }
        Commands::Plans => commands::plans::execute(output, raw),
        Commands::Onboard { key } => commands::onboard::execute(key).await,
    }
}

#[tokio::main]
async fn main() -> ExitCode {
    let cli = Cli::parse();
    let output = cli.output;

    if let Err(e) = run(cli).await {
        match output {
            OutputFormat::Json => {
                println!(
                    "{}",
                    serde_json::json!({"error": e.to_string()})
                );
            }
            OutputFormat::Table => {
                eprintln!("Error: {e}");
            }
        }
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}
