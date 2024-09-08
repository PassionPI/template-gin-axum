use chrono::{DateTime, Utc};

#[derive(serde::Serialize, sqlx::FromRow)]
pub struct TodoScanItem {
    pub id: i64,
    pub done: bool,
    pub title: String,
    pub description: Option<String>,
    pub deadline: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(serde::Deserialize)]
pub struct TodoCreateItem {
    pub title: String,
    pub deadline: Option<DateTime<Utc>>,
    pub description: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct TodoUpdateItem {
    pub id: i64,
    pub done: bool,
    pub title: Option<String>,
    pub deadline: Option<DateTime<Utc>>,
    pub description: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct TodoDelItem {
    pub id: i64,
}
