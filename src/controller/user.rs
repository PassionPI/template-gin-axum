use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use validator::Validate;

use crate::{core::dep::Dep, model::user::Credentials};

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

    Ok(dep
        .jwt_encode(credentials.username)?
        .to_owned()
        .into_response())
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

    if user.password == password {
        return Ok(dep.jwt_encode(user.username)?.to_owned().into_response());
    } else {
        return Ok((StatusCode::BAD_REQUEST, "Password invalid!".to_string()).into_response());
    }
}
