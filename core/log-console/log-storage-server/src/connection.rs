use mongodb::{options::ClientOptions, Client, Database};

pub async fn get_connection() -> mongodb::error::Result<Database> {
    let uri = std::env::var("MONGO_CONNECTION_STR")
        .expect("env variable MONGO_CONNECTION_STR was not found");
    let options = ClientOptions::parse(uri).await?;
    let client = Client::with_options(options)?;
    Ok(client.database("logs"))
}
