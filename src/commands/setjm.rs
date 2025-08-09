use sqlx::{types::chrono::{DateTime, Utc}, Pool, Postgres};

use crate::database::db_set_joinmessage;

pub async fn set_jm(creator: &String, player: String, message: String, server: String, conn: &Pool<Postgres>) -> String {
    let timestamp: DateTime<Utc> = Utc::now();

    let res = db_set_joinmessage(creator.to_string(), player.clone(), message, timestamp, server, conn).await;

    match res {
        Ok(_v) => {
            let msg = format!("Set join message for {}", player.clone());

            msg
        },
        Err(_e) => {
            let msg = format!("Failed to set join message for {}", player.clone());

            msg
        }
    }
}