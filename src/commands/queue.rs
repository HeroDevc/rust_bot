use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct QueueApiResponsePlayers {
    players: QueueApiResponseList
}

#[derive(Deserialize, Debug)]
struct QueueApiResponseList {
    list: Vec<QueueApiResponseListObject>
}

#[derive(Deserialize, Debug)]
struct QueueApiResponseListObject {
    name_clean: String
}

pub async fn get_queue() -> String {
    let url = "https://api.mcstatus.io/v2/status/java/2b2t.org".to_string();

    let http_client: reqwest::Client = reqwest::Client::new();

    let http_result = http_client.get(url).send().await;

    match http_result {
        Ok(v) => {
            let bv = v.text().await;

            match bv {
                Ok(v) => {
                    let json_resp: QueueApiResponsePlayers = serde_json::from_str(&v).expect("Failed");

                    let ingame = json_resp.players.list[0].name_clean.replace("In-game: ", "");
                    let inqueue = json_resp.players.list[1].name_clean.replace("Queue: ", "");
                    let inprioqueue = json_resp.players.list[2].name_clean.replace("Priority queue: ", "");

                    let msg = format!("In-game: {}, Queue: {}, Priority queue: {}", ingame, inqueue, inprioqueue);

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