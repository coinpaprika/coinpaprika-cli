use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::client::ApiClient;
use crate::output::OutputFormat;

#[derive(Debug, Deserialize, Serialize)]
pub struct Person {
    pub id: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub teams_count: Option<i64>,
    pub links: Option<HashMap<String, serde_json::Value>>,
    pub positions: Option<Vec<Position>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Position {
    pub coin_id: Option<String>,
    pub coin_name: Option<String>,
    pub position: Option<String>,
}

pub async fn execute(client: &ApiClient, person_id: &str, output: OutputFormat, raw: bool) -> Result<()> {
    let id = crate::idpath::path_segment(person_id)?;
    let person: Person = client.coinpaprika_get(&format!("/people/{id}"), &[]).await?;
    match output {
        OutputFormat::Table => crate::output::people::print_person_detail(&person),
        OutputFormat::Json => {
            crate::output::print_json_wrapped(&person, crate::output::ResponseMeta::coinpaprika(&format!("/people/{person_id}")), raw)?;
        }
    }
    Ok(())
}
