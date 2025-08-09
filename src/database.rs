use std::error::Error;
use sqlx::postgres::PgPoolOptions;
use sqlx::types::chrono::DateTime;
use sqlx::types::chrono::Utc;
use sqlx::Pool;
use sqlx::Postgres;
use sqlx::Row;

// Example:
//  postgres://user:password@address:port/database
//  postgres://postgres:YouShallNotPass@localhost:5432/my_database
const DB_URL: &str = "";

pub async fn connect_db(max_connections: u32) -> Option<Pool<Postgres>> {
    let pool = PgPoolOptions::new()
        .max_connections(max_connections)
        .connect(DB_URL)
        .await;

    match pool {
        Ok(v) => {
            Some(v)
        },
        Err(e) => {
            println!("{}", e);

            None
        }
    }
}

pub async fn db_get_total_players(server: String, conn: &Pool<Postgres>) -> Result<i64, Box<dyn Error>> {
    let res = sqlx::query("SELECT COUNT(player_name) FROM seen WHERE server = $1")
        .bind(server)
        .fetch_one(conn)
        .await?;

    let count: i64 = res.get("count");

    if count > 0 {
        return Ok(count)
    } else {
        return Err("No players found".into())
    }
}

#[derive(Debug)]
pub struct TopHardNwordArrPlayer {
    pub player_name: String,
    pub hard: i32
}

#[derive(Debug)]
pub struct TopHardNwordArr {
    pub arr: Vec<TopHardNwordArrPlayer>
}

pub async fn db_get_top_nwords_hard(server: String, conn: &Pool<Postgres>) -> Result<TopHardNwordArr, Box<dyn Error>> {
    let res = sqlx::query("SELECT player_name, hard FROM nwords WHERE server = $1 ORDER BY hard DESC LIMIT 5")
        .bind(server)
        .fetch_all(conn)
        .await?;

    if res.len() > 0 {
        let mut arr: Vec<TopHardNwordArrPlayer> = Vec::new();

        for item in res {
            let arr_player = TopHardNwordArrPlayer { player_name: item.get("player_name"), hard: item.get("hard") };

            arr.push(arr_player);
        }

        let final_arr = TopHardNwordArr { arr: arr };

        return Ok(final_arr)
    } else {
        return Err("Empty".into())
    }
}

#[derive(Debug)]
pub struct TopSoftNwordArrPlayer {
    pub player_name: String,
    pub soft: i32
}

#[derive(Debug)]
pub struct TopSoftNwordArr {
    pub arr: Vec<TopSoftNwordArrPlayer>
}

pub async fn db_get_top_nwords_soft(server: String, conn: &Pool<Postgres>) -> Result<TopSoftNwordArr, Box<dyn Error>> {
    let res = sqlx::query("SELECT player_name, soft FROM nwords WHERE server = $1 ORDER BY soft DESC LIMIT 5")
        .bind(server)
        .fetch_all(conn)
        .await?;

    if res.len() > 0 {
        let mut arr: Vec<TopSoftNwordArrPlayer> = Vec::new();

        for item in res {
            let arr_player = TopSoftNwordArrPlayer { player_name: item.get("player_name"), soft: item.get("soft") };

            arr.push(arr_player);
        }

        let final_arr = TopSoftNwordArr { arr: arr };

        return Ok(final_arr)
    } else {
        return Err("Empty".into())
    }
}

#[derive(Debug)]
pub struct TopKillsArrPlayer {
    pub player_name: String,
    pub kills: i32
}

#[derive(Debug)]
pub struct TopKillsArr {
    pub arr: Vec<TopKillsArrPlayer>
}

pub async fn db_get_top_kills(server: String, conn: &Pool<Postgres>) -> Result<TopKillsArr, Box<dyn Error>> {
    let res = sqlx::query("SELECT player_name, kills FROM kds WHERE server = $1 ORDER BY kills DESC LIMIT 5")
        .bind(server)
        .fetch_all(conn)
        .await?;

    if res.len() > 0 {
        let mut arr: Vec<TopKillsArrPlayer> = Vec::new();

        for item in res {
            let arr_player = TopKillsArrPlayer { player_name: item.get("player_name"), kills: item.get("kills") };

            arr.push(arr_player);
        }

        let final_arr = TopKillsArr { arr: arr };

        return Ok(final_arr)
    } else {
        return Err("Empty".into())
    }
}

#[derive(Debug)]
pub struct TopDeathsArrPlayer {
    pub player_name: String,
    pub deaths: i32
}

#[derive(Debug)]
pub struct TopDeathsArr {
    pub arr: Vec<TopDeathsArrPlayer>
}

pub async fn db_get_top_deaths(server: String, conn: &Pool<Postgres>) -> Result<TopDeathsArr, Box<dyn Error>> {
    let res = sqlx::query("SELECT player_name, deaths FROM kds WHERE server = $1 ORDER BY deaths DESC LIMIT 5")
        .bind(server)
        .fetch_all(conn)
        .await?;

    if res.len() > 0 {
        let mut arr: Vec<TopDeathsArrPlayer> = Vec::new();

        for item in res {
            let arr_player = TopDeathsArrPlayer { player_name: item.get("player_name"), deaths: item.get("deaths") };

            arr.push(arr_player);
        }

        let final_arr = TopDeathsArr { arr: arr };

        return Ok(final_arr)
    } else {
        return Err("Empty".into())
    }
}

