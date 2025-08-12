use std::{collections::HashMap, fs::{self, File}, io::Read, process, sync::Arc, time::{Duration, SystemTime}};
use parking_lot::{Mutex};

use rand::{rng, Rng};
use regex::Regex;
use serde::{Serialize, Deserialize};
use azalea::{prelude::*, Vec3};
use sqlx::{types::chrono::{DateTime, Utc}, Pool, Postgres};
use tokio::time::sleep;
use uuid::{Uuid};
use mc_bot::{commands, database::{connect_db, db_batch_update_playtime, db_get_joinmessage, db_get_leavemessage, db_insert_chatlog, db_insert_joindate, db_update_chatcount, db_update_death, db_update_joins, db_update_kill, db_update_last_death, db_update_last_kill, db_update_leaves, db_update_nword_hard, db_update_nword_soft, db_update_playtime, db_update_seen}};

#[derive(Debug)]
enum MatchFirstLetterError {
    EmptyBytesArray,
    EmptyMatchBytesArray
}

fn match_first_letter(letter_to_match: String, text: &String) -> Result<bool, MatchFirstLetterError> {
    let bytes = text.as_bytes();
    let match_bytes = letter_to_match.as_bytes();

    if bytes.len() == 0 {
        return Err(MatchFirstLetterError::EmptyBytesArray);
    }

    if match_bytes.len() == 0 {
        return Err(MatchFirstLetterError::EmptyMatchBytesArray);
    }

    if bytes[0] == match_bytes[0] {
        Ok(true)
    } else {
        Ok(false)
    }
}

// set a server ip or name, whatever you want
const SERVER: &str = "";

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let account = Account::offline("bot");
    // or let account = Account::microsoft("email").await.unwrap();

    let res = get_conf();

    if res.is_err() {
        panic!("Failed to get config");
    }

    let conf = res.unwrap();

    let pool = connect_db(conf.max_connections)
        .await
        .unwrap();

    let state = State {
        pg_pool: Arc::new(Mutex::new(Some(pool))),
        config: Arc::new(Mutex::new(conf)),
        ..Default::default()
    };

    ClientBuilder::new()
        .set_handler(handle)
        .set_state(state)
        .start(account, "localhost")
        .await
        .unwrap();
}

#[derive(Debug, Clone)]
pub struct PlayerChatMessageVec {
    pub player_name: String,
    pub message: String,
    pub timestamp: DateTime<Utc>,
    pub server: String
}

#[derive(Debug, Clone)]
pub struct PlayerMessagesCount {
    pub player_name: String,
    pub count: i32,
    pub server: String
}

#[derive(Default, Clone, Component, Debug)]
pub struct State {
    // config
    pub config: Arc<Mutex<Conf>>,
    // time since last anti afk action
    pub time_sice_last_action: Arc<Mutex<Option<SystemTime>>>,
    // time since last join message sent
    pub time_since_last_jm: Arc<Mutex<Option<SystemTime>>>,
    // time since last leave message sent
    pub time_since_last_lm: Arc<Mutex<Option<SystemTime>>>,
    // used for playtime, when a player leaves or a bot disconnects
    pub player_join_time_hashmap: Arc<Mutex<HashMap<String, SystemTime>>>,
    // pg pool
    pub pg_pool: Arc<Mutex<Option<Pool<Postgres>>>>,
    // tps
    pub ticks_count: Arc<Mutex<u64>>,
    pub bot_join_time: Arc<Mutex<Option<SystemTime>>>,
    // welcome message
    pub time_since_last_wm: Arc<Mutex<Option<SystemTime>>>,
}

