pub mod friend;
pub mod friend_request;
pub mod note;
pub mod note_comment;
pub mod note_image;
pub mod note_tag;
pub mod notice;
pub mod subscriber;
pub mod todo;
pub mod user;

use sqlx::postgres::PgPool;

#[derive(Clone)]
pub struct Pg {
    pub pool: PgPool,
}

impl Pg {
    pub async fn new(uri: &str) -> Self {
        let pool = PgPool::connect(uri)
            .await
            .expect("Failed to create postgres pool");
        println!("Connected to postgres.");

        sqlx::migrate!("./sql")
            .run(&pool)
            .await
            .expect("Migrate error!");

        ensure_table_exists(
            &pool,
            vec![
                &create_index_sql("user_base", "role"),
                &create_index_sql("user_base", "nickname"),
                &create_index_sql("todo", "username"),
                &create_index_sql("note", "user_id"),
                &create_index_sql("friend", "id_a"),
                &create_index_sql("friend", "id_b"),
                &create_index_sql("friend", "status"),
                &create_index_sql("friend_request", "user_by"),
                &create_index_sql("friend_request", "user_to"),
                &create_index_sql("friend_request", "status"),
            ],
        )
        .await;

        Self { pool }
    }
}

async fn ensure_table_exists(pool: &PgPool, sql: Vec<&str>) {
    let mut chunk = String::new();
    for s in sql {
        chunk += s;
    }

    sqlx::raw_sql(&chunk)
        .execute(pool)
        .await
        .expect("Error Exec Initial SQL");
}

fn create_index_sql(table_name: &str, column: &str) -> String {
    format!(
        "CREATE INDEX IF NOT EXISTS idx_{}_{} ON {}({});",
        table_name, column, table_name, column
    )
}
