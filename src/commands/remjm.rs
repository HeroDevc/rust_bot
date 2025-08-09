use sqlx::{Pool, Postgres};

use crate::database::db_remove_joinmessage;

pub async fn remove_jm(player: &String, server: String, conn: &Pool<Postgres>) -> String {
    let res = db_remove_joinmessage(player.to_string(), server, conn).await;

    match res {
        Ok(_v) => {
            let msg = format!("Removed join message for {}", player);

            msg
        },
        Err(_e) => {
            let msg = format!("Failed to remove join message for {}", player);

            msg
        }
    }
}