use sqlx::{Pool, Postgres};

use crate::database::{db_get_faq, db_get_randomfaq};

pub async fn get_random_faq(server: String, conn: &Pool<Postgres>) -> String {
    let res = db_get_randomfaq(server, conn).await;

    match res {
        Ok(v) => {
            let msg = format!("#{}: {}", v.entrynum, v.message);

            msg
        },
        Err(_e) => {
            "No faqs found".to_string()
        }
    }
}

pub async fn get_faq(entry: i64, server: String, conn: &Pool<Postgres>) -> String {
    let res = db_get_faq(entry, server, conn).await;

    match res {
        Ok(v) => {
            let msg = format!("#{}: {}", v.entrynum, v.message);

            msg
        },
        Err(_e) => {
            "Faq not found".to_string()
        }
    }
}