#[derive(Debug)]
pub struct TopPlaytimeArrPlayer {
    pub player_name: String,
    pub seconds: i64
}

#[derive(Debug)]
pub struct TopPlaytimeArr {
    pub arr: Vec<TopPlaytimeArrPlayer>
}

pub async fn db_get_top_playtime(server: String, conn: &Pool<Postgres>) -> Result<TopPlaytimeArr, Box<dyn Error>> {
    let res = sqlx::query("SELECT player_name, seconds FROM playtime WHERE server = $1 ORDER BY seconds DESC LIMIT 5")
        .bind(server)
        .fetch_all(conn)
        .await?;

    if res.len() > 0 {
        let mut arr: Vec<TopPlaytimeArrPlayer> = Vec::new();

        for item in res {
            let arr_player = TopPlaytimeArrPlayer { player_name: item.get("player_name"), seconds: item.get("seconds") };

            arr.push(arr_player);
        }

        let final_arr = TopPlaytimeArr { arr: arr };

        return Ok(final_arr)
    } else {
        return Err("Empty".into())
    }
}

#[derive(Debug)]
pub struct TopMessagesArrPlayer {
    pub player_name: String,
    pub count: i32
}

#[derive(Debug)]
pub struct TopMessagesArr {
    pub arr: Vec<TopMessagesArrPlayer>
}

pub async fn db_get_top_messages(server: String, conn: &Pool<Postgres>) -> Result<TopMessagesArr, Box<dyn Error>> {
    let res = sqlx::query("SELECT player_name, count FROM chatcount WHERE server = $1 ORDER BY count DESC LIMIT 5")
        .bind(server)
        .fetch_all(conn)
        .await?;

    if res.len() > 0 {
        let mut arr: Vec<TopMessagesArrPlayer> = Vec::new();

        for item in res {
            let arr_player = TopMessagesArrPlayer { player_name: item.get("player_name"), count: item.get("count") };

            arr.push(arr_player);
        }

        let final_arr = TopMessagesArr { arr: arr };

        return Ok(final_arr)
    } else {
        return Err("Empty".into())
    }
}

#[derive(Debug)]
pub struct TopJoinsArrPlayer {
    pub player_name: String,
    pub joins: i32
}

#[derive(Debug)]
pub struct TopJoinsArr {
    pub arr: Vec<TopJoinsArrPlayer>
}

pub async fn db_get_top_joins(server: String, conn: &Pool<Postgres>) -> Result<TopJoinsArr, Box<dyn Error>> {
    let res = sqlx::query("SELECT player_name, joins FROM joinsleaves WHERE server = $1 ORDER BY joins DESC LIMIT 5")
        .bind(server)
        .fetch_all(conn)
        .await?;

    if res.len() > 0 {
        let mut arr: Vec<TopJoinsArrPlayer> = Vec::new();

        for item in res {
            let arr_player = TopJoinsArrPlayer { player_name: item.get("player_name"), joins: item.get("joins") };

            arr.push(arr_player);
        }

        let final_arr = TopJoinsArr { arr: arr };

        return Ok(final_arr)
    } else {
        return Err("Empty".into())
    }
}

#[derive(Debug)]
pub struct TopLeavesArrPlayer {
    pub player_name: String,
    pub leaves: i32
}

#[derive(Debug)]
pub struct TopLeavesArr {
    pub arr: Vec<TopLeavesArrPlayer>
}

pub async fn db_get_top_leaves(server: String, conn: &Pool<Postgres>) -> Result<TopLeavesArr, Box<dyn Error>> {
    let res = sqlx::query("SELECT player_name, leaves FROM joinsleaves WHERE server = $1 ORDER BY leaves DESC LIMIT 5")
        .bind(server)
        .fetch_all(conn)
        .await?;

    if res.len() > 0 {
        let mut arr: Vec<TopLeavesArrPlayer> = Vec::new();

        for item in res {
            let arr_player = TopLeavesArrPlayer { player_name: item.get("player_name"), leaves: item.get("leaves") };

            arr.push(arr_player);
        }

        let final_arr = TopLeavesArr { arr: arr };

        return Ok(final_arr)
    } else {
        return Err("Empty".into())
    }
}

#[derive(Debug)]
pub struct JoinsPlayer {
    pub player_name: String,
    pub joins: i32
}

pub async fn db_get_joins(player: String, server: String, conn: &Pool<Postgres>) -> Result<JoinsPlayer, Box<dyn Error>> {
    let res = sqlx::query("SELECT player_name, joins FROM joinsleaves WHERE server = $1 AND LOWER(player_name) = LOWER($2)")
        .bind(server)
        .bind(player)
        .fetch_one(conn)
        .await?;

    if res.len() > 0 {
        let player = JoinsPlayer { player_name: res.get("player_name"), joins: res.get("joins") };

        return Ok(player)
    } else {
        return Err("Player not found".into())
    }
}

pub async fn db_update_joins(player: &String, server: String, conn: &Pool<Postgres>) {
    let res = sqlx::query("INSERT INTO joinsleaves (player_name, joins, leaves, server) VALUES ($1, $2, $3, $4) ON CONFLICT (player_name, server) DO UPDATE SET joins = joinsleaves.joins + EXCLUDED.joins")
        .bind(player)
        .bind(1)
        .bind(0)
        .bind(server)
        .execute(conn)
        .await;

    match res {
        Ok(_) => {},
        Err(e) => {
            println!("[update joins function] {}", e);
        }
    }
}

