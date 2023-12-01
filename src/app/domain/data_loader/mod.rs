use crate::{REQUEST_CONFIG, app::domain::clients::{PeeperClient, get_client}};

use super::models::user_request::UserRequestBundle;

 pub async fn load() {
    let request_bundle: UserRequestBundle = match REQUEST_CONFIG.location.as_str() {
        "file" => confy::load_path(&REQUEST_CONFIG.location_file_path).unwrap_or_default(),
        &_ => confy::load_path(&REQUEST_CONFIG.location_file_path).unwrap_or_default(),
    }; 

    println!("Found req bundle {:?}", request_bundle);

    let mut drom_client = get_client();

    for user_request in request_bundle.requests.iter() {
        println!("Found req {:?}", user_request);
        drom_client.search(&user_request).await;
    }
}