use sqlx::{Pool, Postgres};

use crate::database::db_get_kd;

pub async fn get_kd(player: &String, server: String, conn: &Pool<Postgres>) -> String {
    let res = db_get_kd(player.to_string(), server, conn).await;

    match res {
        Ok(v) => {
            let kd: f32 = (v.kills as f32 / v.deaths as f32) as f32;

            let msg = format!("{} kills: {}, deaths: {}, K/D: {:.2}", v.player_name, v.kills, v.deaths, kd);

            msg
        },
        Err(_e) => {
            "Player not found".to_string()
        }
    }
}