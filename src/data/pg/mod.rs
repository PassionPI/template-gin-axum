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
        ensure_table_exists(
            &pool,
            vec![
                &create_table_sql(
                    "users",
                    vec![
                        "id         BIGSERIAL PRIMARY KEY",
                        "username   TEXT NOT NULL UNIQUE",
                        "created_at TIMESTAMPTZ DEFAULT (NOW() AT TIME ZONE 'UTC')",
                        "updated_at TIMESTAMPTZ DEFAULT (NOW() AT TIME ZONE 'UTC')",
                        "password   TEXT NOT NULL",
                        "role       TEXT",
                        "nickname   TEXT",
                        "avatar     TEXT",
                    ],
                ),
                &create_index_sql("users", "role"),
                &create_index_sql("users", "nickname"),
                &create_table_sql(
                    "todo",
                    vec![
                        "id          BIGSERIAL PRIMARY KEY",
                        "created_at  TIMESTAMPTZ DEFAULT (NOW() AT TIME ZONE 'UTC')",
                        "updated_at  TIMESTAMPTZ DEFAULT (NOW() AT TIME ZONE 'UTC')",
                        "deadline    TIMESTAMPTZ",
                        "username    TEXT NOT NULL",
                        "title       TEXT NOT NULL",
                        "done        BOOLEAN DEFAULT false",
                        "description TEXT",
                    ],
                ),
                &create_index_sql("todo", "username"),
            ],
        )
        .await;
        println!("Execute the initial sql.");

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

fn create_table_sql(table_name: &str, kv: Vec<&str>) -> String {
    let creator = "CREATE TABLE IF NOT EXISTS";
    let start = format!("{} {} (", creator, table_name);
    let last = kv.len() - 1;

    let mut columns = String::new();

    for (i, v) in kv.iter().enumerate() {
        columns += v;
        if i != last {
            columns += ",";
        }
    }

    let end = ");";

    start + &columns + &end
}

fn create_index_sql(table_name: &str, column: &str) -> String {
    format!(
        "CREATE INDEX IF NOT EXISTS idx_{}_{} ON {}({});",
        table_name, column, table_name, column
    )
}
