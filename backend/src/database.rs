use mongodb::{
    options::{ClientOptions, ServerApi, ServerApiVersion},
    Client,
};

fn get_connect_string() -> String {
    format!(
        "mongodb://{}:{}@ac-ribnuqb-shard-00-00.trbjh1s.mongodb.net:27017,ac-ribnuqb-shard-00-01.trbjh1s.mongodb.net:27017,ac-ribnuqb-shard-00-02.trbjh1s.mongodb.net:27017/?ssl=true&replicaSet=atlas-lkdzgj-shard-0&authSource=admin&retryWrites=true&w=majority",
        // "mongodb+srv://{}:{}@bioinf-news.tdhbhd0.mongodb.net/?retryWrites=true&w=majority", //On my machine there is some problem with dns resolution
        std::env::var("DB_USER").expect("DB_USER must be set."),
        std::env::var("DB_PASSWORD").expect("DB_PASSWORD must be set.")
    )
}

pub async fn connect_mongo_db() -> mongodb::error::Result<Client> {
    let mut client_options = ClientOptions::parse(get_connect_string()).await?;

    // Set the server_api field of the client_options object to Stable API version 1
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);
    // Get a handle to the cluster
    Client::with_options(client_options)
}
