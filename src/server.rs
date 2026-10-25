use crate::{db, routes};

pub async fn run() {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is missing");
    let pool = db::connect(&database_url).await.unwrap();

    println!("Database connected successfully");

    let app = routes::create_router(pool);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind TCP listener");

    println!("Server running at http://127.0.0.1:3000");

    axum::serve(listener, app).await.expect("Server failed")
}