fn get_conf() -> Result<Conf, String> {
    let paths = fs::read_dir("./");

    match paths {
        Ok(p_result) => {
            for path in p_result {
                match path {
                    Ok(v) => {
                        if v.file_name() == "config.json" {
                            let file = File::open(v.file_name());
                            
                            match file {
                                Ok(mut v) => {
                                    let mut contents = String::new();

                                    let _ = v.read_to_string(&mut contents);

                                    let conf_struct: Conf = serde_json::from_str(&contents).expect("Json file is not formatted well!");

                                    return Ok(conf_struct)
                                },
                                Err(e) => {
                                    println!("{}", e);

                                    return Err(e.to_string())
                                }
                            }
                        } else {
                            // idk
                            // return Err("File not found".to_string())
                        }
                    },
                    Err(e) => {
                        println!("{}", e);

                        return Err(e.to_string())
                    }
                }
            }

            return Err("Failed to read paths".to_string())
        },
        Err(e) => {
            println!("{}", e);

            return Err(e.to_string())
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Conf {
    owner: String,
    max_connections: u32,
    max_message_len: i32,
    nwords: NwordConf,
    excluded_entities: Vec<String>,
    illegal_characters_in_username: Vec<String>,
    regex: RegexConf,
    use_join_regex: bool,
    use_leave_regex: bool,
    chat_bot: bool,
    anti_afk: bool,
    send_welcome_message: bool,
    welcome_messages: Vec<String>
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct NwordConf {
    hard: Vec<String>,
    soft: Vec<String>
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RegexConf {
    join: String,
    leave: String,
    death1: String,
    death2: String,
    death3: String
}

fn turn_text_to_cool_font(text: String) -> String {
    let font_hash_map: HashMap<String, &str> = [
        ("a".to_string(), "ａ"), ("b".to_string(), "ｂ"), ("c".to_string(), "ｃ"), ("d".to_string(), "ｄ"), ("e".to_string(), "ｅ"),
        ("f".to_string(), "ｆ"), ("g".to_string(), "ｇ"), ("h".to_string(), "ｈ"), ("i".to_string(), "ｉ"), ("j".to_string(), "ｊ"), ("k".to_string(), "ｋ"),
        ("l".to_string(), "ｌ"), ("m".to_string(), "ｍ"), ("n".to_string(), "ｎ"), ("o".to_string(), "ｏ"), ("p".to_string(), "ｐ"), ("q".to_string(), "ｑ"),
        ("r".to_string(), "ｒ"), ("s".to_string(), "ｓ"), ("t".to_string(), "ｔ"), ("u".to_string(), "ｕ"), ("v".to_string(), "ｖ"), ("w".to_string(), "ｗ"),
        ("x".to_string(), "ｘ"), ("y".to_string(), "ｙ"), ("z".to_string(), "ｚ")
    ].into();

    let mut final_string = String::new();

    let splitted_text = text.split("");

    for char in splitted_text {
        let lower_char = char.to_lowercase();

        let mapped_char = font_hash_map.get(&lower_char);

        match mapped_char {
            Some(v) => {
                final_string += v;
            },
            None => {
                final_string += char;
            }
        }
    }

    final_string
}

fn is_excluded_entity(player: String, excluded_entities: Vec<String>) -> bool {
    if excluded_entities.contains(&player.to_lowercase()) {
        true
    } else {
        false
    }
}

fn name_contains_illegal_chars(player: String, illegal_chars: Vec<String>) -> bool {
    for char in illegal_chars {
        if player.to_lowercase().contains(&char) {
            return true
        }
    }

    false
}

fn send_msg(bot: &Client, message: String, state: &State) {
    let max_len = state.config.lock().max_message_len as usize;

    if message.len() > max_len {
        let chars_to_remove = message.len() - max_len;

        let msg = &message[0..message.len() - chars_to_remove];

        let formatted_msg = format!("> {}", msg);

        bot.chat(&formatted_msg);
    } else {
        let msg = format!("> {}", message);

        bot.chat(&msg);
    }
}

async fn handle(bot: Client, event: Event, state: State) -> anyhow::Result<()> {
    match event {
        Event::Chat(e) => {
            println!("[MSGSTR] {}", e.message().to_ansi());

            let message = e.message().to_string();

            let death1_regex_pattern = state.config.lock().clone().regex.death1;
            let death2_regex_pattern = state.config.lock().clone().regex.death2;
            let death3_regex_pattern = state.config.lock().clone().regex.death3;

            let death1_regex = Regex::new(&death1_regex_pattern).unwrap();
            let death2_regex = Regex::new(&death2_regex_pattern).unwrap();
            let death3_regex = Regex::new(&death3_regex_pattern).unwrap();

            let match_res1 = death1_regex.captures(&message);
            let match_res2 = death2_regex.captures(&message);
            let match_res3 = death3_regex.captures(&message);

            match match_res1 {
                Some(v) => {
                    let conn_lock = state.pg_pool.lock().clone();
                    let conn = conn_lock.as_ref();

                    match conn {
                        Some(pg_conn) => {
                            let victim = v.get(1);
                            let killer = v.get(2);

                            match victim {
                                Some(val) => {
                                    let name = val.as_str();

                                    if name.ends_with("'s") {
                                        let replaced_name = &name.replace("'s", "");

                                        db_update_death(replaced_name.to_string(), SERVER.to_string(), pg_conn).await;

                                        let timestamp: DateTime<Utc> = Utc::now();

                                        db_update_last_death(replaced_name.to_string(), message.clone(), timestamp, SERVER.to_string(), pg_conn).await;
                                    } else {
                                        db_update_death(name.to_string(), SERVER.to_string(), pg_conn).await;

                                        let timestamp: DateTime<Utc> = Utc::now();

                                        db_update_last_death(name.to_string(), message.clone(), timestamp, SERVER.to_string(), pg_conn).await;
                                    }
                                },
                                None => {}
                            }

                            match killer {
                                Some(val) => {
                                    let name = val.as_str();

                                    let excluded_entities = state.config.lock().clone().excluded_entities;

                                    let is_excluded = is_excluded_entity(name.to_string(), excluded_entities);

                                    if is_excluded == false {
                                        if name.ends_with("'s") {
                                            let replaced_name = &name.replace("'s", "");

                                            db_update_kill(replaced_name.to_string(), SERVER.to_string(), pg_conn).await;

                                            let timestamp: DateTime<Utc> = Utc::now();

                                            db_update_last_kill(replaced_name.to_string(), message.clone(), timestamp, SERVER.to_string(), pg_conn).await;
                                        } else {
                                            db_update_kill(name.to_string(), SERVER.to_string(), pg_conn).await;

                                            let timestamp: DateTime<Utc> = Utc::now();

                                            db_update_last_kill(name.to_string(), message.clone(), timestamp, SERVER.to_string(), pg_conn).await;
                                        }
                                    }
                                },
                                None => {}
                            }
                        },
                        None => {}
                    }
                },
                None => {}
            }

            match match_res2 {
                Some(v) => {
                    let conn_lock = state.pg_pool.lock().clone();
                    let conn = conn_lock.as_ref();

                    match conn {
                        Some(pg_conn) => {
                            let victim = v.get(1);

                            match victim {
                                Some(val) => {
                                    let name = val.as_str();

                                    db_update_death(name.to_string(), SERVER.to_string(), pg_conn).await;

                                    let timestamp: DateTime<Utc> = Utc::now();

                                    db_update_last_death(name.to_string(), message.clone(), timestamp, SERVER.to_string(), pg_conn).await;
                                },
                                None => {}
                            }
                        },
                        None => {}
                    }
                },
                None => {}
            }

            match match_res3 {
                Some(v) => {
                    let conn_lock = state.pg_pool.lock().clone();
                    let conn = conn_lock.as_ref();

                    match conn {
                        Some(pg_conn) => {
                            let killer = v.get(1);
                            let victim = v.get(2);

                            match killer {
                                Some(val) => {
                                    let name = val.as_str();

                                    let illegal_chars = state.config.lock().clone().illegal_characters_in_username;

                                    let does_contain = name_contains_illegal_chars(name.to_string(), illegal_chars);

                                    if does_contain == false {
                                        let excluded_entities = state.config.lock().clone().excluded_entities;

                                        let is_excluded = is_excluded_entity(name.to_string(), excluded_entities);

                                        if is_excluded == false {
                                            if name.ends_with("'s") {
                                                let replaced_name = &name.replace("'s", "");

                                                db_update_kill(replaced_name.to_string(), SERVER.to_string(), pg_conn).await;

                                                let timestamp: DateTime<Utc> = Utc::now();

                                                db_update_last_kill(replaced_name.to_string(), message.clone(), timestamp, SERVER.to_string(), pg_conn).await;
                                            } else {
                                                db_update_kill(name.to_string(), SERVER.to_string(), pg_conn).await;

                                                let timestamp: DateTime<Utc> = Utc::now();

                                                db_update_last_kill(name.to_string(), message.clone(), timestamp, SERVER.to_string(), pg_conn).await;
                                            }
                                        }
                                    }
                                },
                                None => {}
                            }

                            match victim {
                                Some(val) => {
                                    let name = val.as_str();

                                    db_update_death(name.to_string(), SERVER.to_string(), pg_conn).await;

                                    let timestamp: DateTime<Utc> = Utc::now();

                                    db_update_last_death(name.to_string(), message.clone(), timestamp, SERVER.to_string(), pg_conn).await;
                                },
                                None => {}
                            }
                        },
                        None => {}
                    }
                },
                None => {}
            }

            let (sender, content) = e.split_sender_and_content();

            match sender {
                Some(name) => {
                    println!("{name}: {content}");

                    let conn_lock = state.pg_pool.lock().clone();
                    let conn = conn_lock.as_ref();

                    match conn {
                        Some(pg_conn) => {
                            let timestamp: DateTime<Utc> = Utc::now();

                            db_insert_chatlog(&name, &content, timestamp, SERVER.to_string(), pg_conn).await;

                            db_update_chatcount(&name, SERVER.to_string(), pg_conn).await;
                        },
                        None => {}
                    }

                    let nword_val = state.config.lock().nwords.clone();

                    let hard_nword_vec = nword_val.hard;
                    let soft_nword_vec = nword_val.soft;

                    if hard_nword_vec.len() > 0 {
                        for hard_nword in hard_nword_vec {
                            if content.contains(&hard_nword) {
                                let swords = content.split(" ");
                                let words: Vec<&str> = swords.collect();

                                let mut nword_count = 0;

                                for word in words {
                                    if word.contains(&hard_nword) {
                                        nword_count += 1
                                    }
                                }

                                let conn_lock = state.pg_pool.lock().clone();
                                let conn = conn_lock.as_ref();

                                match conn {
                                    Some(pg_conn) => {
                                        db_update_nword_hard(name.clone(), nword_count, SERVER.to_string(), pg_conn).await;
                                    },
                                    None => {}
                                }
                            }
                        }
                    }

                    if soft_nword_vec.len() > 0 {
                        for soft_nword in soft_nword_vec {
                            if content.contains(&soft_nword) {
                                let swords = content.split(" ");
                                let words: Vec<&str> = swords.collect();

                                let mut nword_count = 0;

                                for word in words {
                                    if word.contains(&soft_nword) {
                                        nword_count += 1
                                    }
                                }

                                let conn_lock = state.pg_pool.lock().clone();
                                let conn = conn_lock.as_ref();

                                match conn {
                                    Some(pg_conn) => {
                                        db_update_nword_soft(name.clone(), nword_count, SERVER.to_string(), pg_conn).await;
                                    },
                                    None => {}
                                }
                            }
                        }
                    }

                    if name == bot.username() {
                        return Ok(());
                    }

                    let is_chat_bot = state.config.lock().clone().chat_bot;

                    if is_chat_bot == false {
                        return Ok(());
                    }

                    let t = String::from("!");
                    let flm = match_first_letter(t, &content);

                    let v = flm.expect("Expected bool!");

                    if v == false {
                        return Ok(());
                    }

                    let mut s_con = String::from(&content);
                    s_con.remove(0);

                    let col = s_con.split(" ");
                    let args: Vec<&str> = col.collect();

                    if args[0].to_lowercase().starts_with("help") {
                        let msg = format!("Shut up");

                        send_msg(&bot, msg, &state);
                    }

                    if args[0].to_lowercase().starts_with("players") {
                        let conn_lock = state.pg_pool.lock().clone();
                        let conn = conn_lock.as_ref();

                        match conn {
                            Some(pg_conn) => {
                                let res = commands::players::get_total_players(SERVER.to_string(), pg_conn).await;

                                send_msg(&bot, res, &state);
                            },
                            None => {
                                send_msg(&bot, "Failed to connect to database".to_string(), &state);
                            }
                        }
                    }

                    if args[0].to_lowercase().starts_with("bp") {
                        let res = commands::bestping::get_best_ping(bot.tab_list());

                        send_msg(&bot, res, &state);
                    }

                    if args[0].to_lowercase().starts_with("wp") {
                        let res = commands::worstping::get_worst_ping(bot.tab_list());

                        send_msg(&bot, res, &state);
                    }

                    if args[0].to_lowercase().starts_with("ping") {
                        if args.len() > 1 && args[1].len() > 0 {
                            let player_uuid: Option<Uuid> = bot.player_uuid_by_username(args[1]);

                            match player_uuid {
                                Some(val) => {
                                    let tablist = bot.tab_list();

                                    let res = commands::ping::get_ping(args[1], tablist, val);

                                    send_msg(&bot, res, &state);
                                },
                                None => {
                                    let msg = format!("Is {} online?", args[1]);
                                    send_msg(&bot, msg, &state);
                                },
                            }
                        } else {
                            let player_uuid: Option<Uuid> = bot.player_uuid_by_username(&name);

                            match player_uuid {
                                Some(val) => {
                                    let tablist = bot.tab_list();

                                    let res = commands::ping::get_ping(&name, tablist, val);

                                    send_msg(&bot, res, &state);
                                },
                                None => {
                                    let msg = format!("Is {name} online?");
                                    send_msg(&bot, msg, &state);
                                },
                            }
                        }
                    }

                    if args[0].to_lowercase().starts_with("top") {
                        if args.len() < 2 {
                            let msg = format!("Choose: nword, kills, deaths, playtime, messages, joins, leaves");

                            send_msg(&bot, msg, &state);

                            return Ok(());
                        }

                        if args[1].to_lowercase().starts_with("nword") {
                            if args.len() < 3 {
                                let msg = format!("Choose: nword soft/hard");

                                send_msg(&bot, msg, &state);

                                return Ok(());
                            }

                            if args[2].to_lowercase().starts_with("hard") {
                                let conn_lock = state.pg_pool.lock().clone();
                                let conn = conn_lock.as_ref();

                                match conn {
                                    Some(pg_conn) => {
                                        let res = commands::top::get_top_nword_hard(SERVER.to_string(), pg_conn).await;

                                        send_msg(&bot, res, &state);
                                    },
                                    None => {
                                        send_msg(&bot, "Failed to connect to database".to_string(), &state);
                                    }
                                }
                            }

                            if args[2].to_lowercase().starts_with("soft") {
                                let conn_lock = state.pg_pool.lock().clone();
                                let conn = conn_lock.as_ref();

                                match conn {
                                    Some(pg_conn) => {
                                        let res = commands::top::get_top_nword_soft(SERVER.to_string(), pg_conn).await;

                                        send_msg(&bot, res, &state);
                                    },
                                    None => {
                                        send_msg(&bot, "Failed to connect to database".to_string(), &state);
                                    }
                                }
                            }
                        }

                        if args[1].to_lowercase().starts_with("kill") {
                            let conn_lock = state.pg_pool.lock().clone();
                            let conn = conn_lock.as_ref();

                            match conn {
                                Some(pg_conn) => {
                                    let res = commands::top::get_top_kills(SERVER.to_string(), pg_conn).await;

                                    send_msg(&bot, res, &state);
                                },
                                None => {
                                    send_msg(&bot, "Failed to connect to database".to_string(), &state);
                                }
                            }
                        }

                        if args[1].to_lowercase().starts_with("death") {
                            let conn_lock = state.pg_pool.lock().clone();
                            let conn = conn_lock.as_ref();

                            match conn {
                                Some(pg_conn) => {
                                    let res = commands::top::get_top_deaths(SERVER.to_string(), pg_conn).await;

                                    send_msg(&bot, res, &state);
                                },
                                None => {
                                    send_msg(&bot, "Failed to connect to database".to_string(), &state);
                                }
                            }
                        }

                        if args[1].to_lowercase().starts_with("playtime") || args[1].to_lowercase().starts_with("pt") {
                            let conn_lock = state.pg_pool.lock().clone();
                            let conn = conn_lock.as_ref();
                            
                            match conn {
                                Some(pg_conn) => {
                                    let res = commands::top::get_top_playtime(SERVER.to_string(), pg_conn).await;

                                    send_msg(&bot, res, &state);
                                },
                                None => {
                                    send_msg(&bot, "Failed to connect to database".to_string(), &state);
                                }
                            }
                        }

                        if args[1].to_lowercase().starts_with("message") {
                            let conn_lock = state.pg_pool.lock().clone();
                            let conn = conn_lock.as_ref();

                            match conn {
                                Some(pg_conn) => {
                                    let res = commands::top::get_top_messages(SERVER.to_string(), pg_conn).await;

                                    send_msg(&bot, res, &state);
                                },
                                None => {
                                    send_msg(&bot, "Failed to connect to database".to_string(), &state);
                                }
                            }
                        }

                        if args[1].to_lowercase().starts_with("join") {
                            let conn_lock = state.pg_pool.lock().clone();
                            let conn = conn_lock.as_ref();

                            match conn {
                                Some(pg_conn) => {
                                    let res = commands::top::get_top_joins(SERVER.to_string(), pg_conn).await;

                                    send_msg(&bot, res, &state);
                                },
                                None => {
                                    send_msg(&bot, "Failed to connect to database".to_string(), &state);
                                }
                            }
                        }

                        if args[1].to_lowercase().starts_with("leave") {
                            let conn_lock = state.pg_pool.lock().clone();
                            let conn = conn_lock.as_ref();

                            match conn {
                                Some(pg_conn) => {
                                    let res = commands::top::get_top_leaves(SERVER.to_string(), pg_conn).await;

                                    send_msg(&bot, res, &state);
                                },
                                None => {
                                    send_msg(&bot, "Failed to connect to database".to_string(), &state);
                                }
                            }
                        }
                    }

                    if args[0].to_lowercase().starts_with("join") {
                        let conn_lock = state.pg_pool.lock().clone();
                        let conn = conn_lock.as_ref();

                        match conn {
                            Some(pg_conn) => {
                                if args.len() > 1 {
                                    let res = commands::joins::get_joins(&args[1].to_lowercase(), SERVER.to_string(), pg_conn).await;

                                    send_msg(&bot, res, &state);
                                } else {
                                    let res = commands::joins::get_joins(&name, SERVER.to_string(), pg_conn).await;

                                    send_msg(&bot, res, &state);
                                }
                            },
                            None => {
                                send_msg(&bot, "Failed to connect to database".to_string(), &state);
                            }
                        }
                    }

                    if args[0].to_lowercase().starts_with("leave") {
                        let conn_lock = state.pg_pool.lock().clone();
                        let conn = conn_lock.as_ref();

                        match conn {
                            Some(pg_conn) => {
                                if args.len() > 1 {
                                    let res = commands::leaves::get_leaves(&args[1].to_lowercase(), SERVER.to_string(), pg_conn).await;

                                    send_msg(&bot, res, &state);
                                } else {
                                    let res = commands::leaves::get_leaves(&name, SERVER.to_string(), pg_conn).await;

                                    send_msg(&bot, res, &state);
                                }
                            },
                            None => {
                                send_msg(&bot, "Failed to connect to database".to_string(), &state);
                            }
                        }
                    }

                    if args[0].to_lowercase().starts_with("message") {
                        let conn_lock = state.pg_pool.lock().clone();
                        let conn = conn_lock.as_ref();

                        match conn {
                            Some(pg_conn) => {
                                if args.len() > 1 {
                                    let res = commands::messages::get_messages(&args[1].to_lowercase(), SERVER.to_string(), pg_conn).await;

                                    send_msg(&bot, res, &state);
                                } else {
                                    let res = commands::messages::get_messages(&name.to_lowercase(), SERVER.to_string(), pg_conn).await;

                                    send_msg(&bot, res, &state);
                                }
                            },
                            None => {
                                send_msg(&bot, "Failed to connect to database".to_string(), &state);
                            }
                        }
                    }

                    if args[0].to_lowercase().starts_with("curse") {
                        if args.len() < 2 {
                            let msg = format!("You don't want to curse yourself, do you?");

                            send_msg(&bot, msg, &state);
                        
                            return Ok(());
                        }

                        let msg = format!("{} cursed {} with a terrible curse! Remove it with /kill", name, args[1]);

                        send_msg(&bot, msg, &state);
                    }

                    if args[0].to_lowercase().starts_with("kick") {
                        if args.len() < 2 {
                            let msg = format!("{} kicked himself", name);

                            send_msg(&bot, msg, &state);

                            return Ok(());
                        }

                        let msg = format!("{} kicked {}", name, args[1]);

                        send_msg(&bot, msg, &state);
                    }

                    if args[0].to_lowercase().starts_with("ban") {
                        if args.len() < 2 {
                            let msg = format!("{} banned himself", name);

                            send_msg(&bot, msg, &state);

                            return Ok(());
                        }

                        let msg = format!("{} banned {}", name, args[1]);

                        send_msg(&bot, msg, &state);
                    }

                    if args[0].to_lowercase().starts_with("mute") {
                        if args.len() < 2 {
                            let msg = format!("{} muted himself", name);

                            send_msg(&bot, msg, &state);

                            return Ok(());
                        }

                        let msg = format!("{} muted {}", name, args[1]);

                        send_msg(&bot, msg, &state);
                    }

                    if args[0].to_lowercase().starts_with("firstword") || args[0].to_lowercase().starts_with("firstmessage") || args[0].to_lowercase().starts_with("firstmsg") {
                        let conn_lock = state.pg_pool.lock().clone();
                        let conn = conn_lock.as_ref();

                        match conn {
                            Some(pg_conn) => {
                                if args.len() > 1 && args[1].len() > 0 {
                                    let res = commands::firstwords::get_firstwords(&args[1].to_string(), SERVER.to_string(), pg_conn).await;

                                    send_msg(&bot, res, &state);
                                } else {
                                    let res = commands::firstwords::get_firstwords(&name, SERVER.to_string(), pg_conn).await;

                                    send_msg(&bot, res, &state);
                                }
                            },
                            None => {
                                send_msg(&bot, "Failed to connect to database".to_string(), &state);
                            }
                        }
                    }

                    if args[0].to_lowercase().starts_with("lastword") || args[0].to_lowercase().starts_with("lastmessage") || args[0].to_lowercase().starts_with("lastmsg") {
                        let conn_lock = state.pg_pool.lock().clone();
                        let conn = conn_lock.as_ref();

                        match conn {
                            Some(pg_conn) => {
                                if args.len() > 1 {
                                    let res = commands::lastwords::get_lastwords(&args[1].to_string(), SERVER.to_string(), pg_conn).await;

                                    send_msg(&bot, res, &state);
                                } else {
                                    let res = commands::lastwords::get_lastwords(&name, SERVER.to_string(), pg_conn).await;

                                    send_msg(&bot, res, &state);
                                }
                            },
                            None => {
                                send_msg(&bot, "Failed to connect to database".to_string(), &state);
                            }
                        }
                    }

                    if args[0].to_lowercase().starts_with("lastkill") {
                        let conn_lock = state.pg_pool.lock().clone();
                        let conn = conn_lock.as_ref();

                        match conn {
                            Some(pg_conn) => {
                                if args.len() > 1 {
                                    let res = commands::lastkill::get_lastkill(&args[1].to_string(), SERVER.to_string(), pg_conn).await;

                                    send_msg(&bot, res, &state);
                                } else {
                                    let res = commands::lastkill::get_lastkill(&name, SERVER.to_string(), pg_conn).await;

                                    send_msg(&bot, res, &state);
                                }
                            },
                            None => {
                                send_msg(&bot, "Failed to connect to database".to_string(), &state);
                            }
                        }
                    }

                    if args[0].to_lowercase().starts_with("firstkill") {
                        let conn_lock = state.pg_pool.lock().clone();
                        let conn = conn_lock.as_ref();

                        match conn {
                            Some(pg_conn) => {
                                if args.len() > 1 {
                                    let res = commands::firstkill::get_firstkill(&args[1].to_string(), SERVER.to_string(), pg_conn).await;

                                    send_msg(&bot, res, &state);
                                } else {
                                    let res = commands::firstkill::get_firstkill(&name, SERVER.to_string(), pg_conn).await;

                                    send_msg(&bot, res, &state);
                                }
                            },
                            None => {
                                send_msg(&bot, "Failed to connect to database".to_string(), &state);
                            }
                        }
                    }

                    if args[0].to_lowercase().starts_with("lastdeath") {
                        let conn_lock = state.pg_pool.lock().clone();
                        let conn = conn_lock.as_ref();

                        match conn {
                            Some(pg_conn) => {
                                if args.len() > 1 {
                                    let res = commands::lastdeath::get_lastdeath(&args[1].to_string(), SERVER.to_string(), pg_conn).await;

                                    send_msg(&bot, res, &state);
                                } else {
                                    let res = commands::lastdeath::get_lastdeath(&name, SERVER.to_string(), pg_conn).await;
                                
                                    send_msg(&bot, res, &state);
                                }
                            },
                            None => {
                                send_msg(&bot, "Failed to connect to database".to_string(), &state);
                            }
                        }
                    }

                    if args[0].to_lowercase().starts_with("firstdeath") {
                        let conn_lock = state.pg_pool.lock().clone();
                        let conn = conn_lock.as_ref();

                        match conn {
                            Some(pg_conn) => {
                                if args.len() > 1 {
                                    let res = commands::firstdeath::get_firstdeath(&args[1].to_string(), SERVER.to_string(), pg_conn).await;

                                    send_msg(&bot, res, &state);
                                } else {
                                    let res = commands::firstdeath::get_firstdeath(&name, SERVER.to_string(), pg_conn).await;

                                    send_msg(&bot, res, &state);
                                }
                            },
                            None => {
                                send_msg(&bot, "Failed to connect to database".to_string(), &state);
                            }
                        }
                    }

                    if args[0].to_lowercase().starts_with("seen") || args[0].to_lowercase().starts_with("lastseen") {
                        let conn_lock = state.pg_pool.lock().clone();
                        let conn = conn_lock.as_ref();

                        match conn {
                            Some(pg_conn) => {
                                if args.len() > 1 {
                                    let res = commands::seen::get_seen(&args[1].to_string(), SERVER.to_string(), pg_conn).await;

                                    send_msg(&bot, res, &state);
                                } else {
                                    let res = commands::seen::get_seen(&name, SERVER.to_string(), pg_conn).await;

                                    send_msg(&bot, res, &state);
                                }
                            },
                            None => {
                                send_msg(&bot, "Failed to connect to database".to_string(), &state);
                            }
                        }
                    }

                    if args[0].to_lowercase().starts_with("jd") || args[0].to_lowercase().starts_with("joindate") || args[0].to_lowercase().starts_with("firstseen") {
                        let conn_lock = state.pg_pool.lock().clone();
                        let conn = conn_lock.as_ref();

                        match conn {
                            Some(pg_conn) => {
                                if args.len() > 1 {
                                    let res = commands::joindate::get_joindate(&args[1].to_string(), SERVER.to_string(), pg_conn).await;

                                    send_msg(&bot, res, &state);
                                } else {
                                    let res = commands::joindate::get_joindate(&name, SERVER.to_string(), pg_conn).await;

                                    send_msg(&bot, res, &state);
                                }
                            },
                            None => {
                                send_msg(&bot, "Failed to connect to database".to_string(), &state);
                            }
                        }
                    }

                    if args[0].to_lowercase().starts_with("kill") {
                        if args[0].to_lowercase().starts_with("kills") {
                            let conn_lock = state.pg_pool.lock().clone();
                            let conn = conn_lock.as_ref();

                            match conn {
                                Some(pg_conn) => {
                                    if args.len() > 1 {
                                        let res = commands::kd::get_kd(&args[1].to_string(), SERVER.to_string(), pg_conn).await;

                                        send_msg(&bot, res, &state);
                                    } else {
                                        let res = commands::kd::get_kd(&name, SERVER.to_string(), pg_conn).await;

                                        send_msg(&bot, res, &state);
                                    }
                                },
                                None => {
                                    send_msg(&bot, "Failed to connect to database".to_string(), &state);
                                }
                            }
                        } else {
                            bot.chat("/kill");
                        }
                    }

                    if args[0].to_lowercase().starts_with("death") || args[0].to_lowercase().starts_with("kd") {
                        let conn_lock = state.pg_pool.lock().clone();
                        let conn = conn_lock.as_ref();

                        match conn {
                            Some(pg_conn) => {
                                if args.len() > 1 {
                                    let res = commands::kd::get_kd(&args[1].to_string(), SERVER.to_string(), pg_conn).await;

                                    send_msg(&bot, res, &state);
                                } else {
                                    let res = commands::kd::get_kd(&name, SERVER.to_string(), pg_conn).await;

                                    send_msg(&bot, res, &state);
                                }
                            },
                            None => {
                                send_msg(&bot, "Failed to connect to database".to_string(), &state);
                            }
                        }
                    }

                    if args[0].to_lowercase().starts_with("coord") {
                        let msg = format!("My coordinates are {:.1} {:.1} {:.1}", bot.position().x, bot.position().y, bot.position().z);

                        send_msg(&bot, msg, &state);
                    }

                    if args[0].to_lowercase().starts_with("rules") {
                        let msg = format!("Rules: No Hacking, No Griefing, No spamming!");

                        send_msg(&bot, msg, &state);
                    }

                    if args[0].to_lowercase().starts_with("no") {
                        send_msg(&bot, "NO!".to_string(), &state);
                    }

                    if args[0].to_lowercase().starts_with("yes") {
                        send_msg(&bot, "YES!".to_string(), &state);
                    }

                    if args[0].to_lowercase().starts_with("dupe") {
                        if args.len() > 1 {
                            let res = commands::dupe::dupe(&name, args[1].to_string());

                            send_msg(&bot, res, &state);
                        } else {
                            let res = commands::dupe::dupe(&name, "Air".to_string());

                            send_msg(&bot, res, &state);
                        }
                    }

                    if args[0].to_lowercase().starts_with("locate") {
                        if args.len() > 1 {
                            let res = commands::locate::locate(&args[1].to_string());

                            send_msg(&bot, res, &state);
                        } else {
                            let res = commands::locate::locate(&name);

                            send_msg(&bot, res, &state);
                        }
                    }

                    if args[0].to_lowercase().starts_with("playtime") || args[0].to_lowercase().starts_with("pt") {
                        let conn_lock = state.pg_pool.lock().clone();
                        let conn = conn_lock.as_ref();

                        match conn {
                            Some(pg_conn) => {
                                if args.len() > 1 {
                                    let lock = state.player_join_time_hashmap.lock().clone();
                                    let join_time = lock.get(&args[1].to_lowercase());

                                    match join_time {
                                        Some(v) => {
                                            let elapsed = v.elapsed();

                                            match elapsed {
                                                Ok(val) => {
                                                    let secs = val.as_secs() as i64;

                                                    let res = commands::playtime::get_playtime(&args[1].to_string(), secs, SERVER.to_string(), pg_conn).await;

                                                    send_msg(&bot, res, &state);
                                                },
                                                Err(e) => {
                                                    println!("{}", e);

                                                    let res = commands::playtime::get_playtime(&args[1].to_string(), 0, SERVER.to_string(), pg_conn).await;

                                                    send_msg(&bot, res, &state);
                                                }
                                            }
                                        },
                                        None => {
                                            let res = commands::playtime::get_playtime(&args[1].to_string(), 0, SERVER.to_string(), pg_conn).await;

                                            send_msg(&bot, res, &state);
                                        }
                                    }
                                } else {
                                    let lock = state.player_join_time_hashmap.lock().clone();
                                    let join_time = lock.get(&name);

                                    match join_time {
                                        Some(v) => {
                                            let elapsed = v.elapsed();

                                            match elapsed {
                                                Ok(val) => {
                                                    let secs = val.as_secs() as i64;

                                                    let res = commands::playtime::get_playtime(&name, secs, SERVER.to_string(), pg_conn).await;

                                                    send_msg(&bot, res, &state);
                                                },
                                                Err(e) => {
                                                    println!("{}", e);

                                                    let res = commands::playtime::get_playtime(&name, 0, SERVER.to_string(), pg_conn).await;

                                                    send_msg(&bot, res, &state);
                                                }
                                            }
                                        },
                                        None => {
                                            let res = commands::playtime::get_playtime(&name, 0, SERVER.to_string(), pg_conn).await;

                                            send_msg(&bot, res, &state);
                                        }
                                    }
                                }
                            },
                            None => {
                                send_msg(&bot, "Failed to connect to database".to_string(), &state);
                            }
                        }
                    }

                    if args[0].to_lowercase().starts_with("nword") {
                        let conn_lock = state.pg_pool.lock().clone();
                        let conn = conn_lock.as_ref();

                        match conn {
                            Some(pg_conn) => {
                                if args.len() > 1 {
                                    let res = commands::nwords::get_nwords(&args[1].to_string(), SERVER.to_string(), pg_conn).await;

                                    send_msg(&bot, res, &state);
                                } else {
                                    let res = commands::nwords::get_nwords(&name, SERVER.to_string(), pg_conn).await;

                                    send_msg(&bot, res, &state);
                                }
                            },
                            None => {
                                send_msg(&bot, "Failed to connect to database".to_string(), &state);
                            }
                        }
                    }

                    if args[0].to_lowercase().starts_with("health") || args[0].to_lowercase().starts_with("hp") {
                        let msg = format!("Health: {:.0}", bot.health());

                        send_msg(&bot, msg, &state);
                    }

                    if args[0].to_lowercase().starts_with("food") || args[0].to_lowercase().starts_with("hunger") {
                        let hunger = bot.hunger();
                        let msg = format!("Food: {:.0}", hunger.food);

                        send_msg(&bot, msg, &state);
                    }

                    if args[0].to_lowercase().starts_with("savemsg") || args[0].to_lowercase().starts_with("savemessage") {
                        let conn_lock = state.pg_pool.lock().clone();
                        let conn = conn_lock.as_ref();

                        match conn {
                            Some(pg_conn) => {
                                let string_to_replace = format!("!{} ", args[0]);
                                let input_string = format!("!{}", args[0]);

                                let msg_content = content.replace(&string_to_replace, "");

                                if msg_content == input_string {
                                    let msg = format!("Message required!");

                                    send_msg(&bot, msg, &state);

                                    return Ok(());
                                }

                                let res = commands::savemsg::savemsg(&name, msg_content, SERVER.to_string(), pg_conn).await;

                                send_msg(&bot, res, &state);
                            },
                            None => {
                                send_msg(&bot, "Failed to connect to database".to_string(), &state);
                            }
                        }
                    }

                    if args[0].to_lowercase().starts_with("playmsg") || args[0].to_lowercase().starts_with("playmessage") {
                        let conn_lock = state.pg_pool.lock().clone();
                        let conn = conn_lock.as_ref();

                        match conn {
                            Some(pg_conn) => {
                                if args.len() > 1 {
                                    let res = commands::playmsg::get_savedmsg(&args[1].to_string(), SERVER.to_string(), pg_conn).await;

                                    send_msg(&bot, res, &state);
                                } else {
                                    let res = commands::playmsg::get_savedmsg(&name, SERVER.to_string(), pg_conn).await;

                                    send_msg(&bot, res, &state);
                                }
                            },
                            None => {
                                send_msg(&bot, "Failed to connect to database".to_string(), &state);
                            }
                        }
                    }

                    if args[0].to_lowercase().starts_with("iam") {
                        let conn_lock = state.pg_pool.lock().clone();
                        let conn = conn_lock.as_ref();

                        match conn {
                            Some(pg_conn) => {
                                let string_to_replace = format!("!{} ", args[0]);
                                let input_string = format!("!{}", args[0]);

                                let msg_content = content.replace(&string_to_replace, "");

                                if msg_content == input_string {
                                    let msg = format!("Message required!");

                                    send_msg(&bot, msg, &state);

                                    return Ok(());
                                }

                                let res = commands::iam::save_iam(&name, msg_content, SERVER.to_string(), pg_conn).await;

                                send_msg(&bot, res, &state);
                            },
                            None => {
                                send_msg(&bot, "Failed to connect to database".to_string(), &state);
                            }
                        }
                    }

                    if args[0].to_lowercase().starts_with("whois") {
                        let conn_lock = state.pg_pool.lock().clone();
                        let conn = conn_lock.as_ref();

                        match conn {
                            Some(pg_conn) => {
                                if args.len() > 1 {
                                    let res = commands::whois::get_whois(&args[1].to_string(), SERVER.to_string(), pg_conn).await;

                                    send_msg(&bot, res, &state);
                                } else {
                                    let res = commands::whois::get_whois(&name, SERVER.to_string(), pg_conn).await;

                                    send_msg(&bot, res, &state);
                                }
                            },
                            None => {
                                send_msg(&bot, "Failed to connect to database".to_string(), &state);
                            }
                        }
                    }

                    if args[0].to_lowercase().starts_with("leak") {
                        let string_to_replace = format!("!{} ", args[0]);
                        let input_string = format!("!{}", args[0]);

                        let msg_content = content.replace(&string_to_replace, "");

                        if msg_content == input_string {
                            let msg = format!("Message required!");

                            send_msg(&bot, msg, &state);

                            return Ok(());
                        }

                        let msg = format!("{} leaked {}", &name, msg_content);

                        send_msg(&bot, msg, &state);
                    }

                    if args[0].to_lowercase().starts_with("gm") || args[0].to_lowercase().starts_with("gamemode") {
                        let string_to_replace = format!("!{} ", args[0]);
                        let input_string = format!("!{}", args[0]);

                        let msg_content = content.replace(&string_to_replace, "");

                        if msg_content == input_string {
                            let msg = format!("Gamemode required!");

                            send_msg(&bot, msg, &state);

                            return Ok(());
                        }

                        let msg = format!("Set gamemode to {} for {}", msg_content, &name);

                        send_msg(&bot, msg, &state);
                    }

                    if args[0].to_lowercase().starts_with("askgod") || args[0].to_lowercase().starts_with("askallah") {
                        let msg = commands::askgod::askgod();

                        send_msg(&bot, msg, &state);
                    }

                    if args[0].to_lowercase().starts_with("give") {
                        if args.len() > 1 {
                            let string_to_replace = format!("!{} {} ", args[0], args[1]);
                            let input_string = format!("!{} {}", args[0], args[1]);

                            let msg_content = content.replace(&string_to_replace, "");

                            if msg_content == input_string {
                                let msg = format!("Item required!");

                                send_msg(&bot, msg, &state);

                                return Ok(());
                            }

                            let msg = format!("{} gave {} to {}", &name, msg_content, args[1]);

                            send_msg(&bot, msg, &state);
                        } else {
                            let msg = format!("Player is required!");

                            send_msg(&bot, msg, &state);
                        }
                    }

                    if args[0].to_lowercase().starts_with("tp") || args[0].to_lowercase().starts_with("teleport") {
                        if args[0].to_lowercase().starts_with("tps") {
                            let join_time_lock = state.bot_join_time.lock().unwrap();

                            let elapsed = join_time_lock.elapsed();

                            match elapsed {
                                Ok(v) => {
                                    let ticks_count_lock = state.ticks_count.lock().clone();

                                    let tps = ticks_count_lock as f32 / v.as_secs() as f32;

                                    let msg = format!("TPS: {:.2}", tps);

                                    send_msg(&bot, msg, &state);
                                },
                                Err(e) => {
                                    println!("{}", e);

                                    send_msg(&bot, "Failed to calculate tps".to_string(), &state);
                                }
                            }
                        } else {
                            let string_to_replace = format!("!{} ", args[0]);
                            let input_string = format!("!{}", args[0]);

                            let msg_content = content.replace(&string_to_replace, "");

                            if msg_content == input_string {
                                let msg = format!("Location required!");

                                send_msg(&bot, msg, &state);

                                return Ok(());
                            }

                            let msg = format!("{} teleported to {}", &name, msg_content);

                            send_msg(&bot, msg, &state);
                        }
                    }

                    if args[0].to_lowercase().starts_with("op") {
                        if args.len() > 1 {
                            let msg = format!("Made {} server operator", args[1].to_string());

                            send_msg(&bot, msg, &state);
                        } else {
                            let msg = format!("Made {} server operator", &name);

                            send_msg(&bot, msg, &state);
                        }
                    }

                    if args[0].to_lowercase().starts_with("bless") {
                        let string_to_replace = format!("!{} ", args[0]);
                        let input_string = format!("!{}", args[0]);

                        let msg_content = content.replace(&string_to_replace, "");

                        if msg_content == input_string {
                            let msg = format!("{} blesses himself", &name);

                            send_msg(&bot, msg, &state);

                            return Ok(());
                        }

                        let msg = format!("{} blesses {}", &name, msg_content);

                        send_msg(&bot, msg, &state);
                    }

                    if args[0].to_lowercase().starts_with("kit") {
                        let string_to_replace = format!("!{} ", args[0]);
                        let input_string = format!("!{}", args[0]);

                        let msg_content = content.replace(&string_to_replace, "");

                        if msg_content == input_string {
                            let msg = format!("Kit required!");

                            send_msg(&bot, msg, &state);

                            return Ok(());
                        }

                        let msg = format!("Gave kit {} to {}", msg_content, &name);

                        send_msg(&bot, msg, &state);
                    }

                    // todo math maybe

                    if args[0].to_lowercase().starts_with("pp") || args[0].to_lowercase().starts_with("penis") {
                        if args.len() > 1 {
                            let rand_size = rng().random_range(1..=45);

                            let formatted_pp = format!("{:=<1$}", "", rand_size);

                            let mut msg = format!("{}: 8{}D", args[1].to_string(), formatted_pp);

                            if rand_size > 40 && rand_size < 46 {
                                msg += " hude dih!!";
                            }

                            if rand_size > 30 && rand_size < 40 {
                                msg += " average dih";
                            }

                            if rand_size > 20 && rand_size < 30 {
                                msg += " ehhhhh.....";
                            }

                            if rand_size > 10 && rand_size < 20 {
                                msg += " small dih";
                            }

                            if rand_size > 1 && rand_size < 10 {
                                msg += " ..."
                            }

                            send_msg(&bot, msg, &state);
                        } else {
                            let rand_size = rng().random_range(1..=45);

                            let formatted_pp = format!("{:=<1$}", "", rand_size);

                            let mut msg = format!("{}: 8{}D", &name, formatted_pp);

                            if rand_size > 40 && rand_size < 46 {
                                msg += " hude dih!!";
                            }

                            if rand_size > 30 && rand_size < 40 {
                                msg += " average dih";
                            }

                            if rand_size > 20 && rand_size < 30 {
                                msg += " ehhhhh.....";
                            }

                            if rand_size > 10 && rand_size < 20 {
                                msg += " small dih";
                            }

                            if rand_size > 1 && rand_size < 10 {
                                msg += " ..."
                            }

                            send_msg(&bot, msg, &state);
                        }
                    }

                    if args[0].to_lowercase().starts_with("online") {
                        if args.len() > 1 {
                            let res = commands::online::get_online_players(args[1].to_string()).await;

                            send_msg(&bot, res, &state);
                        } else {
                            let players = bot.tab_list().len();

                            let msg = format!("{} players online", players);

                            send_msg(&bot, msg, &state);
                        }
                    }

                    if args[0].to_lowercase().starts_with("y/n") || args[0].to_lowercase().starts_with("n/y") {
                        let choices_vec: Vec<String> = vec!["YES".to_string(), "NO".to_string()];

                        let random_choice = &choices_vec[rng().random_range(0..=1)];

                        let msg = format!("{}", random_choice);

                        send_msg(&bot, msg, &state);
                    }

                    if args[0].to_lowercase().starts_with("dice") || args[0].to_lowercase().starts_with("roll") {
                        let rand_int = rng().random_range(1..=6);

                        let msg = format!("{}", rand_int);

                        send_msg(&bot, msg, &state);
                    }

                    if args[0].to_lowercase().starts_with("infect") {
                        if args.len() > 1 {
                            let msg = format!("{} infected {} with autism", &name, args[1].to_string());

                            send_msg(&bot, msg, &state);
                        } else {
                            let msg = format!("{} infected himself with autism", &name);

                            send_msg(&bot, msg, &state);
                        }
                    }

                    if args[0].to_lowercase().starts_with("execute") {
                        let string_to_replace = format!("!{} ", args[0]);
                        let input_string = format!("!{}", args[0]);

                        let msg_content = content.replace(&string_to_replace, "");

                        if msg_content == input_string {
                            let msg = format!("Execution started on {}! Vote /kill yes or no to vote", &name);

                            send_msg(&bot, msg, &state);

                            return Ok(());
                        }

                        let msg = format!("Execution started on {}! Vote /kill yes or no to vote", msg_content);

                        send_msg(&bot, msg, &state);
                    }

                    if args[0].to_lowercase().starts_with("vote") {
                        if args.len() > 1 {
                            if args[1].to_string().to_lowercase() == "yes" {
                                let msg = format!("{} voted yes", &name);

                                send_msg(&bot, msg, &state);
                            } else if args[1].to_string().to_lowercase() == "no" {
                                let msg = format!("{} voted yes", &name);

                                send_msg(&bot, msg, &state);
                            } else {
                                let msg = format!("Wrong choice");

                                send_msg(&bot, msg, &state);
                            }
                        } else {
                            let msg = format!("Choose: yes/no");

                            send_msg(&bot, msg, &state);
                        }
                    }

                    if args[0].to_lowercase().starts_with("jew") {
                        let string_to_replace = format!("!{} ", args[0]);
                        let input_string = format!("!{}", args[0]);

                        let msg_content = content.replace(&string_to_replace, "");

                        if msg_content == input_string {
                            let rand_int = rng().random_range(0..=2);

                            if rand_int == 0 {
                                let msg = format!("{} is a jew!", &name);
                                
                                send_msg(&bot, msg, &state);
                            }

                            if rand_int == 1 {
                                let msg = format!("{} is not a jew!", &name);
                                
                                send_msg(&bot, msg, &state);
                            }

                            if rand_int == 2 {
                                let msg = format!("{} is maybe a jew!", &name);
                                
                                send_msg(&bot, msg, &state);
                            }

                            return Ok(());
                        }

                        let rand_int = rng().random_range(0..=2);

                        if rand_int == 0 {
                            let msg = format!("{} is a jew!", args[1].to_string());
                            
                            send_msg(&bot, msg, &state);
                        }

                        if rand_int == 1 {
                            let msg = format!("{} is not a jew!", args[1].to_string());
                            
                            send_msg(&bot, msg, &state);
                        }

                        if rand_int == 2 {
                            let msg = format!("{} is maybe a jew!", args[1].to_string());
                            
                            send_msg(&bot, msg, &state);
                        }
                    }

                    if args[0].to_lowercase().starts_with("cooltext") {
                        let string_to_replace = format!("!{} ", args[0]);
                        let input_string = format!("!{}", args[0]);

                        let msg_content = content.replace(&string_to_replace, "");

                        if msg_content == input_string {
                            let msg = format!("Text required!");

                            send_msg(&bot, msg, &state);

                            return Ok(());
                        }

                        let res = turn_text_to_cool_font(msg_content);

                        send_msg(&bot, res, &state);
                    }

                    if args[0].to_lowercase().starts_with("motd") {
                        let msg = commands::motd::motd();

                        send_msg(&bot, msg, &state);
                    }

                    if args[0].to_lowercase().starts_with("summon") {
                        let string_to_replace = format!("!{} ", args[0]);
                        let input_string = format!("!{}", args[0]);

                        let msg_content = content.replace(&string_to_replace, "");

                        if msg_content == input_string {
                            let msg = format!("{} summoned nothing :/", &name);

                            send_msg(&bot, msg, &state);

                            return Ok(());
                        }

                        let msg = format!("{} summoned {}", &name, msg_content);

                        send_msg(&bot, msg, &state);
                    }

                    if args[0].to_lowercase().starts_with("setjm") || args[0].to_lowercase().starts_with("setjoinmsg") || args[0].to_lowercase().starts_with("setjoinmessage") || args[0].to_lowercase().starts_with("addjm") || args[0].to_lowercase().starts_with("addjoinmsg") || args[0].to_lowercase().starts_with("addjoinmessage") {
                        let conn_lock = state.pg_pool.lock().clone();
                        let conn = conn_lock.as_ref();

                        match conn {
                            Some(pg_conn) => {
                                if args.len() > 1 {
                                    let string_to_replace = format!("!{} {} ", args[0], args[1]);
                                    let input_string = format!("!{} {}", args[0], args[1]);

                                    let msg_content = content.replace(&string_to_replace, "");

                                    if msg_content == input_string {
                                        let msg = format!("Message required!");

                                        send_msg(&bot, msg, &state);

                                        return Ok(());
                                    }

                                    let res = commands::setjm::set_jm(&name, args[1].to_string(), msg_content, SERVER.to_string(), pg_conn).await;

                                    send_msg(&bot, res, &state);
                                } else {
                                    send_msg(&bot, "Player required!".to_string(), &state);
                                }
                            },
                            None => {
                                send_msg(&bot, "Failed to connect to database".to_string(), &state);
                            }
                        }
                    }

                    if args[0].to_lowercase().starts_with("remjm") || args[0].to_lowercase().starts_with("remjoinmsg") || args[0].to_lowercase().starts_with("remjoinmessage") || args[0].to_lowercase().starts_with("rmjm") || args[0].to_lowercase().starts_with("rmjoinmsg") || args[0].to_lowercase().starts_with("rmjoinmessage") || args[0].to_lowercase().starts_with("removejm") || args[0].to_lowercase().starts_with("removejoinmsg") || args[0].to_lowercase().starts_with("removejoinmessage") {
                        let conn_lock = state.pg_pool.lock().clone();
                        let conn = conn_lock.as_ref();

                        match conn {
                            Some(pg_conn) => {
                                if args.len() > 1 {
                                    let res = commands::remjm::remove_jm(&name, SERVER.to_string(), pg_conn).await;

                                    send_msg(&bot, res, &state);
                                } else {
                                    let res = commands::remjm::remove_jm(&name, SERVER.to_string(), pg_conn).await;

                                    send_msg(&bot, res, &state);
                                }
                            },
                            None => {
                                send_msg(&bot, "Failed to connect to database".to_string(), &state);
                            }
                        }
                    }

                    if args[0].to_lowercase().starts_with("setlm") || args[0].to_lowercase().starts_with("setleavemsg") || args[0].to_lowercase().starts_with("setleavemessage") || args[0].to_lowercase().starts_with("addlm") || args[0].to_lowercase().starts_with("addleavemsg") || args[0].to_lowercase().starts_with("addleavemessage") {
                        let conn_lock = state.pg_pool.lock().clone();
                        let conn = conn_lock.as_ref();

                        match conn {
                            Some(pg_conn) => {
                                if args.len() > 1 {
                                    let string_to_replace = format!("!{} {} ", args[0], args[1]);
                                    let input_string = format!("!{} {}", args[0], args[1]);

                                    let msg_content = content.replace(&string_to_replace, "");

                                    if msg_content == input_string {
                                        let msg = format!("Message required!");

                                        send_msg(&bot, msg, &state);

                                        return Ok(());
                                    }

                                    let res = commands::setlm::set_lm(&name, args[1].to_string(), msg_content, SERVER.to_string(), pg_conn).await;

                                    send_msg(&bot, res, &state);
                                } else {
                                    send_msg(&bot, "Player required!".to_string(), &state);
                                }
                            },
                            None => {
                                send_msg(&bot, "Failed to connect to database".to_string(), &state);
                            }
                        }
                    }

                    if args[0].to_lowercase().starts_with("remlm") || args[0].to_lowercase().starts_with("remleavemsg") || args[0].to_lowercase().starts_with("remleavemessage") || args[0].to_lowercase().starts_with("rmlm") || args[0].to_lowercase().starts_with("rmleavemsg") || args[0].to_lowercase().starts_with("rmleavemessage") || args[0].to_lowercase().starts_with("removelm") || args[0].to_lowercase().starts_with("removeleavemsg") || args[0].to_lowercase().starts_with("removeleavemessage") {
                        let conn_lock = state.pg_pool.lock().clone();
                        let conn = conn_lock.as_ref();

                        match conn {
                            Some(pg_conn) => {
                                if args.len() > 1 {
                                    let res = commands::remlm::remove_lm(&name, SERVER.to_string(), pg_conn).await;

                                    send_msg(&bot, res, &state);
                                } else {
                                    let res = commands::remlm::remove_lm(&name, SERVER.to_string(), pg_conn).await;

                                    send_msg(&bot, res, &state);
                                }
                            },
                            None => {
                                send_msg(&bot, "Failed to connect to database".to_string(), &state);
                            }
                        }
                    }

                    if args[0].to_lowercase().starts_with("addfaq") {
                        let conn_lock = state.pg_pool.lock().clone();
                        let conn = conn_lock.as_ref();

                        match conn {
                            Some(pg_conn) => {
                                let string_to_replace = format!("!{} ", args[0]);
                                let input_string = format!("!{}", args[0]);

                                let msg_content = content.replace(&string_to_replace, "");

                                if msg_content == input_string {
                                    let msg = format!("Text required!");

                                    send_msg(&bot, msg, &state);

                                    return Ok(());
                                }

                                let res = commands::addfaq::add_faq(&name, msg_content, SERVER.to_string(), pg_conn).await;

                                send_msg(&bot, res, &state);
                            },
                            None => {
                                send_msg(&bot, "Failed to connect to database".to_string(), &state);
                            }
                        }
                    }

                    if args[0].to_lowercase().starts_with("faq") {
                        let conn_lock = state.pg_pool.lock().clone();
                        let conn = conn_lock.as_ref();

                        match conn {
                            Some(pg_conn) => {
                                if args.len() > 1 {
                                    let num = args[1].parse::<i64>();

                                    match num {
                                        Ok(v) => {
                                            let res = commands::faq::get_faq(v, SERVER.to_string(), pg_conn).await;

                                            send_msg(&bot, res, &state);
                                        },
                                        Err(_e) => {
                                            send_msg(&bot, "Faq not found".to_string(), &state);
                                        }
                                    }
                                } else {
                                    let res = commands::faq::get_random_faq(SERVER.to_string(), pg_conn).await;

                                    send_msg(&bot, res, &state);
                                }
                            },
                            None => {
                                send_msg(&bot, "Failed to connect to database".to_string(), &state);
                            }
                        }
                    }

                    if args[0].to_lowercase().starts_with("q") {
                        if args[0].to_lowercase().starts_with("quote") {
                            let conn_lock = state.pg_pool.lock().clone();
                            let conn = conn_lock.as_ref();

                            match conn {
                                Some(pg_conn) => {
                                    if args.len() > 1 {
                                        let res = commands::quote::get_quote(&args[1].to_string(), SERVER.to_string(), pg_conn).await;

                                        send_msg(&bot, res, &state);
                                    } else {
                                        let res = commands::quote::get_quote(&name, SERVER.to_string(), pg_conn).await;

                                        send_msg(&bot, res, &state);
                                    }
                                },
                                None => {
                                    send_msg(&bot, "Failed to connect to database".to_string(), &state);
                                }
                            }
                        } else {
                            let res = commands::queue::get_queue().await;

                            send_msg(&bot, res, &state);
                        }
                    }

                    if args[0].to_lowercase().starts_with("report") {
                        if args.len() > 1 {
                            let string_to_replace = format!("!{} {} ", args[0], args[1]);
                            let input_string = format!("!{} {}", args[0], args[1]);

                            let msg_content = content.replace(&string_to_replace, "");

                            if msg_content == input_string {
                                let msg = format!("{} reported {}", &name, args[1].to_string());

                                send_msg(&bot, msg, &state);

                                return Ok(());
                            }

                            let msg = format!("{} reported {} for {}", &name, args[1].to_string(), msg_content);

                            send_msg(&bot, msg, &state);
                        } else {
                            send_msg(&bot, "Player required!".to_string(), &state);
                        }
                    }

                    if args[0].to_lowercase().starts_with("task") {
                        let msg = commands::task::task();

                        send_msg(&bot, msg, &state);
                    }

                    if args[0].to_lowercase().starts_with("bible") {
                        let msg = commands::bible::random_verse();

                        send_msg(&bot, msg, &state);
                    }
                    
                    if args[0].to_lowercase().starts_with("uuid") {
                        if args.len() > 1 {
                            let uuid = bot.player_uuid_by_username(args[1]);

                            match uuid {
                                Some(v) => {
                                    let msg = format!("{}: {}", args[1], v);

                                    send_msg(&bot, msg, &state);
                                },
                                None => {
                                    send_msg(&bot, "Failed to get uuid".to_string(), &state);
                                }
                            }
                        } else {
                            let uuid = bot.player_uuid_by_username(&name);

                            match uuid {
                                Some(v) => {
                                    let msg = format!("{}: {}", &name, v);

                                    send_msg(&bot, msg, &state);
                                },
                                None => {
                                    send_msg(&bot, "Failed to get uuid".to_string(), &state);
                                }
                            }
                        }
                    }

                    if args[0].to_lowercase().starts_with("ip") || args[0].to_lowercase().starts_with("dox") {
                        if args.len() > 1 {
                            let rand_ip_1 = rng().random_range(1..=255);
                            let rand_ip_2 = rng().random_range(1..=255);
                            let rand_ip_3 = rng().random_range(1..=255);
                            let rand_ip_4 = rng().random_range(1..=255);

                            let ip = format!("{}.{}.{}.{}", rand_ip_1, rand_ip_2, rand_ip_3, rand_ip_4);

                            let msg = format!("{}: {}", args[1], ip);

                            send_msg(&bot, msg, &state);
                        } else {
                            let rand_ip_1 = rng().random_range(1..=255);
                            let rand_ip_2 = rng().random_range(1..=255);
                            let rand_ip_3 = rng().random_range(1..=255);
                            let rand_ip_4 = rng().random_range(1..=255);

                            let ip = format!("{}.{}.{}.{}", rand_ip_1, rand_ip_2, rand_ip_3, rand_ip_4);

                            let msg = format!("{}: {}", &name, ip);

                            send_msg(&bot, msg, &state);
                        }
                    }

                    if args[0].to_lowercase().starts_with("close") {
                        let owner = state.config.lock().clone().owner;

                        if name.to_lowercase() == owner.to_lowercase() {
                            bot.disconnect();

                            sleep(Duration::from_secs(1)).await;

                            process::exit(0);
                        }
                    }
                },
                None => {},
            }
        },
        Event::Login => {
            println!("Logged in");

            let mut lock = state.ticks_count.lock();

            *lock = 0;

            let mut lock = state.bot_join_time.lock();

            *lock = Some(SystemTime::now());
        },
        Event::AddPlayer(e) => {
            println!("{:?} joined the game", e.profile.name);

            let conn_lock = state.pg_pool.lock().clone();
            let conn = conn_lock.as_ref();

            match conn {
                Some(pg_conn) => {
                    let timestamp: DateTime<Utc> = Utc::now();

                    let joined_before = db_insert_joindate(&e.profile.name, timestamp, SERVER.to_string(), pg_conn).await;

                    if joined_before == false {
                        let send_w_msg = state.config.lock().send_welcome_message;

                        if send_w_msg == true {
                            let time_since_last_wm_lock = state.time_since_last_wm.lock().clone();

                            match time_since_last_wm_lock {
                                Some(v) => {
                                    let elapsed = v.elapsed();

                                    match elapsed {
                                        Ok(val) => {
                                            let seconds = val.as_secs();

                                            if seconds > 7 {
                                                let messages = state.config.lock().welcome_messages.clone();
                                                let rand_message = &messages[rng().random_range(0..=messages.len() - 1)];

                                                let msg = rand_message.replace("$NAME", &e.profile.name);

                                                send_msg(&bot, msg, &state);
                                            }
                                        },
                                        Err(e) => {
                                            println!("{}", e);
                                        }
                                    }
                                },
                                None => {
                                    let messages = state.config.lock().welcome_messages.clone();
                                    let rand_message = &messages[rng().random_range(0..=messages.len() - 1)];

                                    let msg = rand_message.replace("$NAME", &e.profile.name);

                                    send_msg(&bot, msg, &state);

                                    *state.time_since_last_wm.lock() = Some(SystemTime::now());
                                }
                            }
                        }
                    }

                    db_update_seen(&e.profile.name, timestamp, SERVER.to_string(), pg_conn).await;

                    db_update_joins(&e.profile.name, SERVER.to_string(), pg_conn).await;

                    state.player_join_time_hashmap.lock().insert(e.profile.name.clone(), SystemTime::now());

                    let is_chat_bot = state.config.lock().clone().chat_bot;

                    if is_chat_bot == true {
                        let use_join_regex = state.config.lock().clone().use_join_regex;

                        if use_join_regex == false {
                            let time = state.time_since_last_jm.lock().clone();

                            match time {
                                Some(v) => {
                                    let elapsed = v.elapsed();

                                    match elapsed {
                                        Ok(val) => {
                                            let seconds = val.as_secs();

                                            if seconds > 7 {
                                                let res = db_get_joinmessage(e.profile.name, SERVER.to_string(), pg_conn).await;

                                                match res {
                                                    Ok(v) => {
                                                        let msg = format!("{}: {}", v.player_name, v.message);
                                                        
                                                        send_msg(&bot, msg, &state);

                                                        *state.time_since_last_jm.lock() = Some(SystemTime::now());
                                                    },
                                                    Err(_e) => {}
                                                }
                                            }
                                        },
                                        Err(_e) => {}
                                    }
                                },
                                None => {
                                    let res = db_get_joinmessage(e.profile.name, SERVER.to_string(), pg_conn).await;

                                    match res {
                                        Ok(v) => {
                                            let msg = format!("{}: {}", v.player_name, v.message);
                                            
                                            send_msg(&bot, msg, &state);

                                            *state.time_since_last_jm.lock() = Some(SystemTime::now());
                                        },
                                        Err(_e) => {}
                                    }
                                }
                            }
                        }
                    }
                },
                None => {}
            }
        },
        Event::RemovePlayer(e) => {
            println!("{:?} left the game", e.profile.name);

            let conn_lock = state.pg_pool.lock().clone();
            let conn = conn_lock.as_ref();

            match conn {
                Some(pg_conn) => {
                    let timestamp: DateTime<Utc> = Utc::now();

                    db_update_seen(&e.profile.name, timestamp, SERVER.to_string(), pg_conn).await;

                    db_update_leaves(&e.profile.name, SERVER.to_string(), pg_conn).await;

                    let lock = state.player_join_time_hashmap.lock().clone();
                    let player_option = lock.get(&e.profile.name);

                    match player_option {
                        Some(v) => {
                            let elapsed = v.elapsed().unwrap();

                            let secs = elapsed.as_secs() as i64;

                            db_update_playtime(e.profile.name.clone(), secs, SERVER.to_string(), pg_conn).await;

                            state.player_join_time_hashmap.lock().remove(&e.profile.name);
                        },
                        None => {}
                    }

                    let is_chat_bot = state.config.lock().clone().chat_bot;

                    if is_chat_bot == true {
                        let use_leave_regex = state.config.lock().clone().use_leave_regex;

                        if use_leave_regex == false {
                            let time = state.time_since_last_lm.lock().clone();

                            match time {
                                Some(v) => {
                                    let elapsed = v.elapsed();

                                    match elapsed {
                                        Ok(val) => {
                                            let seconds = val.as_secs();

                                            if seconds > 7 {
                                                let res = db_get_leavemessage(e.profile.name, SERVER.to_string(), pg_conn).await;

                                                match res {
                                                    Ok(v) => {
                                                        let msg = format!("{}: {}", v.player_name, v.message);

                                                        send_msg(&bot, msg, &state);

                                                        *state.time_since_last_lm.lock() = Some(SystemTime::now());
                                                    },
                                                    Err(_e) => {}
                                                }
                                            }
                                        },
                                        Err(_e) => {}
                                    }
                                },
                                None => {
                                    let res = db_get_leavemessage(e.profile.name, SERVER.to_string(), pg_conn).await;

                                    match res {
                                        Ok(v) => {
                                            let msg = format!("{}: {}", v.player_name, v.message);

                                            send_msg(&bot, msg, &state);

                                            *state.time_since_last_lm.lock() = Some(SystemTime::now());
                                        },
                                        Err(_e) => {}
                                    }
                                }
                            }
                        }
                    }
                },
                None => {}
            }
        },
        Event::Disconnect(e) => {
            println!("Bot disconnected!");

            match e {
                Some(v) => {
                    println!("Disconnected with text: {:?}", v);
                },
                None => {
                    println!("Disconnected without text");
                }
            }

            let conn_lock = state.pg_pool.lock().clone();
            let conn = conn_lock.as_ref();

            match conn {
                Some(pg_conn) => {
                    let lock = state.player_join_time_hashmap.lock().clone();

                    let mut player_data_vec: Vec<String> = Vec::new();

                    for item in lock {
                        let elapsed = item.1.elapsed();

                        match elapsed {
                            Ok(v) => {
                                let secs = v.as_secs() as i64;

                                let formatted_string = format!("('{}', '{}', '{}')", item.0, secs, SERVER.to_string());

                                player_data_vec.push(formatted_string);
                            },
                            Err(_e) => {}
                        }
                    }

                    state.player_join_time_hashmap.lock().clear();

                    db_batch_update_playtime(player_data_vec.join(", "), pg_conn).await;
                },
                None => {}
            }
        },
        Event::Death(_e) => println!("Died at {}", bot.position()),
        Event::Spawn => println!("Spawned at {}", bot.position()),
        Event::UpdatePlayer(_e) => {
            let is_loggged_in = bot.logged_in();

            if is_loggged_in == true {
                let time = state.time_sice_last_action.lock().clone();

                match time {
                    Some(v) => {
                        let elapsed = v.elapsed();

                        match elapsed {
                            Ok(val) => {
                                let seconds = val.as_secs();

                                if seconds > 20 {
                                    let p = bot.position();

                                    let rand_x = rng().random_range(-2..=2) as f64;
                                    let rand_z = rng().random_range(-2..=2) as f64;

                                    let pos = Vec3 { x: p.x + rand_x, y: p.y, z: p.z + rand_z };

                                    bot.look_at(pos);
                                    
                                    bot.jump();
                                }
                            },
                            Err(e) => {
                                println!("{}", e);

                                println!("Failed to get duration");
                            }
                        }
                    },
                    None => {
                        *state.time_sice_last_action.lock() = Some(SystemTime::now());
                    }
                }
            }
        },
        Event::Tick => {
            let mut lock = state.ticks_count.lock();

            *lock += 1;
        }
        _ => {}
    }

    Ok(())
}
