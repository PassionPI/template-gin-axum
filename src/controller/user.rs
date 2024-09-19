use std::sync::Arc;

use axum::{
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
    Extension, Json,
};
use validator::Validate;

use crate::{core::dep::Dep, model::user::Credentials, pkg::util::set_cookie_auth};

use super::AppError;

pub async fn sign(
    Extension(dep): Extension<Arc<Dep>>,
    Json(credentials): Json<Credentials>,
) -> Result<impl IntoResponse, AppError> {
    let password = String::from_utf8(dep.rsa.decrypt_base64(&credentials.password)?)?;

    let credentials = Credentials {
        username: credentials.username,
        password,
    };

    credentials.validate()?;

    dep.pg.user_insert(&credentials).await?;

    let token = dep.jwt_encode(credentials.username)?;

    let mut res = Response::new(Body::empty());

    res = set_cookie_auth(res, &token)?;

    Ok(res)
}

pub async fn login(
    Extension(dep): Extension<Arc<Dep>>,
    Json(credentials): Json<Credentials>,
) -> Result<impl IntoResponse, AppError> {
    let user = match dep.pg.user_find_by_username(&credentials.username).await {
        Ok(user) => user,
        Err(_) => return Ok((StatusCode::NOT_FOUND, "User not found!".to_string()).into_response()),
    };

    let password = String::from_utf8(dep.rsa.decrypt_base64(&credentials.password)?)?;

    if user.password != password {
        return Ok((StatusCode::BAD_REQUEST, "Password invalid!".to_string()).into_response());
    }

    let token = dep.jwt_encode(user.username)?;

    let mut res = Response::new(Body::empty());

    res = set_cookie_auth(res, &token)?;

    Ok(res)
}
