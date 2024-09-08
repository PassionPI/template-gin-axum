use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate, Debug, Clone, sqlx::FromRow)]
pub struct Credentials {
    #[validate(length(min = 4, max = 24))]
    pub username: String,
    #[validate(length(min = 8, max = 32))]
    pub password: String,
}
