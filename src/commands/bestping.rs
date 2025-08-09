use std::collections::HashMap;
use azalea::player::PlayerInfo;
use uuid::Uuid;

pub fn get_best_ping(tablist: HashMap<Uuid, PlayerInfo>) -> String {
    let mut bp_player: &String = &String::new();
    let mut bp_ping: i32 = i32::MAX;

    for (_key, value) in tablist.iter() {
        if value.latency < bp_ping && value.latency != 0 {
            bp_ping = value.latency;
            bp_player = &value.profile.name;
        }
    }

    if bp_player == "" {
        let msg = format!("Failed to get the player with the lowest ping.");

        return msg
    }

    let msg = format!("Player with the lowest ping is {bp_player} with {bp_ping}ms");

    msg
}