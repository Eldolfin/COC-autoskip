use colored::Colorize;
#[cfg(feature = "desktop_notification")]
use notify_rust::{Hint, Notification};
use std::io::{stdout, Write};

use adb::Button;
use ocr::{Ressources, RessourcesOCR};
#[cfg(feature = "desktop_notification")]
use sound::SoundEngine;
use utils::{input, random_sleep};

mod adb;
mod ocr;
#[cfg(feature = "desktop_notification")]
mod sound;
mod utils;

const DEFAULT_WANTED_TOTAL: u32 = 1200000;
/// max number of pixels in a box where buttons are clickable.
/// this should almost be equal to half the size of the smallest button.
const CLICK_RANDOM_RADIUS: u8 = 50;
/// after this amount of failed detection, the village will simply be skipped
const MAX_DETECTION_FAILS: u8 = 5;
/// to find these values, take a screenshot, open an image editor and make a box around the
/// upper left 3 values (gold, elixir and dark elixir). No other text should appear in the box!
const CROP_X: u32 = 129;
const CROP_Y: u32 = 181;
const CROP_WIDTH: u32 = 342 - CROP_X;
const CROP_HEIGHT: u32 = 380 - CROP_Y;

const ADB_IP: &str = "192.168.1.29:5555";
// finally, don't forget to change the values in adb.rs for the button positions

fn main() {
    let mut ocr = RessourcesOCR::new();
    #[cfg(feature = "desktop_notification")]
    let sound_engine = SoundEngine::default();

    let wanted_total = if cfg!(feature = "interactive") {
        prompt()
    } else {
        DEFAULT_WANTED_TOTAL
    };

    println!("Starting to search for a base with {wanted_total} gold+elixir!");

    adb::connect();
    start_attacking();

    loop {
        let ressources = search_loop(&mut ocr, wanted_total);
        #[cfg(feature = "desktop_notification")]
        notify_found(&ressources);
        #[cfg(feature = "desktop_notification")]
        sound_engine.play_sound();

        let answer = if cfg!(feature = "interactive") {
            input("Do you wish to continue searching? [y/N]")
                .to_lowercase()
                .contains('y')
        } else {
            false
        };

        if answer {
            adb::click(Button::Next);
        } else {
            break;
        }
    }
}

fn prompt() -> u32 {
    input(
        &format!("Hello fellow clasher, enter the desired amount of gold+elixir you want and press enter to begin the search (default: {DEFAULT_WANTED_TOTAL})"))
    .parse().unwrap_or(DEFAULT_WANTED_TOTAL)
}

fn start_attacking() {
    adb::click(Button::Attack);
    random_sleep();
    adb::click(Button::FindMatch);
}

fn search_loop(ocr: &mut RessourcesOCR, wanted_total: u32) -> Ressources {
    let mut fails = 0;
    loop {
        let image = adb::screen_shot();
        let ressources = ocr.get_ressources(image);

        if let Some(ressources) = ressources {
            print!("Found base {ressources} ");
            if ressources.gold_and_elixir() >= wanted_total {
                println!("{}", "It's good!  ✅".green().bold());
                return ressources;
            } else {
                println!("{}", "Skipping... ❌".red().bold());
                random_sleep();
                adb::click(Button::Next);
                fails = 0;
            }
        } else {
            fails += 1;
            print!("FAIL {fails}/{MAX_DETECTION_FAILS}...\r");
            stdout().flush().unwrap();
        };

        if fails >= MAX_DETECTION_FAILS {
            println!("⚠  Failed to detect ressources {MAX_DETECTION_FAILS} times, skipping");
            adb::click(Button::Next);
            fails = 0;
        }
    }
}

#[cfg(feature = "desktop_notification")]
fn notify_found(ressources: &Ressources) {
    let _ = Notification::new()
        .summary("COC autoskip")
        .appname("COC autoskip")
        .body(&format!(
            "A suitable village has been found with G+E = {}",
            ressources.gold_and_elixir()
        ))
        .icon("phone-symbolic.symbolic")
        .hint(Hint::SuppressSound(true))
        .show();
}
