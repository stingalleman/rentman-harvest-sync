use harvest::HarvestClient;
use rentman::RentmanClient;

mod harvest;
mod rentman;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let harvest_token = std::env::var("HARVEST_TOKEN").expect("No Harvest token defined");
    let harvest_account_id =
        std::env::var("HARVEST_ACCOUNT_ID").expect("No Harvest Account ID defined");
    let harvest_user_agent =
        std::env::var("HARVEST_USER_AGENT").expect("No Harvest User Agent defined");

    let rentman_token = std::env::var("RENTMAN_TOKEN").expect("No Rentman token defined");

    let harvest = HarvestClient::new(harvest_token, harvest_account_id, harvest_user_agent);
    let rentman = RentmanClient::new(rentman_token);

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

        // Client is found, continue to next contact
        if found_client.is_some() {
            continue;
        }

        // Push missing client to vec
        missing_clients.push(MissingClient {
            address: contact.id.to_string(),
            name: contact.name,
        })
    }

    println!("{:?}", missing_clients);
}

#[derive(Debug)]
struct MissingClient {
    name: String,
    address: String,
}
