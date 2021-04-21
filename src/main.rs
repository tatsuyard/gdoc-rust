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
    let gsuite_subject = env::var("GADMIN_SUBJECT").unwrap();
    let auth = ServiceAccountAuthenticator::builder(gsuite_secret)
        .subject(gsuite_subject.to_string())
        .build()
        .await
        .expect("failed to create authenticator");
    let token = auth.token(&["https://www.googleapis.com/auth/drive"])
        .await
        .expect("failed to get token");

    if token.as_str().is_empty() {
        panic!("empty token is not valid");
    }

    let drive_client = GoogleDrive::new(token);

    let drives = drive_client.list_drives().await.unwrap();
    for drive in drives {
        println!("{:?}", drive);
    }
}

fn main() {
    dotenv().ok();

    get_dirves();
    println!("@@@@@");

}
