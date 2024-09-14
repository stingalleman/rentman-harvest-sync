use reqwest::Error;
use serde::{Deserialize, Serialize};

pub struct HarvestClient {
    token: String,
    pub account_id: String,
    pub user_agent: String,
    pub nvt_client: i64,
}

impl HarvestClient {
    pub fn new(token: String, account_id: String, user_agent: String, nvt_client: i64) -> Self {
        Self {
            token,
            account_id,
            user_agent,
            nvt_client,
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

    pub async fn create_client(
        &self,
        create_client: CreateClient,
    ) -> Result<reqwest::Response, Error> {
        let client = reqwest::Client::new();

        client
            .post("https://api.harvestapp.com/v2/clients")
            .header("Authorization", format!("Bearer {}", &self.token))
            .header("Harvest-Account-Id", &self.account_id)
            .header("Content-Type", "application/json")
            .header("User-Agent", &self.user_agent)
            .json(&create_client)
            .send()
            .await
    }

    pub async fn get_projects(&self) -> Result<Projects, Error> {
        let client = reqwest::Client::new();

        let res = client
            .get("https://api.harvestapp.com/v2/projects")
            .header("Authorization", format!("Bearer {}", &self.token))
            .header("Harvest-Account-Id", &self.account_id)
            .header("User-Agent", &self.user_agent)
            .send()
            .await?;

        let json = res.json::<Projects>().await?;

        // Pagination is not currently implemented
        if json.total_entries >= json.per_page {
            panic!("Not all clients are fetched");
        }

        return Ok(json);
    }

    pub async fn create_project(
        &self,
        create_project: CreateProject,
    ) -> Result<reqwest::Response, Error> {
        let client = reqwest::Client::new();

        client
            .post("https://api.harvestapp.com/v2/projects")
            .header("Authorization", format!("Bearer {}", &self.token))
            .header("Harvest-Account-Id", &self.account_id)
            .header("Content-Type", "application/json")
            .header("User-Agent", &self.user_agent)
            .json(&create_project)
            .send()
            .await
    }
}

// client types

#[derive(Serialize, Deserialize)]
pub struct CreateClient {
    pub name: String,
    /// ookwel Rentman ID
    pub address: String,
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
    /// ookwel Rentman ID
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

// projects types
#[derive(Serialize, Deserialize)]
pub struct CreateProject {
    pub client_id: i64,
    pub name: String,
    pub notes: String,
    pub code: String,
    pub is_active: bool,
    #[serde(default = "is_billable")]
    pub is_billable: bool,
    #[serde(default = "default_bill_by")]
    pub bill_by: String,
    #[serde(default = "budget_by")]
    pub budget_by: String,
}

fn default_bill_by() -> String {
    "none".to_string()
}

fn budget_by() -> String {
    "none".to_string()
}

fn is_billable() -> bool {
    true
}

#[derive(Serialize, Deserialize)]
pub struct Projects {
    pub projects: Vec<Project>,
    pub per_page: i64,
    pub total_pages: i64,
    pub total_entries: i64,
    pub next_page: Option<serde_json::Value>,
    pub previous_page: Option<serde_json::Value>,
    pub page: i64,
    pub links: Links,
}

#[derive(Serialize, Deserialize)]
pub struct Project {
    pub id: i64,
    pub name: String,
    pub code: Option<String>,
    pub is_active: bool,
    pub notes: Option<String>,
    pub client: ProjectClient,
}

#[derive(Serialize, Deserialize)]
pub struct ProjectClient {
    pub id: i64,
    pub name: String,
}
