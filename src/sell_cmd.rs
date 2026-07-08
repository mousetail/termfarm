use crate::{
    market::sell_price,
    persistence::{load_farm, save_farm},
};
use std::{collections::HashMap, process::exit, time::SystemTime};

pub fn sell(crop_id: String, mut amount: u16) {
    let mut farm = load_farm();

    let crops = farm.inventory.crops.get_or_insert_with(HashMap::new);
    let owned = crops.get(&crop_id).copied().unwrap_or(0);

    if amount == 0 {
        amount = owned;
    }

    if !crops.contains_key(&crop_id) {
        println!("You don't have any {crop_id} to sell");
        exit(1);
    }
    if amount == 0 {
        usefulog::err("Invalid amount");
        exit(1);
    }
    if amount > owned {
        println!("You only have {owned} {crop_id}");
        exit(1);
    }

    *crops.entry(crop_id.clone()).or_insert(0) -= amount;
    if *crops.entry(crop_id.clone()).or_insert(0) == 0 {
        crops.remove_entry(&crop_id);
    }

    let price = sell_price(crop_id.clone(), &farm);
    let total = price * amount;

    farm.coins += total as u32;
    farm.last_updated = SystemTime::now();

    match save_farm(&farm) {
        true => {
            println!("Sold {amount}x {crop_id} for {total} coins ({price} each)")
        }
        false => {
            usefulog::err("failed to save farm");
            exit(1);
        }
    }
}
