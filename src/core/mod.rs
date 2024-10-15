use dep::Env;

use crate::{
    data::{pg::Pg, rd::Rd},
    pkg::rsa::Rsa,
};

pub mod dep;

#[derive(Clone)]
pub struct Core {
    pub env: Env,
    pub rsa: Rsa,
    pub pg: Pg,
    pub rd: Rd,
}

impl Core {
    pub async fn new() -> Self {
        let env = Env::new();
        let rsa = Rsa::new(&env.dir_private);
        let pg = Pg::new(&env.uri_db_pg).await;
        let rd = Rd::new(&env.uri_db_rd).await;
        Self { env, rsa, pg, rd }
    }
}
