use rand::{rng, Rng};

pub fn task() -> String {
    let tasks_vec = ["Chop down a tree", "Kill a spider", "Kill a zombie", "Kill a skeleton", "Get diamonds", "Get iron", "Get coal", "Craft an iron pickaxe", "Eat an golden apple",
        "Cook a steak", "Make a cake", "Mine sand"];

    let random_task = tasks_vec[rng().random_range(0..=tasks_vec.len() - 1)];

    let msg = format!("{}", random_task);

    msg
}