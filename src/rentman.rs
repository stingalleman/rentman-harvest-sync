use reqwest::Error;
use serde::{Deserialize, Serialize};

pub struct RentmanClient {
    token: String,
}

impl RentmanClient {
    pub fn new(token: String) -> Self {
        Self { token }
    }

    pub async fn get_contacts(&self) -> Result<Contacts, Error> {
        let client = reqwest::Client::new();

        let res = client
            .get("https://api.rentman.net/contacts")
            .header("Authorization", format!("Bearer {}", &self.token))
            .send()
            .await?
            .json::<Contacts>()
            .await?;

        if res.item_count >= res.limit {
            panic!("Not all contacts are fetched");
        }

        return Ok(res);
    }

    pub async fn get_projects(&self) -> Result<Projects, Error> {
        let client = reqwest::Client::new();

        let mut res = client
            .get("https://api.rentman.net/projects")
            .header("Authorization", format!("Bearer {}", &self.token))
            .send()
            .await?
            .json::<Projects>()
            .await?;

        if res.item_count >= res.limit {
            panic!("Not all projects are fetched");
        }

        for project in &mut res.data {
            project.name = project.name.trim_end().to_string();

            project.customer_id = project
                .customer
                .clone()
                .unwrap_or("0".to_string())
                .replace("/contacts/", "")
                .parse::<i64>()
                .unwrap_or(0);
        }

        return Ok(res);
    }

    pub async fn get_statuses(&self) -> Result<Statuses, Error> {
        let client = reqwest::Client::new();

        let res = client
            .get("https://api.rentman.net/statuses")
            .header("Authorization", format!("Bearer {}", &self.token))
            .send()
            .await?
            .json::<Statuses>()
            .await?;

        if res.item_count >= res.limit {
            panic!("Not all statuses are fetched");
        }

        return Ok(res);
    }
}

//
// Contacts
//

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Contacts {
    pub data: Vec<ContactsData>,
    pub item_count: i64,
    pub limit: i64,
    pub offset: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContactsData {
    pub id: i64,
    pub created: String,
    pub modified: String,
    pub displayname: String,
    pub firstname: String,
    pub surfix: String,
    pub surname: String,
    pub code: String,
    pub name: String,
}

//
// Projects
//
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Projects {
    pub data: Vec<ProjectsData>,
    pub item_count: i64,
    pub limit: i64,
    pub offset: i64,
}

#[derive(Serialize, Deserialize)]
pub struct ProjectsData {
    pub id: i64,
    pub displayname: String,
    pub customer: Option<String>,
    /// formated from `Customer` without `/customer/`, if `customer` is None, `customerId` is `0`.
    #[serde(skip_deserializing)]
    pub customer_id: i64,
    pub cust_contact: Option<String>,
    pub name: String,
    pub reference: String,
    /// Powerproductions ID
    pub number: i64,
    pub tags: String,
}

//
// Statuses
//

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Statuses {
    pub data: Vec<StatusesData>,
    pub item_count: i64,
    pub limit: i64,
    pub offset: i64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusesData {
    pub id: i64,
    pub displayname: String,
    pub name: String,
}
