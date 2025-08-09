use sqlx::{Pool, Postgres};

use crate::database::db_get_lastdeath;

pub async fn get_lastdeath(player: &String, server: String, conn: &Pool<Postgres>) -> String {
    let res = db_get_lastdeath(player.to_string(), server, conn).await;

    match res {
        Ok(v) => {
            let formatted_date = v.last_death_timestamp.format("%Y-%m-%d %H:%M:%S");

            let msg = format!("({}) {}: {}", formatted_date, v.player_name, v.last_death_message);

            msg
        },
        Err(_e) => {
            "Player not found".to_string()
        }
    }
}