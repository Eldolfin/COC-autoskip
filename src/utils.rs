use std::{io::BufRead, time::Duration, thread::sleep};
use rand::Rng;
use crate::{adb::Coords, CLICK_RANDOM_RADIUS};

pub fn input(message: &str) -> String {
    println!("{message}");
    std::io::stdin().lock().lines().next().unwrap().unwrap()
}

pub fn random_sleep() {
    let mut rng = rand::thread_rng();
    let ms = rng.gen_range(400..1000);
    let duration = Duration::from_millis(ms);
    sleep(duration);
}


pub fn randomize_coords(coords: Coords) -> Coords {
    let radius = CLICK_RANDOM_RADIUS as i32;
    let randx: i32 = rand::thread_rng().gen_range(-radius..=radius);
    let randy: i32 = rand::thread_rng().gen_range(-radius..=radius);

    (
        (coords.0 as i32 + randx) as u16,
        (coords.1 as i32 + randy) as u16,
    )
}
