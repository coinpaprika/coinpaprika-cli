use anyhow::Result;
use clap::Subcommand;
use serde::Serialize;

use crate::output::OutputFormat;

#[derive(Subcommand)]
pub enum ConfigCommand {
    /// Show current configuration
    Show,
    /// Set CoinPaprika API key
    #[command(name = "set-key", after_help = "Omit KEY to be prompted (input hidden), or pipe it on stdin:\n  echo \"$COINPAPRIKA_API_KEY\" | coinpaprika-cli config set-key\nPassing KEY as an argument works for backwards compatibility but is\nvisible to other local users through the process list.")]
    SetKey {
        /// Your CoinPaprika API key (omit to be prompted; visible to other local users if passed here)
        key: Option<String>,
    },
    /// Reset configuration (delete config file)
    Reset,
}

#[derive(Debug, Serialize)]
struct ConfigInfo {
    config_file: String,
    api_key: Option<String>,
    key_source: String,
    coinpaprika_url: String,
}

pub async fn execute(cmd: ConfigCommand, output: OutputFormat, raw: bool) -> Result<()> {
    match cmd {
        ConfigCommand::Show => {
            let key_source = crate::config::key_source(None);
            let api_key = crate::config::resolve_api_key(None);
            let config_path = crate::config::config_path()?;

            match output {
                OutputFormat::Table => {
                    crate::output::config::print_config_show(
                        &config_path.display().to_string(),
                        api_key.as_deref(),
                        key_source,
                    );
                }
                OutputFormat::Json => {
                    let has_key = api_key.is_some();
                    let info = ConfigInfo {
                        config_file: config_path.display().to_string(),
                        api_key: api_key.map(|k| crate::config::mask_key(&k)),
                        key_source: key_source.to_string(),
                        coinpaprika_url: if has_key {
                            "https://api-pro.coinpaprika.com/v1".to_string()
                        } else {
                            "https://api.coinpaprika.com/v1".to_string()
                        },
                    };
                    crate::output::print_json_wrapped(&info, crate::output::ResponseMeta::coinpaprika("/config"), raw)?;
                }
            }
        }
        ConfigCommand::SetKey { key } => {
            let key = match key {
                Some(k) => {
                    eprintln!(
                        "warning: passing the key as an argument exposes it to other local \
                         users on this machine (via `ps`). Omit it next time to be prompted, \
                         or pipe it on stdin."
                    );
                    k.trim().to_string()
                }
                None => crate::config::read_key_interactively("CoinPaprika API key (input hidden): ")?,
            };
            anyhow::ensure!(!key.is_empty(), "no key provided");
            crate::config::save_api_key(&key)?;
            println!("API key saved to {}", crate::config::config_path()?.display());
            println!("Key: {}", crate::config::mask_key(&key));
        }
        ConfigCommand::Reset => {
            crate::config::delete_config()?;
            println!("Configuration deleted.");
        }
    }
    Ok(())
}
