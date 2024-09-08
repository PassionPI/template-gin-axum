use crate::{
    data::{pg::Pg, rd::Rd},
    pkg::rsa::Rsa,
};
use std::env;

#[derive(Clone)]
pub struct Env {
    pub port: String,
    pub db_rd_uri: String,
    pub db_pg_uri: String,
    pub jwt_secret: Vec<u8>,
    pub private_dir: String,
}

impl Env {
    pub fn new() -> Self {
        let port = match env::var("PORT") {
            Ok(port) => port,
            Err(_) => "8060".to_string(),
        };
        let db_rd_uri = match env::var("REDIS_URI") {
            Ok(uri) => uri,
            Err(_) => "".to_string(),
        };
        let db_pg_uri = match env::var("POSTGRES_URI") {
            Ok(uri) => uri,
            Err(_) => "".to_string(),
        };
        let jwt_secret = match env::var("JWT_SECRET") {
            Ok(secret) => secret.into_bytes(),
            Err(_) => "".to_string().into_bytes(),
        };
        Self {
            port,
            db_rd_uri,
            db_pg_uri,
            jwt_secret,
            private_dir: "./private".to_string(),
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
        let rsa = Rsa::new(&env.private_dir);
        let pg = Pg::new(&env.db_pg_uri).await;
        let rd = Rd::new(&env.db_rd_uri).await;
        Self { env, rsa, pg, rd }
    }
}
