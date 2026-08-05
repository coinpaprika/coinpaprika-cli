use crate::commands::people::Person;
use crate::output::{detail_field, print_coinpaprika_footer, print_detail_table, truncate};

pub fn print_person_detail(person: &Person) {
    let mut rows: Vec<[String; 2]> = Vec::new();
    detail_field!(rows, "Name", person.name.clone().unwrap_or_else(|| "-".into()));
    detail_field!(rows, "ID", person.id.clone().unwrap_or_else(|| "-".into()));
    detail_field!(rows, "Teams", person.teams_count.map(|t| t.to_string()).unwrap_or_else(|| "-".into()));

    if let Some(positions) = &person.positions {
        for pos in positions {
            let label = pos.position.clone().unwrap_or_else(|| "Position".into());
            let value = pos.coin_name.clone().unwrap_or_else(|| "-".into());
            detail_field!(rows, &label, format!("{value} ({})", pos.coin_id.as_deref().unwrap_or("-")));
        }
    }

    if let Some(desc) = &person.description {
        if !desc.is_empty() {
            detail_field!(rows, "Description", truncate(desc, 200));
        }
    }

    print_detail_table(rows);
    print_coinpaprika_footer();
}
