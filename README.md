# coinpaprika-cli

> For agents: `curl -sSL https://raw.githubusercontent.com/coinpaprika/coinpaprika-cli/main/install.sh | sh`

Free crypto market data from your terminal. 8,000+ coins, real-time prices, OHLCV, exchanges.

## Quick start

```bash
# Install
curl -sSL https://raw.githubusercontent.com/coinpaprika/coinpaprika-cli/main/install.sh | sh

# Setup (optional — free tier works without a key)
coinpaprika-cli onboard

# Try it
coinpaprika-cli ticker btc-bitcoin
coinpaprika-cli global
coinpaprika-cli search ethereum
```

## API key setup (3 options)

1. **Config file:** `coinpaprika-cli onboard` — saves to `~/.coinpaprika/config.json`
2. **Environment variable:** `export COINPAPRIKA_API_KEY=your-key`
3. **CLI flag:** `coinpaprika-cli --api-key your-key ticker btc-bitcoin`

Priority: CLI flag > env var > config file.

## Rate limits & pricing

- **Free tier:** 20,000 calls/mo, no key needed — run `coinpaprika-cli plans` for full details
- **Starter/Business/Enterprise:** [coinpaprika.com/api/pricing](https://coinpaprika.com/api/pricing)

## All commands

| Command | Description | Example |
|---------|-------------|---------|
| `global` | Market overview | `coinpaprika-cli global` |
| `coins` | List coins | `coinpaprika-cli coins --limit 10` |
| `coin` | Coin details | `coinpaprika-cli coin btc-bitcoin` |
| `coin-events` | Coin events | `coinpaprika-cli coin-events btc-bitcoin` |
| `coin-exchanges` | Coin exchanges | `coinpaprika-cli coin-exchanges btc-bitcoin` |
| `coin-markets` | Coin markets | `coinpaprika-cli coin-markets btc-bitcoin` |
| `tickers` | All tickers | `coinpaprika-cli tickers --limit 20` |
| `ticker` | Single ticker | `coinpaprika-cli ticker btc-bitcoin` |
| `ticker-history` | Historical tickers [Starter+] | `coinpaprika-cli ticker-history btc-bitcoin --start 2024-01-01` |
| `ohlcv` | Historical OHLCV [Starter+] | `coinpaprika-cli ohlcv btc-bitcoin --start 2024-01-01` |
| `ohlcv-latest` | Last full day OHLCV | `coinpaprika-cli ohlcv-latest btc-bitcoin` |
| `ohlcv-today` | Today's OHLCV | `coinpaprika-cli ohlcv-today btc-bitcoin` |
| `exchanges` | List exchanges | `coinpaprika-cli exchanges --limit 10` |
| `exchange` | Exchange details | `coinpaprika-cli exchange binance` |
| `exchange-markets` | Exchange markets | `coinpaprika-cli exchange-markets binance` |
| `tags` | List tags | `coinpaprika-cli tags` |
| `tag` | Tag details | `coinpaprika-cli tag defi` |
| `person` | Person details | `coinpaprika-cli person vitalik-buterin` |
| `search` | Search everything | `coinpaprika-cli search bitcoin` |
| `convert` | Currency conversion | `coinpaprika-cli convert btc-bitcoin usd-us-dollars` |
| `platforms` | Contract platforms | `coinpaprika-cli platforms` |
| `contracts` | Platform contracts | `coinpaprika-cli contracts eth-ethereum` |
| `contract-ticker` | Ticker by contract | `coinpaprika-cli contract-ticker eth-ethereum 0xdac...` |
| `contract-history` | Contract history [Starter+] | `coinpaprika-cli contract-history eth-ethereum 0xdac... --start 2024-01-01` |
| `key-info` | API key info [Paid] | `coinpaprika-cli key-info` |
| `mappings` | ID mappings [Business+] | `coinpaprika-cli mappings` |
| `changelog` | Coin ID changelog [Starter+] | `coinpaprika-cli changelog` |
| `config show` | Show config | `coinpaprika-cli config show` |
| `config set-key` | Set API key | `coinpaprika-cli config set-key <KEY>` |
| `config reset` | Delete config | `coinpaprika-cli config reset` |
| `plans` | Free tier details & paid overview | `coinpaprika-cli plans` |
| `status` | API health check | `coinpaprika-cli status` |
| `attribution` | Attribution snippets | `coinpaprika-cli attribution` |
| `onboard` | Setup wizard | `coinpaprika-cli onboard` |
| `shell` | Interactive REPL | `coinpaprika-cli shell` |

## Output formats

```bash
# Table (default)
coinpaprika-cli ticker btc-bitcoin

# JSON with metadata
coinpaprika-cli --output json ticker btc-bitcoin

# Raw JSON (no _meta wrapper, for piping)
coinpaprika-cli --output json --raw ticker btc-bitcoin
```

## Links

- API docs: https://api.coinpaprika.com
- Pricing: https://coinpaprika.com/api/pricing
- GitHub: https://github.com/coinpaprika/coinpaprika-cli
