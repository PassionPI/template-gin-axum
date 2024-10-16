use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use validator::Validate;

use crate::{core::Core, model::user::Credentials, pkg::util::create_auth_response};

use super::AppError;

pub async fn sign(
    Extension(core): Extension<Arc<Core>>,
    Json(credentials): Json<Credentials>,
) -> Result<impl IntoResponse, AppError> {
    let password = String::from_utf8(core.rsa.decrypt_base64(&credentials.password)?)?;

    let credentials = Credentials {
        username: credentials.username,
        password,
    };

    credentials.validate()?;

    core.pg.user_insert(&credentials).await?;

    let token = core.jwt_encode(credentials.username.clone())?;

    Ok(create_auth_response(&token, &credentials.username)?)
}

pub async fn login(
    Extension(core): Extension<Arc<Core>>,
    Json(credentials): Json<Credentials>,
) -> Result<impl IntoResponse, AppError> {
    let user = match core.pg.user_find_by_username(&credentials.username).await {
        Ok(user) => user,
        Err(_) => return Ok((StatusCode::NOT_FOUND, "User not found!".to_string()).into_response()),
    };

    let password = String::from_utf8(core.rsa.decrypt_base64(&credentials.password)?)?;

    if user.password != password {
        return Ok((StatusCode::BAD_REQUEST, "Password invalid!".to_string()).into_response());
    }

    let token = core.jwt_encode(user.username.clone())?;

    Ok(create_auth_response(&token, &user.username)?)
}
