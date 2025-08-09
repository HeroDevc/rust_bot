use sqlx::{Pool, Postgres};

use crate::database::db_remove_leavemessage;

pub async fn remove_lm(player: &String, server: String, conn: &Pool<Postgres>) -> String {
    let res = db_remove_leavemessage(player.to_string(), server, conn).await;

    match res {
        Ok(_v) => {
            let msg = format!("Removed leave message for {}", player.clone());

            msg
        },
        Err(_e) => {
            let msg = format!("Failed to remove leave message for {}", player);

            msg
        }
    }
}