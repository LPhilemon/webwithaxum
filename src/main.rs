#![allow(unused)]

use axum::response::Html;
use std::net::SocketAddr;
use axum::routing::get;
use axum::Router;

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route(
        "/hello",
        get(|| async { Html("Hello <strong>World!!!</strong>") }),
    );


// region:    ---Start Server
let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
// endregion: ---Start Server
println!("->> LISTENING on {addr}\n");

axum::Server::bind(&addr).serve(routes_hello.into_make_service()).await.unwrap();

}