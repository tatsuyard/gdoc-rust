extern crate dotenv;
use dotenv::dotenv;
use std::env;
use google_drive::GoogleDrive;
use yup_oauth2::{read_service_account_key, ServiceAccountAuthenticator};

// async fn get_dirves() {
//     let gsuite_credential_file =
//         env::var("GADMIN_CREDENTIAL_FILE").unwrap();
// }

fn main() {
    dotenv().ok();

    for (key, value) in env::vars() {
        println!("{}: {}", key, value);
    }
}
