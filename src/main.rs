use adb::Button;
use ocr::RessourcesOCR;
use rand::Rng;
use rodio::source::{Buffered, Source};
use rodio::Decoder;
use std::io::{BufReader, Cursor};
use std::{io::BufRead, thread::sleep, time::Duration};

mod adb;
mod ocr;

const DEFAULT_WANTED_TOTAL: u32 = 1200000;

const CROP_X: u32 = 129;
const CROP_Y: u32 = 181;
const CROP_WIDTH: u32 = 342 - CROP_X;
const CROP_HEIGHT: u32 = 380 - CROP_Y;

const MAX_DETECTION_FAILS: u8 = 5;

fn main() {
    let wanted_total = prompt();
    println!("Starting to search for a base with {wanted_total} gold+elixir!");

    let mut ocr = RessourcesOCR::new();
    let sound = load_sound();
    let (_stream, handle) =
        rodio::OutputStream::try_default().expect("Output stream failed to open");
    let sink = rodio::Sink::try_new(&handle).expect("Sink open failed");

    start_attacking();

    loop {
        search_loop(&mut ocr, wanted_total);
        sink.append(sound.clone());
        let answer = input("Do you wish to continue searching? [y/N]");
        if !answer.to_lowercase().contains('y') {
            break;
        } else {
            adb::click(Button::Next);
        }
    }
}

fn search_loop(ocr: &mut RessourcesOCR, wanted_total: u32) {
    let mut fails = 1;
    loop {
        let image = adb::screen_shot();
        let ressources = ocr.get_ressources(image);

        if let Some(ressources) = ressources {
            if ressources.gold_and_elixir() >= wanted_total {
                break;
            }
            random_sleep();
            adb::click(Button::Next);
            fails = 1;
        } else {
            println!("FAIL {fails}/{MAX_DETECTION_FAILS}");
            fails += 1;
        };

        if fails > MAX_DETECTION_FAILS {
            println!("Failed to detect ressources more than {MAX_DETECTION_FAILS} times, skipping");
            adb::click(Button::Next);
        }
    }
}

fn load_sound() -> Buffered<Decoder<BufReader<Cursor<&'static [u8; 262222]>>>> {
    let file = Cursor::new(include_bytes!("./found-soundeffect.wav"));
    rodio::Decoder::new(BufReader::new(file))
        .unwrap()
        .buffered()
}

fn start_attacking() {
    adb::click(Button::Attack);
    random_sleep();
    adb::click(Button::FindMatch);
}

fn prompt() -> u32 {
    input(
        &format!("Hello fellow clasher, enter the desired amount of gold+elixir you want and press enter to begin the search (default: {DEFAULT_WANTED_TOTAL})"))
    .parse().unwrap_or(DEFAULT_WANTED_TOTAL)
}

fn input(message: &str) -> String {
    println!("{message}");
    std::io::stdin().lock().lines().next().unwrap().unwrap()
}

fn random_sleep() {
    let mut rng = rand::thread_rng();
    let ms = rng.gen_range(400..1000);
    let duration = Duration::from_millis(ms);
    sleep(duration);
}
