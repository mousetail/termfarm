use uuid::Uuid;

use crate::{
    models::Plot,
    persistence::{load_farm, save_farm},
    plot_pricing::next_plot_price,
};
use std::{process::exit, time::SystemTime};

pub fn buyplot(showprice: bool) {
    let mut farm = load_farm();

    let current_plots = farm.plots.iter().count();
    let price = next_plot_price(current_plots as u16);

    if showprice {
        println!("Plot {} costs {price} coins", current_plots + 1);
        return;
    }

    if farm.coins < price as u32 {
        println!(
            "Not enough coins.\nYou have {}, but need {price} coins",
            farm.coins
        );
        exit(1);
    }

    farm.coins -= price as u32;
    farm.plots.push(Plot {
        id: Uuid::new_v4(),
        planted_crop: None,
        planted_at: None,
    });
    farm.last_updated = SystemTime::now();

    match save_farm(&farm) {
        true => {
            println!("Bought new plot for {price}");
            println!("Total plots is now {}", farm.plots.iter().count())
        }
        false => {
            usefulog::err("failed to save farm");
            exit(1);
        }
    }
}
