#![allow(unused)]
pub use self::error::{Error, Result};

use std::net::SocketAddr;

use axum::{
    extract::{Path, Query},
    http::Method,
    middleware,
    response::{Html, IntoResponse, Response},
    routing::{get, get_service},
    Router,
};
use serde::Deserialize;
use tower_cookies::CookieManagerLayer;
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
};

mod error;
mod web;
mod model;

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any);
    let routes_all = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .layer(cors)
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    // region: --Start server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("-->> Listening on {:?}", addr);
    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();

    // endregion: --Start server
    async fn main_response_mapper(res: Response) -> Response {
        println!("->> {:<12} - {res:?}", "RESPONSE_MAPPER");
        println!();
        res
    }

    fn routes_static() -> Router {
        Router::new().nest_service("/", get_service(ServeDir::new("./")))
    }
    // region: --Routes Hello
    fn routes_hello() -> Router {
        Router::new()
            .route("/hello", get(handler_hello))
            .route("/hello/:name", get(handler_hello2))
    }
    #[derive(Debug, Deserialize)]
    struct HelloParams {
        name: Option<String>,
    }

    async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
        println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");
        let name = params.name.as_deref().unwrap_or("World");
        Html(format!("Hello <strong>{name}!!!!</strong>"))
        // Html("Hello <strong>World!!!!</strong>")
    }

    async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
        println!("->> {:<12} - handler_hello2 - {name:?}", "HANDLER");
        Html(format!("Hello <strong>{name}!!!!</strong>"))
    }
    // endregion: --Handler Hello
}
