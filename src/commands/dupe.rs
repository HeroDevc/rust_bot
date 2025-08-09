use rand::Rng;

pub fn dupe(player: &String, item: String) -> String {
    let dupes: Vec<String> = vec!["minecart dupe".to_string(), "11/11 dupe".to_string(), "donkey dupe".to_string(), "popbob sex dupe".to_string()];

    let rand_int = rand::rng().random_range(0..=3);

    let msg = format!("{} duped {} with {}", player, item, dupes[rand_int]);

    msg
}