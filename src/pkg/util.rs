use axum::{
    body::Body,
    http::{header::SET_COOKIE, HeaderValue},
    response::{IntoResponse, Response},
    Json,
};

use crate::data::JWT_DAYS_EXP;

const MAX_AGE: i64 = JWT_DAYS_EXP * 24 * 3600;

#[derive(serde::Serialize)]
struct UserInfo {
    pub username: String,
}

pub fn create_auth_response(token: &str, username: &str) -> anyhow::Result<Response<Body>> {
    let mut response = Json(UserInfo {
        username: username.to_owned(),
    })
    .into_response();

    set_auth_cookie(&mut response, token)?;

    Ok(response)
}

pub fn set_auth_cookie(response: &mut Response<Body>, token: &str) -> anyhow::Result<()> {
    let cookie_value = format!(
        "Auth={}; Path=/; Max-Age={}; SameSite=Strict; HttpOnly; Secure;",
        token, MAX_AGE
    );

    response
        .headers_mut()
        .insert(SET_COOKIE, HeaderValue::from_str(&cookie_value)?);

    Ok(())
}
