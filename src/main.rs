mod db;
mod models;
mod routes;
mod server;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();

    server::run().await;
}
