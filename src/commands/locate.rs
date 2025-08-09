use rand::Rng;

pub fn locate(player: &String) -> String {
    let mut final_msg = format!("{} coords are ", player);

    let choices_vec: Vec<String> = vec!["p".to_string(), "n".to_string()];

    let axis_choice_x = &choices_vec[rand::rng().random_range(0..=1)];
    let axis_choice_z = &choices_vec[rand::rng().random_range(0..=1)];

    if axis_choice_x == "p" {
        let msg = format!("{} ", rand::rng().random_range(0..=30000000));

        final_msg += &msg;
    } else {
        let msg = format!("-{} ", rand::rng().random_range(0..=30000000));

        final_msg += &msg;
    }

    if axis_choice_z == "p" {
        let msg = format!("{}", rand::rng().random_range(0..=30000000));

        final_msg += &msg;
    } else {
        let msg = format!("-{} ", rand::rng().random_range(0..=30000000));

        final_msg += &msg;
    }

    final_msg
}