use sqlx::{Pool, Postgres};

use crate::database::db_get_whois;

pub async fn get_whois(player: &String, server: String, conn: &Pool<Postgres>) -> String {
    let res = db_get_whois(player.to_string(), server, conn).await;

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