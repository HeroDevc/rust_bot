use sqlx::{Pool, Postgres};

use crate::database::db_get_playtime;

pub async fn get_playtime(player: &String, count: i64, server: String, conn: &Pool<Postgres>) -> String {
    let res = db_get_playtime(player.to_string(), server, conn).await;

    match res {
        Ok(v) => {
            let player_seconds = v.seconds + count;

            let days = player_seconds / (24 * 60 * 60);
            let hours = (player_seconds % (24 * 60 * 60)) / (60 * 60);
            let minutes = (player_seconds % (60 * 60)) / 60;
            let seconds = player_seconds % 60;

            let mut pt_vec: Vec<String> = Vec::new();

            if days > 0 {
                let formatted_days = format!("{days} days");

                pt_vec.push(formatted_days);
            }

            if hours > 0 {
                let formatted_hours = format!("{hours} hours");

                pt_vec.push(formatted_hours);
            }

            if minutes > 0 {
                let formatted_minutes = format!("{minutes} minutes");

                pt_vec.push(formatted_minutes);
            }

            if seconds > 0 {
                let formatted_seconds = format!("{seconds} seconds");

                pt_vec.push(formatted_seconds);
            }

            let final_pt_string = pt_vec.join(", ");

            let msg = format!("{}: {}", v.player_name, final_pt_string);

            msg
        },
        Err(_e) => {
            if count > 0 {
                let days = count / (24 * 60 * 60);
                let hours = (count % (24 * 60 * 60)) / (60 * 60);
                let minutes = (count % (60 * 60)) / 60;
                let seconds = count % 60;

                let mut pt_vec: Vec<String> = Vec::new();

                if days > 0 {
                    let formatted_days = format!("{days} days");

                    pt_vec.push(formatted_days);
                }

                if hours > 0 {
                    let formatted_hours = format!("{hours} hours");

                    pt_vec.push(formatted_hours);
                }

                if minutes > 0 {
                    let formatted_minutes = format!("{minutes} minutes");

                    pt_vec.push(formatted_minutes);
                }

                if seconds > 0 {
                    let formatted_seconds = format!("{seconds} seconds");

                    pt_vec.push(formatted_seconds);
                }

                let final_pt_string = pt_vec.join(", ");

                let msg = format!("{}: {}", player, final_pt_string);

                msg
            } else {
                "Player not found".to_string()
            }
        }
    }
}