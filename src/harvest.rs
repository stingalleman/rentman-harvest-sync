use reqwest::Error;
use serde::{Deserialize, Serialize};

pub struct HarvestClient {
    token: String,
    account_id: String,
    user_agent: String,
}

impl HarvestClient {
    pub fn new(token: String, account_id: String, user_agent: String) -> Self {
        Self {
            token,
            account_id,
            user_agent,
        }
    }

    pub async fn get_clients(&self) -> Result<Clients, Error> {
        let client = reqwest::Client::new();

        let res = client
            .get("https://api.harvestapp.com/v2/clients")
            .header("Authorization", format!("Bearer {}", &self.token))
            .header("Harvest-Account-Id", &self.account_id)
            .header("User-Agent", &self.user_agent)
            .send()
            .await?;

        let json = res.json::<Clients>().await?;

        // Pagination is not currently implemented
        if json.total_entries >= json.per_page {
            panic!("Not all clients are fetched");
        }

        return Ok(json);
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Clients {
    pub clients: Vec<Client>,
    pub per_page: i64,
    pub total_pages: i64,
    pub total_entries: i64,
    pub next_page: Option<serde_json::Value>,
    pub previous_page: Option<serde_json::Value>,
    pub page: i64,
    pub links: Links,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Client {
    pub id: i64,
    pub name: String,
    pub is_active: bool,
    // ookwel Rentman ID
    pub address: Option<String>,
    pub statement_key: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Links {
    pub first: String,
    pub next: Option<serde_json::Value>,
    pub previous: Option<serde_json::Value>,
    pub last: String,
}
