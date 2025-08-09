use sqlx::{Pool, Postgres};

use crate::database::db_get_nwords;

pub async fn get_nwords(player: &String, server: String, conn: &Pool<Postgres>) -> String {
    let res = db_get_nwords(player.to_string(), server, conn).await;

    match res {
        Ok(v) => {
            let msg = format!("{} hard: {}, soft: {}", v.player_name, v.hard, v.soft);

            msg
        },
        Err(_e) => {
            "Player not found".to_string()
        }
    }
}