use sqlx::{Pool, Postgres};

use crate::database::db_get_lastkill;

pub async fn get_lastkill(player: &String, server: String, conn: &Pool<Postgres>) -> String {
    let res = db_get_lastkill(player.to_string(), server, conn).await;

    match res {
        Ok(v) => {
            let formatted_date = v.last_kill_timestamp.format("%Y-%m-%d %H:%M:%S");

            let msg = format!("({}) {}: {}", formatted_date, v.player_name, v.last_kill_message);

            msg
        },
        Err(_e) => {
            "Player not found".to_string()
        }
    }
}