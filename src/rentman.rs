use reqwest::Error;
use serde::{Deserialize, Serialize};

pub struct RentmanClient {
    token: String,
    pub btdb_id: i64,
}

impl RentmanClient {
    pub fn new(token: String, btdb_id: i64) -> Self {
        Self { token, btdb_id }
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

        Ok(res)
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

        Ok(res)
    }

    // pub async fn get_statuses(&self) -> Result<Vec<StatusesData>, Error> {
    //     let client = reqwest::Client::new();

    //     let res = client
    //         .get("https://api.rentman.net/statuses")
    //         .header("Authorization", format!("Bearer {}", &self.token))
    //         .send()
    //         .await?
    //         .json::<Statuses>()
    //         .await?;

    //     if res.item_count >= res.limit {
    //         panic!("Not all statuses are fetched");
    //     }

    //     return Ok(res.data);
    // }

    pub async fn get_subprojects(&self) -> Result<Vec<SubprojectData>, Error> {
        let client = reqwest::Client::new();

        let mut data: Vec<SubprojectData> = vec![];
        let mut offset = 0;

        loop {
            let mut res = client
                .get(format!(
                    "https://api.rentman.net/subprojects?offset={}",
                    offset
                ))
                .header("Authorization", format!("Bearer {}", &self.token))
                .send()
                .await?
                .json::<Subprojects>()
                .await?;

            // If no more data, break
            if res.item_count == 0 {
                break;
            }

            // Offset `offset` variable by length of `data`
            offset += res.item_count;

            for subproject in &mut res.data {
                subproject.project_id = subproject
                    .project
                    .clone()
                    .replace("/projects/", "")
                    .parse::<i64>()
                    .expect("Subproject project_id parse error");
            }

            data.append(&mut res.data);
        }

        Ok(data)
    }

    // pub async fn get_equipment(&self) -> Result<Vec<EquipmentData>, Error> {
    //     let client = reqwest::Client::new();

    //     let mut data: Vec<EquipmentData> = vec![];
    //     let mut offset = 0;

    //     loop {
    //         let mut res = client
    //             .get(format!(
    //                 "https://api.rentman.net/equipment?offset={}",
    //                 offset
    //             ))
    //             .header("Authorization", format!("Bearer {}", &self.token))
    //             .send()
    //             .await?
    //             .json::<Equipment>()
    //             .await?;

    //         // If no more data, break
    //         if res.item_count == 0 {
    //             break;
    //         }

    //         // Offset `offset` variable by length of `data`
    //         offset += res.item_count;

    //         data.append(&mut res.data);
    //     }

    // return Ok(data);
    // }
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
    pub name: String,
    /// Powerproductions ID
    pub number: i64,
    pub tags: String,
    pub planperiod_start: Option<String>,
}

//
// Statuses
//

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Statuses {
    pub data: Vec<StatusesData>,
    pub item_count: i64,
    pub limit: i64,
    pub offset: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StatusesData {
    pub id: i64,
    pub displayname: String,
    pub name: String,
}

//
// Subprojects
//

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subprojects {
    pub data: Vec<SubprojectData>,
    pub item_count: i64,
    pub limit: i64,
    pub offset: i64,
}

#[derive(Serialize, Deserialize)]
pub struct SubprojectData {
    pub id: i64,
    pub displayname: String,
    pub project: String,
    /// Formatted from `project` without `/projects/`.
    #[serde(skip_deserializing)]
    pub project_id: i64,
    pub order: i64,
    pub name: String,
    pub status: Status,
    pub is_template: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum Status {
    #[serde(rename = "/statuses/1")]
    Optie,
    #[serde(rename = "/statuses/2")]
    Geannuleerd,
    #[serde(rename = "/statuses/3")]
    Bevestigd,
    #[serde(rename = "/statuses/4")]
    Klaargezet,
    #[serde(rename = "/statuses/5")]
    OpLocatie,
    #[serde(rename = "/statuses/6")]
    Retour,
    #[serde(rename = "/statuses/7")]
    Aanvraag,
    #[serde(rename = "/statuses/8")]
    Concept,
    #[serde(rename = "/statuses/9")]
    Factureren,
}

//
// Equipment
//

// #[derive(Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Equipment {
//     data: Vec<EquipmentData>,
//     item_count: i64,
//     limit: i64,
//     offset: i64,
// }

// #[derive(Serialize, Deserialize)]
// pub struct EquipmentData {
//     id: i64,
//     created: String,
//     displayname: String,
//     code: String,
//     name: String,
// }
