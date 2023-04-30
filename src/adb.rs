use crate::{utils::randomize_coords, ADB_IP};
use image::{DynamicImage, ImageFormat};
use std::{io::Cursor, process::Command};

#[derive(Debug)]
pub enum Button {
    Attack,
    FindMatch,
    Next,
}

pub type Coords = (u16, u16);

impl Button {
    /// This should be changed to, according to where these buttons are placed
    /// on your screen. try to get the middle coordinates as there will be some randomizations!
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

pub fn connect() {
    Command::new("adb")
        .arg("connect")
        .arg(ADB_IP)
        .output()
        .unwrap_or_else(|err| panic!("Failed to execute adb connect {:?}", err));
}
