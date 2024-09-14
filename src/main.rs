use harvest::{CreateClient, HarvestClient};
use rentman::RentmanClient;

mod harvest;
mod rentman;

async fn update_clients(harvest: &HarvestClient, rentman: &RentmanClient) {
    // Get Harvest clients
    let clients = match harvest.get_clients().await {
        Ok(x) => x,
        Err(e) => {
            panic!("Error getting clients: {}", e);
        }
    };

    // Get Rentman contacts
    let contacts = match rentman.get_contacts().await {
        Ok(x) => x,
        Err(e) => {
            panic!("Error getting contacts: {}", e);
        }
    };

    // Vec to store missing clients
    let mut missing_clients: Vec<MissingClient> = vec![];

    // Loop thru Rentman contacts
    for contact in contacts.data {
        // Loop thru Harvest clients to check if client is found
        let found_client = clients.clients.iter().find(|x| {
            let harvest_rentman_id = match x.address.as_ref() {
                Some(x) => x,
                None => {
                    println!("heeft geen address");
                    return false;
                }
            }
            .parse::<i64>()
            .unwrap_or(0);

            harvest_rentman_id == contact.id
        });

        // Client is found, check for updates, then continue to next contact
        if found_client.is_some() {
            // TODO: check updates

            continue;
        }

        // Push missing client to vec
        missing_clients.push(MissingClient {
            address: contact.id.to_string(),
            name: contact.name,
        })
    }

    // Insert missing clients
    for client in missing_clients {
        println!("Creating client: {:?}", client);

        harvest
            .create_client(CreateClient {
                name: client.name,
                address: client.address,
            })
            .await
            .unwrap();
    }
}

async fn update_projects(harvest: &HarvestClient, rentman: &RentmanClient) {
    // Get Harvest projects
    let harvest_projects = match harvest.get_projects().await {
        Ok(x) => x,
        Err(e) => {
            panic!("Error getting Harvest Projects: {}", e);
        }
    };

    // Get Rentman projects
    let rentman_projects = rentman
        .get_projects()
        .await
        .expect("Can't fetch Rentman projects");

    let rentman_subprojects = rentman
        .get_subprojects()
        .await
        .expect("Can't fetch Rentman subprojects");

    let mut missing_projects: Vec<MissingProject> = vec![];

    for rentman_project in rentman_projects.data {
        // If project is template, skip
        if rentman_project.name.to_lowercase().contains("template") {
            continue;
        }

        // If project is BTDB, skip
        if rentman_project.customer_id == rentman.btdb_id {
            continue;
        }

        // Match subproject to project
        let mut statuses: Vec<&rentman::Status> = vec![];

        let mut is_template = false;
        for subproject in &rentman_subprojects {
            if subproject.project_id != rentman_project.id {
                continue;
            }

            statuses.push(&subproject.status);

            is_template = subproject.is_template;
        }

        // Check state

        let mut is_active = true;
        if !statuses.is_empty() {
            let first = statuses.first().unwrap();

            if statuses.iter().all(|x| x == first) {
                match first {
                    rentman::Status::Geannuleerd => is_active = false,
                    rentman::Status::Retour => is_active = false,
                    _ => {}
                }
            }
        }

        // if project contains template, skip
        if is_template {
            continue;
        }

        // Loop thru Harvest projects to check if project is found
        let found_project = harvest_projects.projects.iter().find(|x| {
            let harvest_rentman_id = match x.notes.as_ref() {
                Some(x) => x,
                None => {
                    println!("heeft geen notes");
                    return false;
                }
            }
            .parse::<i64>()
            .unwrap_or(0);

            harvest_rentman_id == rentman_project.id
        });

        // Project is found, Check for updates, then continue.
        if found_project.is_some() {
            // TODO: check updates

            continue;
        }

        // Push missing project to vec
        missing_projects.push(MissingProject {
            is_active,
            rentman_id: rentman_project.id.to_string(),
            name: rentman_project.name,
            rentman_client_id: rentman_project.customer_id,
            pp_id: rentman_project.number.to_string(),
        })
    }

    // Get Harvest clients
    let clients = match harvest.get_clients().await {
        Ok(x) => x,
        Err(e) => {
            panic!("Error getting clients: {}", e);
        }
    };

    for project in missing_projects {
        let client_id: i64;

        if project.rentman_client_id == 0 {
            client_id = harvest.nvt_client;
        } else {
            // Find correct Harvest client
            let client = clients.clients.iter().find(|x| {
                if x.address
                    .as_ref()
                    .is_some_and(|v| v == &project.rentman_client_id.to_string())
                {
                    return true;
                };

                false
            });

            if client.is_none() {
                println!("Client not found for project: {}", project.name);
                continue;
            }

            client_id = client.unwrap().id;
        }

        println!("Creating project: {:?} ({})", project, client_id);

        harvest
            .create_project(harvest::CreateProject {
                is_active: project.is_active,
                name: project.name,
                client_id,
                code: project.pp_id,
                notes: project.rentman_id.to_string(),
                bill_by: "none".to_string(),
                budget_by: "none".to_string(),
                is_billable: true,
            })
            .await
            .unwrap();
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let harvest_token = std::env::var("HARVEST_TOKEN").expect("No Harvest token defined");
    let harvest_account_id =
        std::env::var("HARVEST_ACCOUNT_ID").expect("No Harvest Account ID defined");
    let harvest_user_agent =
        std::env::var("HARVEST_USER_AGENT").expect("No Harvest User Agent defined");
    let nvt_client = std::env::var("HARVEST_NVT_CLIENT")
        .expect("No Harvest NVT Client defined")
        .parse::<i64>()
        .expect("Harvest NVT client is not a valid number");

    let rentman_token = std::env::var("RENTMAN_TOKEN").expect("No Rentman token defined");
    let btdb_id = std::env::var("RENTMAN_BTDB_ID")
        .expect("No Rentman BTDB ID defined")
        .parse::<i64>()
        .expect("Rentman BTDB ID is not a valid number");

    let harvest = HarvestClient::new(
        harvest_token,
        harvest_account_id,
        harvest_user_agent,
        nvt_client,
    );

    let rentman = RentmanClient::new(rentman_token, btdb_id);

    // Clients
    update_clients(&harvest, &rentman).await;

    // Projects
    update_projects(&harvest, &rentman).await;
}

#[derive(Debug)]
struct MissingClient {
    name: String,
    address: String,
}

#[derive(Debug)]
struct MissingProject {
    name: String,
    rentman_id: String,
    rentman_client_id: i64,
    pp_id: String,
    is_active: bool,
}
