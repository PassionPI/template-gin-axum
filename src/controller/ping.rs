use axum::{
    http::header::SET_COOKIE,
    response::{IntoResponse, Response},
};

pub async fn ping() -> impl IntoResponse {
    let mut res = Response::new("pong".into_response());
    res.headers_mut().insert(
        SET_COOKIE,
        "Auth=123; Path=/; Max-Age=3600; SameSite=Strict; HttpOnly; Secure;"
            .parse()
            .unwrap(),
    );

    res
}
