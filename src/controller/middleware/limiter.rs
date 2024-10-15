use std::net::SocketAddr;
use std::sync::Arc;

use axum::{
    extract::{ConnectInfo, Request},
    http::StatusCode,
    middleware::Next,
    response::IntoResponse,
    RequestExt,
};
use redis::AsyncCommands;

use crate::{controller::AppError, core::Core};

const INC: i64 = 1;
const MIN: i64 = 1;
const MAX: i64 = 100;
const EXPIRE: i64 = 60;

pub async fn limiter(mut req: Request, next: Next) -> Result<impl IntoResponse, AppError> {
    let addr = req.extract_parts::<ConnectInfo<SocketAddr>>().await?;
    let core = match req.extensions().get::<Arc<Core>>() {
        Some(core) => core,
        None => {
            return Ok((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Dependency not found".to_string(),
            )
                .into_response())
        }
    };

    let key = format!(
        "{}:{}:{}",
        req.uri().path(),
        "middleware.limiter",
        addr.ip()
    );

    let mut conn = core.rd.conn.clone();

    let count = conn.get(&key).await.unwrap_or(MIN);

    if count == MIN {
        conn.expire(&key, EXPIRE).await?;
    }

    if count >= MAX {
        return Ok((
            StatusCode::TOO_MANY_REQUESTS,
            "Too many requests".to_string(),
        )
            .into_response());
    }

    conn.incr(&key, INC).await?;

    Ok(next.run(req).await.into_response())
}
