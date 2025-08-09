use sqlx::{types::chrono::{DateTime, Utc}, Pool, Postgres};

use crate::database::db_insertiam;

pub async fn save_iam(player: &String, message: String, server: String, conn: &Pool<Postgres>) -> String {
    let timestamp: DateTime<Utc> = Utc::now();

    let res = db_insertiam(player.to_string(), message, timestamp, server, conn).await;

    match res {
        Ok(_v) => {
            "Information added".to_string()
        },
        Err(_e) => {
            "Failed to add information".to_string()
        }
    }
}