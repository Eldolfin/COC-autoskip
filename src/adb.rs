use crate::utils::randomize_coords;
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
    let pngbytes = run_commmand("adb shell screencap -p".to_string())
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

    run_commmand(format!(
        "adb shell input mouse tap {} {}",
        coords.0.to_string(),
        coords.1.to_string()
    ))
    .unwrap_or_else(|error| panic!("Failed to tap with adb: {:?}", error));
}

pub fn wait_volume_key() {
    run_commmand("adb logcat -c".to_string()).unwrap();
    run_commmand("adb logcat -e vol_ -m 1".to_string()).unwrap();
}

// simplifies the creation of commands
fn run_commmand(command: String) -> Result<std::process::Output, std::io::Error> {
    let mut args = command.split_whitespace();
    let mut command = Command::new(args.next().unwrap());
    let command = command.args(args);

    command.output()
}
