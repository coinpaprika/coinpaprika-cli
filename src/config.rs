use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::IsTerminal;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub api_key: Option<String>,
}

pub fn config_dir() -> Result<PathBuf> {
    let home = dirs::home_dir().context("Could not determine home directory")?;
    Ok(home.join(".coinpaprika"))
}

pub fn config_path() -> Result<PathBuf> {
    Ok(config_dir()?.join("config.json"))
}

#[allow(dead_code)]
pub fn config_exists() -> bool {
    config_path().map(|p| p.exists()).unwrap_or(false)
}

pub fn load_config() -> Result<Config> {
    let path = config_path()?;
    if !path.exists() {
        return Ok(Config { api_key: None });
    }
    let contents = fs::read_to_string(&path)
        .with_context(|| format!("Failed to read config file at {}", path.display()))?;
    let config: Config = serde_json::from_str(&contents)
        .with_context(|| "Failed to parse config file")?;
    Ok(config)
}

pub fn save_api_key(key: &str) -> Result<()> {
    let dir = config_dir()?;
    fs::create_dir_all(&dir)?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&dir, fs::Permissions::from_mode(0o700))?;
    }

    let config = Config {
        api_key: Some(key.to_string()),
    };
    let json = serde_json::to_string_pretty(&config)?;
    let path = config_path()?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        use std::io::Write;
        let mut file = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .mode(0o600)
            .open(&path)?;
        file.write_all(json.as_bytes())?;
    }

    #[cfg(not(unix))]
    fs::write(&path, &json)?;

    Ok(())
}

pub fn delete_config() -> Result<()> {
    let dir = config_dir()?;
    if dir.exists() {
        fs::remove_dir_all(&dir)?;
    }
    Ok(())
}

/// Resolve API key with priority: CLI flag > env var > config file
pub fn resolve_api_key(cli_key: Option<&str>) -> Option<String> {
    // 1. CLI flag
    if let Some(key) = cli_key {
        if !key.is_empty() {
            eprintln!(
                "warning: the API key came from --api-key, which exposes it to other local \
                 users on this machine (via `ps`). Prefer COINPAPRIKA_API_KEY, the config \
                 file (coinpaprika-cli onboard), or piping the key on stdin to `config set-key`."
            );
            return Some(key.to_string());
        }
    }

    // 2. Environment variable
    if let Ok(key) = std::env::var("COINPAPRIKA_API_KEY") {
        if !key.is_empty() {
            return Some(key);
        }
    }

    // 3. Config file
    if let Ok(config) = load_config() {
        if let Some(key) = config.api_key {
            if !key.is_empty() {
                return Some(key);
            }
        }
    }

    None
}

pub fn mask_key(key: &str) -> String {
    if key.len() <= 8 {
        return "****".to_string();
    }
    format!("{}...{}", &key[..4], &key[key.len() - 4..])
}

/// Read an API key from a private channel: a no-echo prompt when stdin is
/// an interactive terminal, or a single line from stdin when it is piped
/// (a script or an agent feeding the key in). Either way the key never
/// appears as a command-line argument, so it is not visible to other local
/// users through the process list.
pub fn read_key_interactively(prompt: &str) -> Result<String> {
    let line = if std::io::stdin().is_terminal() {
        rpassword::prompt_password(prompt).context("failed to read the key from the terminal")?
    } else {
        let mut buf = String::new();
        std::io::stdin()
            .read_line(&mut buf)
            .context("failed to read the key from stdin")?;
        buf
    };
    Ok(line.trim().to_string())
}

pub fn key_source(cli_key: Option<&str>) -> &'static str {
    if let Some(k) = cli_key {
        if !k.is_empty() {
            return "CLI flag (--api-key)";
        }
    }
    if let Ok(k) = std::env::var("COINPAPRIKA_API_KEY") {
        if !k.is_empty() {
            return "Environment variable (COINPAPRIKA_API_KEY)";
        }
    }
    if let Ok(config) = load_config() {
        if config.api_key.is_some() {
            return "Config file (~/.coinpaprika/config.json)";
        }
    }
    "Not set (using free tier)"
}
