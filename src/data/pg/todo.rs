use crate::model::{
    common::Pagination,
    todo::{TodoCreateItem, TodoDelItem, TodoScanItem, TodoUpdateItem},
};

use super::Pg;

impl Pg {
    pub async fn todo_find_by_username(
        &self,
        username: &str,
        pagination: &Pagination,
    ) -> anyhow::Result<Vec<TodoScanItem>> {
        let rows = sqlx::query_as::<_, TodoScanItem>(
            "
            SELECT id, title, done, updated_at, deadline, description
            FROM todo
            WHERE username = $1
            ORDER BY id DESC
            OFFSET $2
            LIMIT $3
            ",
        )
        .bind(username)
        .bind(pagination.page * pagination.size)
        .bind(pagination.size)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }

    pub async fn todo_insert_item(
        &self,
        username: &str,
        insert: &TodoCreateItem,
    ) -> anyhow::Result<()> {
        sqlx::query(
            "
            INSERT INTO todo 
                (username, title, deadline, description)
            VALUES 
                ($1, $2, $3, $4)
            ",
        )
        .bind(username)
        .bind(&insert.title)
        .bind(insert.deadline)
        .bind(&insert.description)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn todo_update_item(&self, update: &TodoUpdateItem) -> anyhow::Result<()> {
        sqlx::query(
            "
            UPDATE todo
            SET 
                updated_at = CURRENT_TIMESTAMP,
                done = $1,
                deadline = $2
            WHERE id = $3
            ",
        )
        .bind(update.done)
        .bind(update.deadline)
        .bind(update.id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn todo_delete_item(&self, del: &TodoDelItem) -> anyhow::Result<()> {
        sqlx::query(
            "
            DELETE FROM todo
            WHERE id = $1
            ",
        )
        .bind(del.id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
