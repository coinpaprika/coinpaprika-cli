use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::client::ApiClient;
use crate::output::OutputFormat;

#[derive(Debug, Deserialize, Serialize)]
pub struct Tag {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub tag_type: Option<String>,
    pub coin_counter: Option<i64>,
    pub ico_counter: Option<i64>,
    pub coins: Option<Vec<String>>,
}

pub async fn execute_list(client: &ApiClient, limit: usize, output: OutputFormat, raw: bool) -> Result<()> {
    let tags: Vec<Tag> = client.coinpaprika_get("/tags", &[]).await?;
    let tags: Vec<Tag> = tags.into_iter().take(limit).collect();
    match output {
        OutputFormat::Table => crate::output::tags::print_tags_table(&tags),
        OutputFormat::Json => {
            crate::output::print_json_wrapped(&tags, crate::output::ResponseMeta::coinpaprika("/tags"), raw)?;
        }
    }
    Ok(())
}

pub async fn execute_detail(client: &ApiClient, tag_id: &str, output: OutputFormat, raw: bool) -> Result<()> {
    let id = crate::idpath::path_segment(tag_id)?;
    let tag: Tag = client.coinpaprika_get(&format!("/tags/{id}"), &[]).await?;
    match output {
        OutputFormat::Table => crate::output::tags::print_tag_detail(&tag),
        OutputFormat::Json => {
            crate::output::print_json_wrapped(&tag, crate::output::ResponseMeta::coinpaprika(&format!("/tag/{tag_id}")), raw)?;
        }
    }
    Ok(())
}
