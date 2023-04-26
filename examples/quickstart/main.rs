use std::{
    collections::BTreeMap,
    env,
};

use convex::ConvexClient;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    dotenv::from_filename(".env.local").ok();

    let deployment_url = env::var("CONVEX_URL").unwrap();

    let mut client = ConvexClient::new(&deployment_url).await.unwrap();
    let result = client.query("getTasks", BTreeMap::new()).await.unwrap();
    println!("{result:#?}");
}