#[derive(Debug)]
pub struct LeavesPlayer {
    pub player_name: String,
    pub leaves: i32
}

pub async fn db_get_leaves(player: String, server: String, conn: &Pool<Postgres>) -> Result<LeavesPlayer, Box<dyn Error>> {
    let res = sqlx::query("SELECT player_name, leaves FROM joinsleaves WHERE server = $1 AND LOWER(player_name) = LOWER($2)")
        .bind(server)
        .bind(player)
        .fetch_one(conn)
        .await?;

    if res.len() > 0 {
        let player = LeavesPlayer { player_name: res.get("player_name"), leaves: res.get("leaves") };

        return Ok(player)
    } else {
        return Err("Player not found".into())
    }
}

pub async fn db_update_leaves(player: &String, server: String, conn: &Pool<Postgres>) {
    let res = sqlx::query("INSERT INTO joinsleaves (player_name, joins, leaves, server) VALUES ($1, $2, $3, $4) ON CONFLICT (player_name, server) DO UPDATE SET leaves = joinsleaves.leaves + EXCLUDED.leaves")
        .bind(player)
        .bind(0)
        .bind(1)
        .bind(server)
        .execute(conn)
        .await;

    match res {
        Ok(_) => {},
        Err(e) => {
            println!("[Update leaves function] {}", e);
        }
    }
}

#[derive(Debug)]
pub struct MessagesPlayer {
    pub player_name: String,
    pub count: i32
}

pub async fn db_get_messages(player: String, server: String, conn: &Pool<Postgres>) -> Result<MessagesPlayer, Box<dyn Error>> {
    let res = sqlx::query("SELECT player_name, count FROM chatcount WHERE server = $1 AND LOWER(player_name) = LOWER($2)")
        .bind(server)
        .bind(player)
        .fetch_one(conn)
        .await?;

    if res.len() > 0 {
        let player = MessagesPlayer { player_name: res.get("player_name"), count: res.get("count") };

        return Ok(player)
    } else {
        return Err("Player not found".into())
    }
}

#[derive(Debug)]
pub struct FirstWordsPlayer {
    pub player_name: String,
    pub message: String,
    pub timestamp: DateTime<Utc>
}

pub async fn db_get_firstwords(player: String, server: String, conn: &Pool<Postgres>) -> Result<FirstWordsPlayer, Box<dyn Error>> {
    let res = sqlx::query("SELECT player_name, message, timestamp FROM chatlogs WHERE server = $1 AND LOWER(player_name) = LOWER($2) ORDER BY timestamp ASC")
        .bind(server)
        .bind(player)
        .fetch_one(conn)
        .await?;

    if res.len() > 0 {
        let player = FirstWordsPlayer { player_name: res.get("player_name"), message: res.get("message"), timestamp: res.get("timestamp") };

        return Ok(player)
    } else {
        return Err("Player not found".into())
    }
}

#[derive(Debug)]
pub struct LastWordsPlayer {
    pub player_name: String,
    pub message: String,
    pub timestamp: DateTime<Utc>
}

pub async fn db_get_lastwords(player: String, server: String, conn: &Pool<Postgres>) -> Result<LastWordsPlayer, Box<dyn Error>> {
    let res = sqlx::query("SELECT player_name, message, timestamp FROM chatlogs WHERE server = $1 AND LOWER(player_name) = LOWER($2) ORDER BY timestamp DESC")
        .bind(server)
        .bind(player)
        .fetch_one(conn)
        .await?;

    if res.len() > 0 {
        let player = LastWordsPlayer { player_name: res.get("player_name"), message: res.get("message"), timestamp: res.get("timestamp") };

        return Ok(player)
    } else {
        return Err("Player not found".into())
    }
}

#[derive(Debug)]
pub struct LastKillPlayer {
    pub player_name: String,
    pub last_kill_message: String,
    pub last_kill_timestamp: DateTime<Utc>
}

pub async fn db_get_lastkill(player: String, server: String, conn: &Pool<Postgres>) -> Result<LastKillPlayer, Box<dyn Error>> {
    let res = sqlx::query("SELECT player_name, last_kill_message, last_kill_timestamp FROM lastkills WHERE server = $1 AND LOWER(player_name) = LOWER($2)")
        .bind(server)
        .bind(player)
        .fetch_one(conn)
        .await?;

    let last_kill_message: Result<String, sqlx::Error> = res.try_get("last_kill_message");
    let last_kill_timestamp: Result<DateTime<Utc>, sqlx::Error> = res.try_get("last_kill_timestamp");

    match last_kill_message {
        Ok(_v) => {},
        Err(e) => {
            println!("{}", e);

            return Err("Failed to retrieve data, possibly NULL".into());
        }
    }

    match last_kill_timestamp {
        Ok(_v) => {},
        Err(e) => {
            println!("{}", e);

            return Err("Failed to retrieve data, possibly NULL".into());
        }
    }

    if res.len() > 0 {
        let player = LastKillPlayer { player_name: res.get("player_name"), last_kill_message: res.get("last_kill_message"), last_kill_timestamp: res.get("last_kill_timestamp") };

        return Ok(player)
    } else {
        return Err("Player not found".into())
    }
}

#[derive(Debug)]
pub struct FirstKillPlayer {
    pub player_name: String,
    pub first_kill_message: String,
    pub first_kill_timestamp: DateTime<Utc>
}

