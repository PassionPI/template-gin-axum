use crate::{
    data::{pg::Pg, rd::Rd},
    pkg::rsa::Rsa,
};
use std::env;

#[derive(Clone)]
pub struct Env {
    pub port: String,
    pub uri_db_rd: String,
    pub uri_db_pg: String,
    pub secret_jwt: Vec<u8>,
    pub dir_private: String,
    pub dir_asset: String,
}

impl Env {
    pub fn new() -> Self {
        let port = match env::var("PORT") {
            Ok(port) => port,
            Err(_) => "8060".to_string(),
        };
        let uri_db_rd = match env::var("REDIS_URI") {
            Ok(uri) => uri,
            Err(_) => "".to_string(),
        };
        let uri_db_pg = match env::var("POSTGRES_URI") {
            Ok(uri) => uri,
            Err(_) => "".to_string(),
        };
        let secret_jwt = match env::var("JWT_SECRET") {
            Ok(secret) => secret.into_bytes(),
            Err(_) => "".to_string().into_bytes(),
        };

        Self {
            port,
            uri_db_rd,
            uri_db_pg,
            secret_jwt,
            dir_private: "./private".to_string(),
            dir_asset: "/asset".to_string(),
        }
    }
}

#[derive(Clone)]
pub struct Dep {
    pub env: Env,
    pub rsa: Rsa,
    pub pg: Pg,
    pub rd: Rd,
}

impl Dep {
    pub async fn new() -> Self {
        let env = Env::new();
        let rsa = Rsa::new(&env.dir_private);
        let pg = Pg::new(&env.uri_db_pg).await;
        let rd = Rd::new(&env.uri_db_rd).await;
        Self { env, rsa, pg, rd }
    }
}
