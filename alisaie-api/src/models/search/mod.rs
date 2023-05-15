use serde::Deserialize;

use self::character::SearchCharacter;

pub mod character;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum SearchModel {
    Character(SearchCharacter),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SearchResult {
    pub pagination: Pagination,
    pub result: Vec<SearchModel>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Pagination {
    pub page: u64,
    pub page_next: Option<u64>,
    pub page_prev: Option<u64>,
    pub page_total: u64,
    pub results: u64,
    pub results_per_page: u64,
    pub results_total: u64,
}
