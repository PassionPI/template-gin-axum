use std::net::SocketAddr;

use axum::{
    extract::{ConnectInfo, Request},
    middleware::Next,
    response::IntoResponse,
    RequestExt,
};

use crate::controller::AppError;

pub async fn logger(mut req: Request, next: Next) -> Result<impl IntoResponse, AppError> {
    let now = chrono::Utc::now();
    let addr = req.extract_parts::<ConnectInfo<SocketAddr>>().await?;
    let path = String::from(req.uri().path());
    let method = req.method().to_string();

    let res = next.run(req).await.into_response();

    let cost = chrono::Utc::now()
        .signed_duration_since(now)
        .num_milliseconds();

    println!(
        "[Axum] {} | {} | {} | {}ms | {} {}",
        now.format("%Y/%m/%d - %H:%M:%S"),
        &res.status(),
        addr.ip(),
        cost,
        method,
        path,
    );

    Ok(res)
}
