use sqlx::{types::chrono::{DateTime, Utc}, Pool, Postgres};

use crate::database::db_set_leavemessage;

pub async fn set_lm(creator: &String, player: String, message: String, server: String, conn: &Pool<Postgres>) -> String {
    let timestamp: DateTime<Utc> = Utc::now();

    let res = db_set_leavemessage(creator.to_string(), player.clone(), message, timestamp, server, conn).await;

    match res {
        Ok(_v) => {
            let msg = format!("Set leave message for {}", player.clone());

            msg
        },
        Err(_e) => {
            let msg = format!("Failed to set leave message for {}", player.clone());

            msg
        }
    }
}