use sqlx::{Pool, Postgres};

use crate::database::{db_get_top_deaths, db_get_top_joins, db_get_top_kills, db_get_top_leaves, db_get_top_messages, db_get_top_nwords_hard, db_get_top_nwords_soft, db_get_top_playtime};

pub async fn get_top_nword_hard(server: String, conn: &Pool<Postgres>) -> String {
    let res = db_get_top_nwords_hard(server, conn).await;

    match res {
        Ok(v) => {
            let mut final_string = String::new();

            let mut i = 0;
            for item in v.arr {
                i += 1;

                let formatted_item = format!("{}. {}: {}, ", i, item.player_name, item.hard);

                final_string += &formatted_item;
            }

            final_string
        },
        Err(_e) => {
            // e.to_string()

            "Empty".to_string()
        }
    }
}

pub async fn get_top_nword_soft(server: String, conn: &Pool<Postgres>) -> String {
    let res = db_get_top_nwords_soft(server, conn).await;

    match res {
        Ok(v) => {
            let mut final_string = String::new();

            let mut i = 0;
            for item in v.arr {
                i += 1;

                let formatted_item = format!("{}. {}: {}, ", i, item.player_name, item.soft);

                final_string += &formatted_item;
            }

            final_string
        },
        Err(_e) => {
            // e.to_string()

            "Empty".to_string()
        }
    }
}

pub async fn get_top_kills(server: String, conn: &Pool<Postgres>) -> String {
    let res = db_get_top_kills(server, conn).await;

    match res {
        Ok(v) => {
            let mut final_string = String::new();

            let mut i = 0;
            for item in v.arr {
                i += 1;

                let formatted_item = format!("{}. {}: {}, ", i, item.player_name, item.kills);

                final_string += &formatted_item;
            }

            final_string
        },
        Err(_e) => {
            // e.to_string()

            "Empty".to_string()
        }
    }
}

pub async fn get_top_deaths(server: String, conn: &Pool<Postgres>) -> String {
    let res = db_get_top_deaths(server, conn).await;

    match res {
        Ok(v) => {
            let mut final_string = String::new();

            let mut i = 0;
            for item in v.arr {
                i += 1;

                let formatted_item = format!("{}. {}: {}, ", i, item.player_name, item.deaths);

                final_string += &formatted_item;
            }

            final_string
        },
        Err(_e) => {
            // e.to_string()

            "Empty".to_string()
        }
    }
}

pub async fn get_top_playtime(server: String, conn: &Pool<Postgres>) -> String {
    let res = db_get_top_playtime(server, conn).await;

    match res {
        Ok(v) => {
            let mut final_string = String::new();

            let mut i = 0;
            for item in v.arr {
                i += 1;

                let days = item.seconds / (24 * 60 * 60);
                let hours = (item.seconds % (24 * 60 * 60)) / (60 * 60);
                let minutes = (item.seconds % (60 * 60)) / 60;

                let mut final_pt_string = String::new();

                if days > 0 {
                    let formatted_days = format!("{days}d, ");

                    final_pt_string += &formatted_days;
                }

                if hours > 0 {
                    let formatted_hours = format!("{hours}h, ");

                    final_pt_string += &formatted_hours;
                }

                if minutes > 0 {
                    let formatted_minutes = format!("{minutes}m");

                    final_pt_string += &formatted_minutes;
                }

                let formatted_item = format!("{}. {}: {}, ", i, item.player_name, final_pt_string);

                final_string += &formatted_item;
            }

            final_string
        },
        Err(_e) => {
            // e.to_string()

            "Empty".to_string()
        }
    }
}

pub async fn get_top_messages(server: String, conn: &Pool<Postgres>) -> String {
    let res = db_get_top_messages(server, conn).await;

    match res {
        Ok(v) => {
            let mut final_string = String::new();

            let mut i = 0;
            for item in v.arr {
                i += 1;

                let formatted_item = format!("{}. {}: {}, ", i, item.player_name, item.count);

                final_string += &formatted_item;
            }

            final_string
        },
        Err(_e) => {
            // e.to_string()

            "Empty".to_string()
        }
    }
}

pub async fn get_top_joins(server: String, conn: &Pool<Postgres>) -> String {
    let res = db_get_top_joins(server, conn).await;

    match res {
        Ok(v) => {
            let mut final_string = String::new();

            let mut i = 0;
            for item in v.arr {
                i += 1;

                let formatted_item = format!("{}. {}: {}, ", i, item.player_name, item.joins);

                final_string += &formatted_item;
            }

            final_string
        },
        Err(_e) => {
            // e.to_string()

            "Empty".to_string()
        }
    }
}

pub async fn get_top_leaves(server: String, conn: &Pool<Postgres>) -> String {
    let res = db_get_top_leaves(server, conn).await;

    match res {
        Ok(v) => {
            let mut final_string = String::new();

            let mut i = 0;
            for item in v.arr {
                i += 1;

                let formatted_item = format!("{}. {}: {}, ", i, item.player_name, item.leaves);

                final_string += &formatted_item;
            }

            final_string
        },
        Err(_e) => {
            // e.to_string()

            "Empty".to_string()
        }
    }
}