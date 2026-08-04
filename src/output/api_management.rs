use crate::commands::api_management::KeyInfo;
use crate::output::{detail_field, print_coinpaprika_footer, print_detail_table};

pub fn print_key_info(info: &KeyInfo) {
    let mut rows: Vec<[String; 2]> = Vec::new();
    detail_field!(rows, "Plan", info.plan.clone().unwrap_or_else(|| "-".into()));
    if let Some(usage) = &info.usage {
        detail_field!(rows, "Usage", serde_json::to_string_pretty(usage).unwrap_or_else(|_| "-".into()));
    }
    if let Some(msg) = &info.message {
        if !msg.is_empty() {
            detail_field!(rows, "Message", msg.clone());
        }
    }
    print_detail_table(rows);
    print_coinpaprika_footer();
}
