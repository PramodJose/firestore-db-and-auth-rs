use firestore_db_and_auth::{Credentials, ServiceSession};
use firestore_db_and_auth::*;

#[tokio::main]
async fn main() -> errors::Result<()> {
    // Create credentials object. You may as well do that programmatically.
    let cred = Credentials::from_file("/Users/uy41gd/rust_projects/firestore-db-and-auth-rs/examples/firebase-service-account.json").await
        .expect("Read credentials file")
        .download_jwkset().await
        .expect("Failed to download public keys");

    // To use any of the Firestore methods, you need a session first. You either want
    // an impersonated session bound to a Firebase Auth user or a service account session.
    let session = ServiceSession::new(cred).await
        .expect("Create a service account session");

    println!("{:#?}", session);
    Ok(())
}