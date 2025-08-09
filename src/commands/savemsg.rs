use sqlx::{types::chrono::{DateTime, Utc}, Pool, Postgres};

use crate::database::db_savemsg;

pub async fn savemsg(player: &String, message: String, server: String, conn: &Pool<Postgres>) -> String {
    let timestamp: DateTime<Utc> = Utc::now();

    let res = db_savemsg(player.to_string(), message, timestamp, server, conn).await;

    match res {
        Ok(_v) => {
            "Added message".to_string()
        },
        Err(_e) => {
            "Failed to add message".to_string()
        }
    }
}