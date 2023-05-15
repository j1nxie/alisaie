use ffxiv_types::World;
use serde::Deserialize;
use url::Url;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SearchCharacter {
    #[serde(rename = "ID")]
    pub id: u64,
    pub name: String,
    pub server: World,
    pub lang: String,
    pub avatar: Url,
    pub rank: Option<serde_json::Value>,
    pub rank_icon: Option<serde_json::Value>,
    pub feast_matches: u64,
}
