use crate::commands::convert::ConvertResult;
use crate::output::{detail_field, print_coinpaprika_footer, print_detail_table};

pub fn print_convert_result(result: &ConvertResult) {
    let mut rows: Vec<[String; 2]> = Vec::new();
    detail_field!(rows, "From", format!("{} ({})",
        result.base_currency_name.as_deref().unwrap_or("-"),
        result.base_currency_id.as_deref().unwrap_or("-")
    ));
    detail_field!(rows, "To", format!("{} ({})",
        result.quote_currency_name.as_deref().unwrap_or("-"),
        result.quote_currency_id.as_deref().unwrap_or("-")
    ));
    detail_field!(rows, "Amount", result.amount.map(|a| a.to_string()).unwrap_or_else(|| "-".into()));
    detail_field!(rows, "Price", result.price.map(|p| format!("{p}")).unwrap_or_else(|| "-".into()));
    print_detail_table(rows);
    print_coinpaprika_footer();
}
