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
}

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
    pub accounting_code: String,
    pub name: String,
    pub gender: String,
    #[serde(rename = "updateHash")]
    pub update_hash: String,
}
