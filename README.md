# coinpaprika-cli

> For agents: `curl -sSL https://raw.githubusercontent.com/coinpaprika/coinpaprika-cli/main/install.sh | sh`

Crypto market data from your terminal. 12,000+ cryptocurrencies, 350+ exchanges, real-time prices, OHLCV. Free tier, no API key needed to start.

## Quick start

```bash
# Install
curl -sSL https://raw.githubusercontent.com/coinpaprika/coinpaprika-cli/main/install.sh | sh

# Setup (optional, the free tier works without a key)
coinpaprika-cli onboard

# Try it
coinpaprika-cli ticker btc-bitcoin
coinpaprika-cli global
coinpaprika-cli search ethereum
```

## API key setup

Prefer these two: neither one puts the key where other local users can read
it with `ps`.

1. **Config file:** `coinpaprika-cli onboard` prompts for the key (input
   hidden) and saves it to `~/.coinpaprika/config.json` (0600). You can also
   pipe the key in: `echo "$KEY" | coinpaprika-cli config set-key`.
2. **Environment variable:** `export COINPAPRIKA_API_KEY=your-key`

A `--api-key` flag and a `config set-key <KEY>` / `onboard --key <KEY>`
argument form still work for backwards compatibility, but command-line
arguments are visible to every other user on a shared machine through the
process list, so the CLI prints a warning whenever the key arrives that way.

Priority: `--api-key` flag > env var > config file.

## Rate limits & pricing

- **Free tier:** 20,000 calls/mo, no key needed. Run `coinpaprika-cli plans` for full details
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
| `config set-key` | Set API key (prompts if KEY omitted) | `coinpaprika-cli config set-key` |
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