pub async fn db_get_firstkill(player: String, server: String, conn: &Pool<Postgres>) -> Result<FirstKillPlayer, Box<dyn Error>> {
    let res = sqlx::query("SELECT player_name, first_kill_message, first_kill_timestamp FROM lastkills WHERE server = $1 AND LOWER(player_name) = LOWER($2)")
        .bind(server)
        .bind(player)
        .fetch_one(conn)
        .await?;

    let first_kill_message: Result<String, sqlx::Error> = res.try_get("first_kill_message");
    let first_kill_timestamp: Result<DateTime<Utc>, sqlx::Error> = res.try_get("first_kill_timestamp");

    match first_kill_message {
        Ok(_v) => {},
        Err(e) => {
            println!("{}", e);

            return Err("Failed to retrieve data, possibly NULL".into());
        }
    }

    match first_kill_timestamp {
        Ok(_v) => {},
        Err(e) => {
            println!("{}", e);

            return Err("Failed to retrieve data, possibly NULL".into());
        }
    }

    if res.len() > 0 {
        let player = FirstKillPlayer { player_name: res.get("player_name"), first_kill_message: res.get("first_kill_message"), first_kill_timestamp: res.get("first_kill_timestamp") };

        return Ok(player)
    } else {
        return Err("Player not found".into())
    }
}

#[derive(Debug)]
pub struct LastDeathPlayer {
    pub player_name: String,
    pub last_death_message: String,
    pub last_death_timestamp: DateTime<Utc>
}

pub async fn db_get_lastdeath(player: String, server: String, conn: &Pool<Postgres>) -> Result<LastDeathPlayer, Box<dyn Error>> {
    let res = sqlx::query("SELECT player_name, last_death_message, last_death_timestamp FROM lastdeaths WHERE server = $1 AND LOWER(player_name) = LOWER($2)")
        .bind(server)
        .bind(player)
        .fetch_one(conn)
        .await?;

    let last_death_message: Result<String, sqlx::Error> = res.try_get("last_death_message");
    let last_death_timestamp: Result<DateTime<Utc>, sqlx::Error> = res.try_get("last_death_timestamp");

    match last_death_message {
        Ok(_v) => {},
        Err(e) => {
            println!("{}", e);

            return Err("Failed to retrieve data, possibly NULL".into());
        }
    }

    match last_death_timestamp {
        Ok(_v) => {},
        Err(e) => {
            println!("{}", e);

            return Err("Failed to retrieve data, possibly NULL".into());
        }
    }

    if res.len() > 0 {
        let player = LastDeathPlayer { player_name: res.get("player_name"), last_death_message: res.get("last_death_message"), last_death_timestamp: res.get("last_death_timestamp") };

        return Ok(player)
    } else {
        return Err("Player not found".into())
    }
}

#[derive(Debug)]
pub struct FirstDeathPlayer {
    pub player_name: String,
    pub first_death_message: String,
    pub first_death_timestamp: DateTime<Utc>
}

pub async fn db_get_firstdeath(player: String, server: String, conn: &Pool<Postgres>) -> Result<FirstDeathPlayer, Box<dyn Error>> {
    let res = sqlx::query("SELECT player_name, first_death_message, first_death_timestamp FROM lastdeaths WHERE server = $1 AND LOWER(player_name) = LOWER($2)")
        .bind(server)
        .bind(player)
        .fetch_one(conn)
        .await?;

    let first_death_message: Result<String, sqlx::Error> = res.try_get("first_death_message");
    let first_death_timestamp: Result<DateTime<Utc>, sqlx::Error> = res.try_get("first_death_timestamp");

    match first_death_message {
        Ok(_v) => {},
        Err(e) => {
            println!("{}", e);

            return Err("Failed to retrieve data, possibly NULL".into());
        }
    }

    match first_death_timestamp {
        Ok(_v) => {},
        Err(e) => {
            println!("{}", e);

            return Err("Failed to retrieve data, possibly NULL".into());
        }
    }

    if res.len() > 0 {
        let player = FirstDeathPlayer { player_name: res.get("player_name"), first_death_message: res.get("first_death_message"), first_death_timestamp: res.get("first_death_timestamp") };

        return Ok(player)
    } else {
        return Err("Player not found".into())
    }
}

#[derive(Debug)]
pub struct SeenPlayer {
    pub player_name: String,
    pub timestamp: DateTime<Utc>,
}

pub async fn db_get_seen(player: String, server: String, conn: &Pool<Postgres>) -> Result<SeenPlayer, Box<dyn Error>> {
    let res = sqlx::query("SELECT player_name, timestamp FROM seen WHERE server = $1 AND LOWER(player_name) = LOWER($2)")
        .bind(server)
        .bind(player)
        .fetch_one(conn)
        .await?;

    if res.len() > 0 {
        let player = SeenPlayer { player_name: res.get("player_name"), timestamp: res.get("timestamp") };

        return Ok(player)
    } else {
        return Err("Player not found".into())
    }
}

pub async fn db_update_seen(player: &String, timestamp: DateTime<Utc>, server: String, conn: &Pool<Postgres>) {
    let res = sqlx::query("INSERT INTO seen (player_name, timestamp, server) VALUES ($1, $2, $3) ON CONFLICT (player_name, server) DO UPDATE SET timestamp = EXCLUDED.timestamp")
        .bind(player)
        .bind(timestamp)
        .bind(server)
        .execute(conn)
        .await;

    match res {
        Ok(_) => {},
        Err(e) => {
            println!("[Update seen function] {}", e);
        }
    }
}

