use std::collections::HashMap;
use azalea::player::PlayerInfo;
use uuid::Uuid;

pub fn get_worst_ping(tablist: HashMap<Uuid, PlayerInfo>) -> String {
    let mut wp_player: &String = &String::new();
    let mut wp_ping: i32 = 0;

    for (_key, value) in tablist.iter() {
        if value.latency > wp_ping && value.latency != 0 {
            wp_ping = value.latency;
            wp_player = &value.profile.name;
        }
    }

    if wp_player == "" {
        let msg = format!("Failed to get the player with the highest ping.");

        return msg
    }

    let msg = format!("Player with the highest ping is {wp_player} with {wp_ping}ms");

    msg
}