use std::{io::Cursor, process::Command};

use image::{DynamicImage, ImageFormat};
use rand::Rng;

#[derive(Debug)]
pub enum Button {
    Attack,
    FindMatch,
    Next,
}

type Coords = (u16, u16);

impl Button {
    fn into_coords(self) -> Coords {
        match self {
            Self::Attack => (150, 1250),
            Self::FindMatch => (2250, 850),
            Self::Next => (2900, 1050),
        }
    }
}

pub fn screen_shot() -> DynamicImage {
    let pngbytes = Command::new("adb")
        .arg("shell")
        .arg("screencap")
        .arg("-p")
        .output()
        .unwrap_or_else(|error| panic!("Failed to take screenshot with adb: {:?}", error))
        .stdout;

    let mut image = image::io::Reader::new(Cursor::new(pngbytes));
    image.set_format(ImageFormat::Png);
    image
        .decode()
        .unwrap_or_else(|error| panic!("Failed to decode transfered image: {:?}\nIs adb working? (try `adb list`) Is the phone unplugged?", error))
}

pub fn click(button: Button) {
    let coords = randomize_coords(button.into_coords());

    Command::new("adb")
        .arg("shell")
        .arg("input")
        .arg("mouse")
        .arg("tap")
        .arg(coords.0.to_string())
        .arg(coords.1.to_string())
        .output()
        .unwrap_or_else(|error| panic!("Failed to tap with adb: {:?}", error));
}

fn randomize_coords(coords: Coords) -> Coords {
    let radius = 50;
    let randx: i32 = rand::thread_rng().gen_range(-radius..=radius);
    let randy: i32 = rand::thread_rng().gen_range(-radius..=radius);

    (
        (coords.0 as i32 + randx) as u16,
        (coords.1 as i32 + randy) as u16,
    )
}