#[derive(Debug)]
pub struct JoindatePlayer {
    pub player_name: String,
    pub timestamp: DateTime<Utc>,
}

pub async fn db_get_joindate(player: String, server: String, conn: &Pool<Postgres>) -> Result<JoindatePlayer, Box<dyn Error>> {
    let res = sqlx::query("SELECT player_name, timestamp FROM joindate WHERE server = $1 AND LOWER(player_name) = LOWER($2)")
        .bind(server)
        .bind(player)
        .fetch_one(conn)
        .await?;

    if res.len() > 0 {
        let player = JoindatePlayer { player_name: res.get("player_name"), timestamp: res.get("timestamp") };

        return Ok(player)
    } else {
        return Err("Player not found".into())
    }
}

pub async fn db_insert_joindate(player: &String, timestamp: DateTime<Utc>, server: String, conn: &Pool<Postgres>) -> bool {
    let res = sqlx::query("INSERT INTO joindate (player_name, timestamp, server) VALUES ($1, $2, $3) ON CONFLICT (player_name, server) DO NOTHING")
        .bind(player)
        .bind(timestamp)
        .bind(server)
        .execute(conn)
        .await;

    match res {
        Ok(v) => {
            if v.rows_affected() > 0 {
                false
            } else {
                true
            }
        },
        Err(e) => {
            println!("[Insert joindate function] {}", e);

            true
        }
    }
}

#[derive(Debug)]
pub struct KdPlayer {
    pub player_name: String,
    pub kills: i32,
    pub deaths: i32
}

pub async fn db_get_kd(player: String, server: String, conn: &Pool<Postgres>) -> Result<KdPlayer, Box<dyn Error>> {
    let res = sqlx::query("SELECT player_name, kills, deaths FROM kds WHERE server = $1 AND LOWER(player_name) = LOWER($2)")
        .bind(server)
        .bind(player)
        .fetch_one(conn)
        .await?;

    if res.len() > 0 {
        let player = KdPlayer { player_name: res.get("player_name"), kills: res.get("kills"), deaths: res.get("deaths") };

        return Ok(player)
    } else {
        return Err("Player not found".into())
    }
}

#[derive(Debug)]
pub struct PlaytimePlayer {
    pub player_name: String,
    pub seconds: i64,
}

pub async fn db_get_playtime(player: String, server: String, conn: &Pool<Postgres>) -> Result<PlaytimePlayer, Box<dyn Error>> {
    let res = sqlx::query("SELECT player_name, seconds FROM playtime WHERE server = $1 AND LOWER(player_name) = LOWER($2)")
        .bind(server)
        .bind(player)
        .fetch_one(conn)
        .await?;

    if res.len() > 0 {
        let player = PlaytimePlayer { player_name: res.get("player_name"), seconds: res.get("seconds") };

        return Ok(player)
    } else {
        return Err("Player not found".into())
    }
}

pub async fn db_update_playtime(player: String, seconds: i64, server: String, conn: &Pool<Postgres>) {
    let res = sqlx::query("INSERT INTO playtime (player_name, seconds, server) VALUES ($1, $2, $3) ON CONFLICT (player_name, server) DO UPDATE SET seconds = playtime.seconds + EXCLUDED.seconds")
        .bind(player)
        .bind(seconds)
        .bind(server)
        .execute(conn)
        .await;

    match res {
        Ok(_) => {},
        Err(e) => {
            println!("[Update playtime function] {}", e);
        }
    }
}

pub async fn db_batch_update_playtime(data: String, conn: &Pool<Postgres>) {
    let query = format!("INSERT INTO playtime (player_name, seconds, server) VALUES {} ON CONFLICT (player_name, server) DO UPDATE SET seconds = playtime.seconds + EXCLUDED.seconds", data);

    let res = sqlx::query(&query)
        .execute(conn)
        .await;

    match res {
        Ok(_) => {},
        Err(e) => {
            println!("[Batch update playtime function] {}", e);
        }
    }
}

#[derive(Debug)]
pub struct NwordsPlayer {
    pub player_name: String,
    pub hard: i32,
    pub soft: i32
}

pub async fn db_get_nwords(player: String, server: String, conn: &Pool<Postgres>) -> Result<NwordsPlayer, Box<dyn Error>> {
    let res = sqlx::query("SELECT player_name, hard, soft FROM nwords WHERE server = $1 AND LOWER(player_name) = LOWER($2)")
        .bind(server)
        .bind(player)
        .fetch_one(conn)
        .await?;

    if res.len() > 0 {
        let player = NwordsPlayer { player_name: res.get("player_name"), hard: res.get("hard"), soft: res.get("soft") };

        return Ok(player)
    } else {
        return Err("Player not found".into())
    }
}

#[derive(Debug)]
pub struct SavedMessagePlayer {
    pub player_name: String,
    pub message: String
}

pub async fn db_get_savedmsg(player: String, server: String, conn: &Pool<Postgres>) -> Result<SavedMessagePlayer, Box<dyn Error>> {
    let res = sqlx::query("SELECT player_name, message FROM messages WHERE server = $1 AND LOWER(player_name) = LOWER($2) ORDER BY timestamp DESC LIMIT 1")
        .bind(server)
        .bind(player)
        .fetch_one(conn)
        .await?;

    if res.len() > 0 {
        let player = SavedMessagePlayer { player_name: res.get("player_name"), message: res.get("message") };

        return Ok(player)
    } else {
        return Err("Player not found".into())
    }
}

