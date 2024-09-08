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

use crate::core::dep::Dep;

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
        let exp = match Utc::now().checked_add_signed(Duration::days(7)) {
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
        let key = EncodingKey::from_secret(self.env.jwt_secret.as_ref());

        Ok(encode(&header, &claims, &key)?)
    }
    pub fn jwt_decode(&self, token: &str) -> anyhow::Result<TokenData<Claims>> {
        let token = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(self.env.jwt_secret.as_ref()),
            &Validation::default(),
        )?;

        Ok(token)
    }
}

pub async fn auth(mut req: Request, next: Next) -> impl IntoResponse {
    let dep = req.extensions().get::<Arc<Dep>>().unwrap();
    let auth_header = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let auth_header = match auth_header {
        Some(auth_header) => auth_header.replace("Bearer ", ""),
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                "Authorization header missing".to_string(),
            )
                .into_response()
        }
    };

    let jwt = match dep.jwt_decode(&auth_header) {
        Ok(jwt) => jwt,
        Err(e) => {
            return (
                StatusCode::UNAUTHORIZED,
                "Invalid token: ".to_string() + &e.to_string(),
            )
                .into_response();
        }
    };

    if Utc::now().timestamp_millis() > jwt.claims.exp {
        return (StatusCode::UNAUTHORIZED, "Token has expired.".to_string()).into_response();
    }

    req.extensions_mut().insert(jwt.claims);
    next.run(req).await.into_response()
}
