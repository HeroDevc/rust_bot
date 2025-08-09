use sqlx::{Pool, Postgres};

use crate::database::db_get_savedmsg;

pub async fn get_savedmsg(player: &String, server: String, conn: &Pool<Postgres>) -> String {
    let res = db_get_savedmsg(player.to_string(), server, conn).await;

    match res {
        Ok(v) => {
            let msg = format!("{}: {}", v.player_name, v.message);

            msg
        },
        Err(_e) => {
            "Player not found".to_string()
        }
    }
}