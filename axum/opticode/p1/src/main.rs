// use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};
mod box_custom;
use box_custom::run_box;
#[tokio::main]
async fn main() {
    // server().await;
    run_box();
    
}

// async fn server() {
//     let app: Router = Router::new().route("/api/test", get(test));

//     axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
//         .serve(app.into_make_service())
//         .await
//         .unwrap()
// }

// async fn test() -> impl IntoResponse {
//     println!("Test Api");
//     (StatusCode::ACCEPTED, "Hey There")
// }
