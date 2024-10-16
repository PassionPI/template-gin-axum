mod app;
mod controller;
mod core;
mod data;
mod model;
mod pkg;

use std::net::SocketAddr;

use app::create;

#[tokio::main]
async fn main() {
    let (app, dep) = create().await;

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", "0.0.0.0", dep.env.port))
        .await
        .expect("could not bind to address");

    println!(
        "Listening on http://{}",
        listener.local_addr().expect("could not determine address")
    );

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .expect("server failed");
}
