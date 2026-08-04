use tabled::{Table, Tabled};
use tabled::settings::Style;

use crate::commands::tags::Tag;
use crate::output::{detail_field, print_coinpaprika_footer, print_detail_table, truncate};

#[derive(Tabled)]
struct TagRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Type")]
    tag_type: String,
    #[tabled(rename = "Coins")]
    coins: String,
    #[tabled(rename = "ICOs")]
    icos: String,
}

pub fn print_tags_table(tags: &[Tag]) {
    let rows: Vec<TagRow> = tags.iter().map(|t| TagRow {
        id: t.id.clone(),
        name: truncate(&t.name, 35),
        tag_type: t.tag_type.clone().unwrap_or_else(|| "-".into()),
        coins: t.coin_counter.map(|c| c.to_string()).unwrap_or_else(|| "-".into()),
        icos: t.ico_counter.map(|c| c.to_string()).unwrap_or_else(|| "-".into()),
    }).collect();

    let table = Table::new(rows).with(Style::rounded()).to_string();
    println!("{table}");
    print_coinpaprika_footer();
}

pub fn print_tag_detail(tag: &Tag) {
    let mut rows: Vec<[String; 2]> = Vec::new();
    detail_field!(rows, "ID", tag.id.clone());
    detail_field!(rows, "Name", tag.name.clone());
    detail_field!(rows, "Type", tag.tag_type.clone().unwrap_or_else(|| "-".into()));
    detail_field!(rows, "Coins", tag.coin_counter.map(|c| c.to_string()).unwrap_or_else(|| "-".into()));
    detail_field!(rows, "ICOs", tag.ico_counter.map(|c| c.to_string()).unwrap_or_else(|| "-".into()));
    if let Some(desc) = &tag.description {
        if !desc.is_empty() {
            detail_field!(rows, "Description", truncate(desc, 200));
        }
    }
    print_detail_table(rows);
    print_coinpaprika_footer();
}