pub async fn db_savemsg(player: String, message: String, timestamp: DateTime<Utc>, server: String, conn: &Pool<Postgres>) -> Result<String, Box<dyn Error>> {
    let res = sqlx::query("INSERT INTO messages (player_name, message, timestamp, server) VALUES ($1, $2, $3, $4) ON CONFLICT (player_name, server) DO UPDATE SET message = EXCLUDED.message, timestamp = EXCLUDED.timestamp")
        .bind(player)
        .bind(message)
        .bind(timestamp)
        .bind(server)
        .execute(conn)
        .await;

    match res {
        Ok(_) => {
            return Ok("Message added".to_string())
        },
        Err(e) => {
            println!("{}", e);

            return Err("Failed to add message".into())
        }
    }
}

pub async fn db_insertiam(player: String, message: String, timestamp: DateTime<Utc>, server: String, conn: &Pool<Postgres>) -> Result<String, Box<dyn Error>> {
    let res = sqlx::query("INSERT INTO aboutuser (player_name, message, timestamp, server) VALUES ($1, $2, $3, $4) ON CONFLICT (player_name, server) DO UPDATE SET message = EXCLUDED.message, timestamp = EXCLUDED.timestamp")
        .bind(player)
        .bind(message)
        .bind(timestamp)
        .bind(server)
        .execute(conn)
        .await;

    match res {
        Ok(_) => {
            return Ok("Information added".to_string())
        },
        Err(e) => {
            println!("{}", e);

            return Err("Failed to add information".into())
        }
    }
}

#[derive(Debug)]
pub struct WhoIsPlayer {
    pub player_name: String,
    pub message: String
}

pub async fn db_get_whois(player: String, server: String, conn: &Pool<Postgres>) -> Result<WhoIsPlayer, Box<dyn Error>> {
    let res = sqlx::query("SELECT player_name, message FROM aboutuser WHERE server = $1 AND LOWER(player_name) = LOWER($2) ORDER BY timestamp DESC LIMIT 1")
        .bind(server)
        .bind(player)
        .fetch_one(conn)
        .await?;

    if res.len() > 0 {
        let player = WhoIsPlayer { player_name: res.get("player_name"), message: res.get("message") };

        return Ok(player)
    } else {
        return Err("Player not found".into())
    }
}

pub async fn db_set_joinmessage(creator: String, player: String, message: String, timestamp: DateTime<Utc>, server: String, conn: &Pool<Postgres>) -> Result<String, Box<dyn Error>> {
    let res = sqlx::query("INSERT INTO joinmsgs (creator, player_name, message, timestamp, server) VALUES ($1, $2, $3, $4, $5) ON CONFLICT (player_name, server) DO UPDATE SET message = EXCLUDED.message, creator = EXCLUDED.creator")
        .bind(creator)
        .bind(player)
        .bind(message)
        .bind(timestamp)
        .bind(server)
        .execute(conn)
        .await;

    match res {
        Ok(_v) => {
            return Ok("Set join message".to_string());
        },
        Err(e) => {
            println!("{}", e);

            return Err("Failed to set join message".into());
        }
    }
}

pub async fn db_remove_joinmessage(player: String, server: String, conn: &Pool<Postgres>) -> Result<String, Box<dyn Error>> {
    let res = sqlx::query("DELETE FROM joinmsgs WHERE server = $1 AND LOWER(player_name) = LOWER($2)")
        .bind(player)
        .bind(server)
        .execute(conn)
        .await;

    match res {
        Ok(_v) => {
            return Ok("Removed join message".to_string());
        },
        Err(e) => {
            println!("{}", e);

            return Err("Failed to remove join message".into());
        }
    }
}

pub async fn db_set_leavemessage(creator: String, player: String, message: String, timestamp: DateTime<Utc>, server: String, conn: &Pool<Postgres>) -> Result<String, Box<dyn Error>> {
    let res = sqlx::query("INSERT INTO leavemsgs (creator, player_name, message, timestamp, server) VALUES ($1, $2, $3, $4, $5) ON CONFLICT (player_name, server) DO UPDATE SET message = $3, creator = $1")
        .bind(creator)
        .bind(player)
        .bind(message)
        .bind(timestamp)
        .bind(server)
        .execute(conn)
        .await;

    match res {
        Ok(_v) => {
            return Ok("Set leave message".to_string());
        },
        Err(e) => {
            println!("{}", e);

            return Err("Failed to set leave message".into());
        }
    }
}

pub async fn db_remove_leavemessage(player: String, server: String, conn: &Pool<Postgres>) -> Result<String, Box<dyn Error>> {
    let res = sqlx::query("DELETE FROM leavemsgs WHERE server = $1 AND LOWER(player_name) = LOWER($2)")
        .bind(player)
        .bind(server)
        .execute(conn)
        .await;

    match res {
        Ok(_v) => {
            return Ok("Removed leave message".to_string());
        },
        Err(e) => {
            println!("{}", e);

            return Err("Failed to remove leave message".into());
        }
    }
}

