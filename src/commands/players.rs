use sqlx::{Pool, Postgres};

use crate::database::db_get_total_players;

pub async fn get_total_players(server: String, conn: &Pool<Postgres>) -> String {
    let count = db_get_total_players(server, conn).await;

    match count {
        Ok(v) => {
            let msg = format!("I've seen {v} players on this server.");

            msg
        },
        Err(e) => {
            println!("{}", e);

            let msg = format!("I've never seen anyone playing here.");

            msg
        }
    }
}