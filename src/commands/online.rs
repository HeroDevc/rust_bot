use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct OnlineApiResponseStruct {
    players: OnlinePlayersStruct
}

#[derive(Deserialize, Debug)]
struct OnlinePlayersStruct {
    online: i32,
    max: i32
}

pub async fn get_online_players(server: String) -> String {
    let url = format!("https://api.mcstatus.io/v2/status/java/{}", server);

    let http_client: reqwest::Client = reqwest::Client::new();

    let http_result = http_client.get(url).send().await;

    match http_result {
        Ok(v) => {
            let bv = v.text().await;

            match bv {
                Ok(v) => {
                    let json_resp: OnlineApiResponseStruct = serde_json::from_str(&v).expect("Failed");

                    let msg = format!("{}/{} players online", json_resp.players.online, json_resp.players.max);

                    msg
                },
                Err(_e) => {
                    let msg = format!("Failed to get the server");

                    msg
                }
            }
        },
        Err(_e) => {
            let msg = format!("Failed to get the server");

            msg
        }
    }
}