use std::sync::Arc;

use axum::{
    extract::Request,
    http::{self, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use serde::{Deserialize, Serialize};

use crate::{
    core::dep::Dep,
    data::{JWT_DAYS_EXP, JWT_DAYS_REFRESH},
    pkg::util::set_auth_cookie,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    exp: i64,
    // user_id: i64,
    pub username: String,
}

impl Dep {
    pub fn jwt_encode(
        &self,
        // user_id: i64,
        username: String,
    ) -> anyhow::Result<String> {
        let exp = match Utc::now().checked_add_signed(Duration::days(JWT_DAYS_EXP)) {
            Some(exp) => exp.timestamp_millis(),
            None => {
                return Err(anyhow::anyhow!("Failed to generate token"));
            }
        };

        let header = Header::new(Algorithm::HS256);
        let claims = Claims {
            // user_id,
            username,
            exp,
        };
        let key = EncodingKey::from_secret(self.env.secret_jwt.as_ref());

        Ok(encode(&header, &claims, &key)?)
    }
    pub fn jwt_decode(&self, token: &str) -> anyhow::Result<TokenData<Claims>> {
        let token = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.env.secret_jwt.as_ref()),
            &Validation::default(),
        )?;

        Ok(token)
    }
}

pub async fn auth(mut req: Request, next: Next) -> impl IntoResponse {
    let dep = match req.extensions().get::<Arc<Dep>>() {
        Some(dep) => dep.clone(),
        None => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Dependency not found".to_string(),
            )
                .into_response();
        }
    };

    let token = match req
        .headers()
        .get(http::header::COOKIE)
        .and_then(|header| header.to_str().ok())
        .and_then(|cookie| {
            cookie
                .split("; ")
                .collect::<Vec<&str>>()
                .into_iter()
                .find(|cookie| cookie.starts_with("Auth="))
        })
        .and_then(|auth| auth.strip_prefix("Auth="))
    {
        Some(cookie) => cookie,
        None => {
            return (StatusCode::UNAUTHORIZED, "Auth cookie missing".to_string()).into_response()
        }
    };

    let jwt = match dep.jwt_decode(token) {
        Ok(jwt) => jwt,
        Err(e) => {
            return (
                StatusCode::UNAUTHORIZED,
                "Invalid token: ".to_string() + &e.to_string(),
            )
                .into_response();
        }
    };

    let exp = jwt.claims.exp.to_owned();
    let now = Utc::now().timestamp_millis();
    let username = jwt.claims.username.clone();

    if now > exp {
        return (StatusCode::UNAUTHORIZED, "Token has expired.".to_string()).into_response();
    }

    req.extensions_mut().insert(jwt.claims);

    let mut response = next.run(req).await.into_response();

    let token = match dep.jwt_encode(username) {
        Ok(token) => token,
        Err(e) => {
            return (
                StatusCode::UNAUTHORIZED,
                "Failed to generate token: ".to_string() + &e.to_string(),
            )
                .into_response();
        }
    };

    if exp - now < JWT_DAYS_REFRESH * 24 * 60 * 60 * 1000 {
        match set_auth_cookie(&mut response, &token) {
            Ok(_) => (),
            Err(e) => {
                return (
                    StatusCode::UNAUTHORIZED,
                    "Failed to set cookie: ".to_string() + &e.to_string(),
                )
                    .into_response();
            }
        }
    }

    response.into_response()
}
