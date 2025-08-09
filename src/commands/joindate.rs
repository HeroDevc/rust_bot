use sqlx::{Pool, Postgres};

use crate::database::db_get_joindate;

pub async fn get_joindate(player: &String, server: String, conn: &Pool<Postgres>) -> String {
    let res = db_get_joindate(player.to_string(), server, conn).await;

    match res {
        Ok(v) => {
            let formatted_date = v.timestamp.format("%Y-%m-%d %H:%M:%S");

            let msg = format!("{}: {}", v.player_name, formatted_date);

            msg
        },
        Err(_e) => {
            "Player not found".to_string()
        }
    }
}