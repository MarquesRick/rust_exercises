use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct TaskModel {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub created_at: Option<DateTime<Utc>>,
}
