use sqlx::{types::chrono::{DateTime, Utc}, Pool, Postgres};

use crate::database::{db_get_allfaqs, db_insert_faq};

// terrible error handling but didn't want a bigger headache

pub async fn add_faq(player: &String, message: String, server: String, conn: &Pool<Postgres>) -> String {
    let timestamp: DateTime<Utc> = Utc::now();

    let all_entries = db_get_allfaqs(server.to_string(), conn).await;

    let mut i = 1;

    match all_entries {
        Ok(v) => {
            i += v;

            let res = db_insert_faq(i, player.to_string(), message, timestamp, server.clone(), conn).await;

            match res {
                Ok(_val) => {
                    let msg = format!("Added faq #{}", i);

                    msg
                },
                Err(_e) => {
                    "Failed to add faq".to_string()
                }
            }
        },
        Err(_e) => {
            "Failed to add faq".to_string()
        }
    }
}