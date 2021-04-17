extern crate dotenv;
use dotenv::dotenv;
use std::env;
use google_drive::GoogleDrive;
use yup_oauth2::{read_service_account_key, ServiceAccountAuthenticator};

async fn get_dirves() {
    let gsuite_credential_file =
        env::var("GADMIN_CREDENTIAL_FILE").unwrap();
    let gsuite_secret = read_service_account_key(gsuite_credential_file)
        .await
        .expect("failed to read gsuite credential file");
    let auth = ServiceAccountAuthenticator::builder(gsuite_secret)
        .subject(gsuite_subject.to_string())
        .build()
        .await
        .expect("failed to create authenticator");
}

fn main() {
    dotenv().ok();

    get_dirves();
    // for (key, value) in env::vars() {
    //     println!("{}: {}", key, value);
    // }
}
