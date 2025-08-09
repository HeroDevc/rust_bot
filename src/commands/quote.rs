use sqlx::{Pool, Postgres};

use crate::database::db_get_quote;

pub async fn get_quote(player: &String, server: String, conn: &Pool<Postgres>) -> String {
    let res = db_get_quote(player.to_string(), server, conn).await;

    match res {
        Ok(v) => {
            let formatted_date = v.timestamp.format("%Y-%m-%d %H:%M:%S");

            let msg = format!("({}) {}: {}", formatted_date, v.player_name, v.message);

            msg
        },
        Err(_e) => {
            "Player not found".to_string()
        }
    }
}