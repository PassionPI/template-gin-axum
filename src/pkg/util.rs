use axum::{
    body::Body,
    http::{header::SET_COOKIE, HeaderValue},
    response::Response,
};

const MAX_AGE: i32 = 7 * 24 * 3600;

pub fn set_cookie_auth(
    mut response: Response<Body>,
    token: &str,
) -> anyhow::Result<Response<Body>> {
    let cookie_value = format!(
        "Auth={}; Path=/; Max-Age={}; SameSite=Strict; HttpOnly; Secure;",
        token, MAX_AGE
    );

    response
        .headers_mut()
        .insert(SET_COOKIE, HeaderValue::from_str(&cookie_value)?);

    Ok(response)
}
