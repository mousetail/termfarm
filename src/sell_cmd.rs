use crate::{persistence::load_farm, sell::sell_crop};
use std::{collections::HashMap, process::exit};

pub fn sell(crop_id: String, amount: u16) {
    let mut farm = load_farm();

    let crops = farm.inventory.crops.get_or_insert_with(HashMap::new);
    let owned = crops.get(&crop_id).copied().unwrap_or(0);

    if !crops.contains_key(&crop_id) {
        println!("You don't have any {crop_id} to sell");
        exit(1);
    }
    if amount > owned {
        println!("You only have {owned} {crop_id}");
        exit(1);
    }

    sell_crop(crop_id, amount, true);
}
