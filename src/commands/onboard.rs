use anyhow::Result;

pub async fn execute(key: Option<String>) -> Result<()> {
    // Non-interactive mode: validate then save the key
    if let Some(key) = key {
        eprintln!(
            "warning: --key exposes the API key to other local users on this machine (via \
             `ps`). Omit it to be prompted instead, or pipe the key on stdin."
        );
        println!("Validating key...");
        let client = crate::client::ApiClient::new(Some(key.clone()));
        match client.coinpaprika_get::<serde_json::Value>("/key/info", &[]).await {
            Ok(info) => {
                crate::config::save_api_key(&key)?;
                let plan = info.get("plan")
                    .and_then(|p| p.as_str())
                    .unwrap_or("unknown");
                println!("Key validated! Plan: {plan}");
                println!("Saved to {}", crate::config::config_path()?.display());
                println!("Key: {}", crate::config::mask_key(&key));
            }
            Err(_) => {
                println!("Could not validate key (it may still work). Saving anyway.");
                crate::config::save_api_key(&key)?;
                println!("Saved to {}", crate::config::config_path()?.display());
                println!("Key: {}", crate::config::mask_key(&key));
            }
        }
        println!("\nYou're all set! Try these commands:");
        println!("  coinpaprika-cli ticker btc-bitcoin");
        println!("  coinpaprika-cli global");
        println!("  coinpaprika-cli search ethereum");
        return Ok(());
    }

    // Interactive mode
    println!("┌─────────────────────────────────────────────────┐");
    println!("│  Welcome to coinpaprika-cli!                    │");
    println!("│  Crypto market data for developers & AI agents  │");
    println!("└─────────────────────────────────────────────────┘");
    println!();
    println!("Do you have a CoinPaprika API key? (y/n)");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let input = input.trim().to_lowercase();

    if input == "y" || input == "yes" {
        let key = crate::config::read_key_interactively("\nPaste your API key (input hidden): ")?;

        if key.is_empty() {
            println!("No key entered. You can set it later with:");
            println!("  coinpaprika-cli onboard");
            println!("  (or set COINPAPRIKA_API_KEY in your environment)");
            return Ok(());
        }

        // Validate by calling /key-info
        println!("Validating key...");
        let client = crate::client::ApiClient::new(Some(key.clone()));
        match client.coinpaprika_get::<serde_json::Value>("/key/info", &[]).await {
            Ok(info) => {
                crate::config::save_api_key(&key)?;
                let plan = info.get("plan")
                    .and_then(|p| p.as_str())
                    .unwrap_or("unknown");
                println!("Key validated! Plan: {plan}");
                println!("Saved to {}", crate::config::config_path()?.display());
            }
            Err(_) => {
                println!("Could not validate key (it may still work). Saving anyway.");
                crate::config::save_api_key(&key)?;
                println!("Saved to {}", crate::config::config_path()?.display());
            }
        }
    } else {
        println!();
        println!("No problem! The free tier works without an API key.");
        println!("20,000 calls/month, 25+ endpoints, 2,000 assets.");
        println!();
        println!("Run coinpaprika-cli plans to see exactly what's included.");
        println!();
        println!("For historical data, ticker history, and higher limits:");
        println!("  https://coinpaprika.com/api/pricing");
    }

    println!();
    println!("You're all set! Try these commands:");
    println!("  coinpaprika-cli ticker btc-bitcoin      # get Bitcoin price");
    println!("  coinpaprika-cli global                   # market overview");
    println!("  coinpaprika-cli search ethereum          # find coins");
    println!();

    Ok(())
}
