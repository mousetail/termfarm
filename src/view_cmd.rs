use humantime::format_duration;

use crate::{crops::crop_registry, persistence::load_farm};
use std::time::{Duration, SystemTime};

pub fn view() {
    let farm = load_farm();
    let registry = crop_registry();
    let now = SystemTime::now();

    println!(" Plots");
    println!("-------");

    for (index, plot) in farm.plots.iter().enumerate() {
        let plot_number = index + 1;

        match plot.planted_crop.clone() {
            Some(crop_id) => match plot.planted_at.clone() {
                Some(at) => {
                    let crop = &registry[&crop_id];
                    let elapsed = at.elapsed().unwrap();
                    let remaining = crop.grow_time as f64 - elapsed.as_secs_f64();

                    if remaining <= 0.0 {
                        println!("[{plot_number}] {} {} ready to harvest", crop.icon, crop.id)
                    } else {
                        println!(
                            "[{plot_number}] {} {} {} left",
                            crop.icon,
                            crop.id,
                            format_duration(Duration::from_secs_f64(remaining))
                        )
                    }
                }
                None => {
                    usefulog::err(
                        "found crop without a plantedAt value! this should not be possible.",
                    );
                    std::process::exit(1);
                }
            },
            None => println!("[{plot_number}] empty"),
        }
    }
}
