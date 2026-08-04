use tabled::{Table, Tabled};
use tabled::settings::Style;

use crate::commands::search::SearchResult;
use crate::output::{print_coinpaprika_footer, truncate};

#[derive(Tabled)]
struct CurrencyRow {
    #[tabled(rename = "Rank")]
    rank: String,
    #[tabled(rename = "Symbol")]
    symbol: String,
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "ID")]
    id: String,
}

pub fn print_search_results(result: &SearchResult) {
    if let Some(currencies) = &result.currencies {
        if !currencies.is_empty() {
            println!("Currencies:");
            let rows: Vec<CurrencyRow> = currencies.iter().map(|c| CurrencyRow {
                rank: c.rank.map(|r| r.to_string()).unwrap_or_else(|| "-".into()),
                symbol: c.symbol.clone(),
                name: truncate(&c.name, 30),
                id: c.id.clone(),
            }).collect();
            let table = Table::new(rows).with(Style::rounded()).to_string();
            println!("{table}\n");
        }
    }

    if let Some(exchanges) = &result.exchanges {
        if !exchanges.is_empty() {
            println!("Exchanges:");
            for e in exchanges {
                println!("  {} · {}", e.id.as_deref().unwrap_or("-"), e.name.as_deref().unwrap_or("-"));
            }
            println!();
        }
    }

    if let Some(people) = &result.people {
        if !people.is_empty() {
            println!("People:");
            for p in people {
                println!("  {} · {} (teams: {})",
                    p.id.as_deref().unwrap_or("-"),
                    p.name.as_deref().unwrap_or("-"),
                    p.teams_count.unwrap_or(0)
                );
            }
            println!();
        }
    }

    if let Some(tags) = &result.tags {
        if !tags.is_empty() {
            println!("Tags:");
            for t in tags {
                println!("  {} · {} (coins: {})",
                    t.id.as_deref().unwrap_or("-"),
                    t.name.as_deref().unwrap_or("-"),
                    t.coin_counter.unwrap_or(0)
                );
            }
            println!();
        }
    }

    print_coinpaprika_footer();
}
