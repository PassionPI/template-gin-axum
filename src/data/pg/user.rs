use crate::model::user::Credentials;

use super::Pg;

impl Pg {
    pub async fn user_find_by_username(&self, username: &str) -> anyhow::Result<Credentials> {
        let row = sqlx::query_as::<_, Credentials>(
            "
            SELECT username, password 
            FROM user_base
            WHERE username = $1
            ",
        )
        .bind(username)
        .fetch_one(&self.pool)
        .await?;

        Ok(row)
    }

    pub async fn user_insert(&self, credentials: &Credentials) -> anyhow::Result<()> {
        sqlx::query(
            "
            INSERT INTO user_base (username, password)
            VALUES ($1, $2)
            ",
        )
        .bind(&credentials.username)
        .bind(&credentials.password)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