pub async fn db_get_allfaqs(server: String, conn: &Pool<Postgres>) -> Result<i64, String> {
    let res = sqlx::query("SELECT COUNT(message) AS count FROM faqmsgs WHERE server = $1")
        .bind(server)
        .fetch_one(conn)
        .await;

    match res {
        Ok(v) => {
            let val = v.get("count");

            return Ok(val)
        },
        Err(e) => {
            println!("{}", e);

            return Err("Failed to get faq count".into());
        }
    }
}

pub async fn db_insert_faq(entry: i64, player: String, message: String, timestamp: DateTime<Utc>, server: String, conn: &Pool<Postgres>) -> Result<String, String> {
    let res = sqlx::query("INSERT INTO faqmsgs (entrynum, player_name, message, timestamp, server) VALUES ($1, $2, $3, $4, $5)")
        .bind(entry)
        .bind(player)
        .bind(message)
        .bind(timestamp)
        .bind(server)
        .execute(conn)
        .await;

    match res {
        Ok(_v) => {
            return Ok("Faq added".to_string());
        },
        Err(e) => {
            println!("{}", e);

            return Err("Failed to add faq".into());
        }
    }
}

#[derive(Debug)]
pub struct FaqPlayer {
    pub entrynum: i32,
    pub message: String
}

pub async fn db_get_randomfaq(server: String, conn: &Pool<Postgres>) -> Result<FaqPlayer, Box<dyn Error>> {
    let res = sqlx::query("SELECT entrynum, message FROM faqmsgs WHERE server = $1 ORDER BY RANDOM() LIMIT 1")
        .bind(server)
        .fetch_one(conn)
        .await?;

    if res.len() > 0 {
        let player = FaqPlayer { entrynum: res.get("entrynum"), message: res.get("message") };

        return Ok(player)
    } else {
        return Err("No faqs found".into())
    }
}

pub async fn db_get_faq(entry: i64, server: String, conn: &Pool<Postgres>) -> Result<FaqPlayer, Box<dyn Error>> {
    let res = sqlx::query("SELECT entrynum, message FROM faqmsgs WHERE server = $1 AND entrynum = $2")
        .bind(server)
        .bind(entry)
        .fetch_one(conn)
        .await?;

    if res.len() > 0 {
        let player = FaqPlayer { entrynum: res.get("entrynum"), message: res.get("message") };

        return Ok(player)
    } else {
        return Err("No faqs found".into())
    }
}

pub async fn db_update_death(player: String, server: String, conn: &Pool<Postgres>) {
    let res = sqlx::query("INSERT INTO kds (player_name, kills, deaths, server) VALUES ($1, $2, $3, $4) ON CONFLICT (player_name, server) DO UPDATE SET deaths = kds.deaths + EXCLUDED.deaths")
        .bind(player)
        .bind(0)
        .bind(1)
        .bind(server)
        .execute(conn)
        .await;

    match res {
        Ok(_v) => {},
        Err(e) => {
            println!("[Update death function] {}", e);
        }
    }
}

pub async fn db_update_kill(player: String, server: String, conn: &Pool<Postgres>) {
    let res = sqlx::query("INSERT INTO kds (player_name, kills, deaths, server) VALUES ($1, $2, $3, $4) ON CONFLICT (player_name, server) DO UPDATE SET kills = kds.kills + EXCLUDED.kills")
        .bind(player)
        .bind(1)
        .bind(0)
        .bind(server)
        .execute(conn)
        .await;

    match res {
        Ok(_v) => {},
        Err(e) => {
            println!("[Update kill function] {}", e);
        }
    }
}

pub async fn db_update_last_death(player: String, message: String, timestamp: DateTime<Utc>, server: String, conn: &Pool<Postgres>) {
    let res = sqlx::query("SELECT player_name FROM lastdeaths WHERE LOWER(player_name) = LOWER($1) AND server = $2")
        .bind(&player)
        .bind(&server)
        .fetch_one(conn)
        .await;

    match res {
        Ok(_val) => {
            let res2 = sqlx::query("UPDATE lastdeaths SET last_death_message = $1, last_death_timestamp = $2 WHERE player_name = $3 AND server = $4")
                .bind(message)
                .bind(timestamp)
                .bind(player)
                .bind(server)
                .execute(conn)
                .await;

            match res2 {
                Ok(_v) => {},
                Err(e) => {
                    println!("{e}");
                }
            }
        },
        Err(e) => {
            let no_rows_err = e.to_string() == "no rows returned by a query that expected to return at least one row";

            if no_rows_err == true {
                let res = sqlx::query("INSERT INTO lastdeaths (player_name, first_death_message, first_death_timestamp, last_death_message, last_death_timestamp, server) VALUES ($1, $2, $3, $4, $5, $6)")
                    .bind(player)
                    .bind(message.clone())
                    .bind(timestamp.clone())
                    .bind(message)
                    .bind(timestamp)
                    .bind(server)
                    .execute(conn)
                    .await;

                match res {
                    Ok(_v) => {},
                    Err(e) => {
                        println!("{e}");
                    }
                }
            }
        }
    }
}

