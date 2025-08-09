use std::collections::HashMap;
use azalea::player::PlayerInfo;
use uuid::Uuid;

pub fn get_ping(player_name: &str, tablist: HashMap<Uuid, PlayerInfo>, player_uuid: Uuid) -> String {
    let player_ping: Option<&azalea::player::PlayerInfo> = tablist.get(&player_uuid);

    match player_ping {
        Some(val2) => {
            let msg = format!("{}'s ping us {}ms", player_name, val2.latency);
            
            return msg
        },
        None => {
            let msg = format!("Failed to get {}'s ping.", player_name);
            return msg
        }
    }
}