use crate::routes;

pub async fn run() {
    let app = routes::create_router();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind TCP listener");

    println!("Server running at http://127.0.0.1:3000");

    axum::serve(listener, app).await.expect("Server failed")
}
