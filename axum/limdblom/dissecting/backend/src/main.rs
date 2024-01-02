use std::net::SocketAddr;

use axum::{routing::get, Json, Router};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/",get(handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("server started , listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("failed to run server");
}

#[derive(serde::Serialize)]
struct Message {
    message: String,
}

async fn handler() -> Json<Message> {
    Json(Message {
        message : String::from("Hello, World!")
    })
}
