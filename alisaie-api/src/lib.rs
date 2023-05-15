use reqwest::Client;

pub mod error;
pub mod models;

#[derive(Debug, Default)]
pub struct XivApi {
    client: Client,
    key: Option<String>,
}

impl XivApi {
    pub fn new() -> Self {
        let client = Client::new();
        Self { client, key: None }
    }

    pub fn with_key<S: AsRef<str>>(key: S) -> Self {
        let client = Client::new();
        let key = Some(key.as_ref().to_string());
        Self { client, key }
    }
}
