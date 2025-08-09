use sqlx::{Pool, Postgres};

use crate::database::db_get_leaves;

pub async fn get_leaves(player: &String, server: String, conn: &Pool<Postgres>) -> String {
    let res = db_get_leaves(player.to_string(), server, conn).await;

    match res {
        Ok(v) => {
            let formatted_text = format!("{}: {}", v.player_name, v.leaves);

            formatted_text
        },
        Err(_e) => {
            "Player not found".to_string()
        }
    }
}