pub async fn db_update_last_kill(player: String, message: String, timestamp: DateTime<Utc>, server: String, conn: &Pool<Postgres>) {
    let res = sqlx::query("SELECT player_name FROM lastkills WHERE LOWER(player_name) = LOWER($1) AND server = $2")
        .bind(&player)
        .bind(&server)
        .fetch_one(conn)
        .await;

    match res {
        Ok(_val) => {
            let res2 = sqlx::query("UPDATE lastkills SET last_kill_message = $1, last_kill_timestamp = $2 WHERE player_name = $3 AND server = $4")
                .bind(message)
                .bind(timestamp)
                .bind(player)
                .bind(server)
                .execute(conn)
                .await;

            match res2 {
                Ok(_v) => {},
                Err(e) => {
                    println!("{e}");
                }
            }
        },
        Err(e) => {
            let no_rows_err = e.to_string() == "no rows returned by a query that expected to return at least one row";

            if no_rows_err == true {
                let res = sqlx::query("INSERT INTO lastkills (player_name, first_kill_message, first_kill_timestamp, last_kill_message, last_kill_timestamp, server) VALUES ($1, $2, $3, $4, $5, $6)")
                    .bind(player)
                    .bind(message.clone())
                    .bind(timestamp.clone())
                    .bind(message)
                    .bind(timestamp)
                    .bind(server)
                    .execute(conn)
                    .await;

                match res {
                    Ok(_v) => {},
                    Err(e) => {
                        println!("{e}");
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct JoinMessagePlayer {
    pub player_name: String,
    pub message: String
}

pub async fn db_get_joinmessage(player: String, server: String, conn: &Pool<Postgres>) -> Result<JoinMessagePlayer, Box<dyn Error>> {
    let res = sqlx::query("SELECT player_name, message FROM joinmsgs WHERE LOWER(player_name) = LOWER($1) AND server = $2")
        .bind(player)
        .bind(server)
        .fetch_one(conn)
        .await?;

    if res.len() > 0 {
        let player = JoinMessagePlayer { player_name: res.get("player_name"), message: res.get("message") };

        return Ok(player)
    } else {
        return Err("Join message not found".into())
    }
}


#[derive(Debug)]
pub struct LeaveMessagePlayer {
    pub player_name: String,
    pub message: String
}

pub async fn db_get_leavemessage(player: String, server: String, conn: &Pool<Postgres>) -> Result<LeaveMessagePlayer, Box<dyn Error>> {
    let res = sqlx::query("SELECT player_name, message FROM leavemsgs WHERE LOWER(player_name) = LOWER($1) AND server = $2")
        .bind(player)
        .bind(server)
        .fetch_one(conn)
        .await?;

    if res.len() > 0 {
        let player = LeaveMessagePlayer { player_name: res.get("player_name"), message: res.get("message") };

        return Ok(player)
    } else {
        return Err("Leave message not found".into())
    }
}

pub async fn db_update_nword_hard(player: String, count: i32, server: String, conn: &Pool<Postgres>) {
    let res = sqlx::query("INSERT INTO nwords (player_name, hard, soft, server) VALUES ($1, $2, $3, $4) ON CONFLICT (player_name, server) DO UPDATE SET hard = nwords.hard + EXCLUDED.hard")
        .bind(player)
        .bind(count)
        .bind(0)
        .bind(server)
        .execute(conn)
        .await;

    match res {
        Ok(_) => {},
        Err(e) => {
            println!("[Update nword hard function] {}", e);
        }
    }
}

pub async fn db_update_nword_soft(player: String, count: i32, server: String, conn: &Pool<Postgres>) {
    let res = sqlx::query("INSERT INTO nwords (player_name, hard, soft, server) VALUES ($1, $2, $3, $4) ON CONFLICT (player_name, server) DO UPDATE SET soft = nwords.soft + EXCLUDED.soft")
        .bind(player)
        .bind(0)
        .bind(count)
        .bind(server)
        .execute(conn)
        .await;

    match res {
        Ok(_) => {},
        Err(e) => {
            println!("[Update nword soft function] {}", e);
        }
    }
}

#[derive(Debug)]
pub struct ChatMessagePlayer {
    pub player_name: String,
    pub message: String,
    pub timestamp: DateTime<Utc>
}

pub async fn db_get_quote(player: String, server: String, conn: &Pool<Postgres>) -> Result<ChatMessagePlayer, Box<dyn Error>> {
    let res = sqlx::query("SELECT player_name, message, timestamp FROM chatlogs WHERE LOWER(player_name) = LOWER($1) AND server = $2 ORDER BY RANDOM() LIMIT 1")
        .bind(player)
        .bind(server)
        .fetch_one(conn)
        .await?;

    if res.len() > 0 {
        let player = ChatMessagePlayer { player_name: res.get("player_name"), message: res.get("message"), timestamp: res.get("timestamp") };

        return Ok(player)
    } else {
        return Err("Unable to get a random message".into())
    }
}

pub async fn db_batch_insert_chatlog(data: String, conn: &Pool<Postgres>) {
    let query = format!("INSERT INTO chatlogs (player_name, message, timestamp, server) VALUES {}", data);

    let res = sqlx::query(&query)
        .execute(conn)
        .await;

    match res {
        Ok(_) => {},
        Err(e) => {
            println!("{}", e);
        }
    }
}

pub async fn db_batch_update_chatcount(data: String, conn: &Pool<Postgres>) {
    let query = format!("INSERT INTO chatcount (player_name, count, server) VALUES {} ON CONFLICT (player_name, server) DO UPDATE SET count = chatcount.count + EXCLUDED.count", data);

    let res = sqlx::query(&query)
        .execute(conn)
        .await;

    match res {
        Ok(_) => {},
        Err(e) => {
            println!("[Batch update chatcount function] {}", e);
        }
    }
}