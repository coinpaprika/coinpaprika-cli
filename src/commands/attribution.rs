use anyhow::Result;
use serde::Serialize;
use crate::output::OutputFormat;

#[derive(Serialize)]
struct AttributionData {
    name: String,
    url: String,
    api: String,
    html: String,
    markdown: String,
    badge: String,
}

pub fn execute(output: OutputFormat, raw: bool) -> Result<()> {
    match output {
        OutputFormat::Table => {
            println!("  ─── CoinPaprika attribution snippets (copy & paste) ───\n");
            println!("  HTML:");
            println!("    <a href=\"https://coinpaprika.com\">Powered by CoinPaprika</a>\n");
            println!("  React/JSX:");
            println!("    <a href=\"https://coinpaprika.com\" target=\"_blank\" rel=\"noopener\">");
            println!("      Powered by CoinPaprika");
            println!("    </a>\n");
            println!("  Markdown:");
            println!("    [Powered by CoinPaprika](https://coinpaprika.com)\n");
            println!("  Plain text:");
            println!("    Data provided by CoinPaprika (https://coinpaprika.com)\n");
            println!("  GitHub README badge:");
            println!("    [![CoinPaprika](https://img.shields.io/badge/data-CoinPaprika-green)](https://coinpaprika.com)\n");
            println!("  Attribution is appreciated, not required.");
            println!("  API: api.coinpaprika.com");
        }
        OutputFormat::Json => {
            let data = AttributionData {
                name: "CoinPaprika".into(),
                url: "https://coinpaprika.com".into(),
                api: "https://api.coinpaprika.com".into(),
                html: "<a href=\"https://coinpaprika.com\">Powered by CoinPaprika</a>".into(),
                markdown: "[Powered by CoinPaprika](https://coinpaprika.com)".into(),
                badge: "https://img.shields.io/badge/data-CoinPaprika-green".into(),
            };
            crate::output::print_json_wrapped(&data, crate::output::ResponseMeta::coinpaprika("/attribution"), raw)?;
        }
    }
    Ok(())
}
