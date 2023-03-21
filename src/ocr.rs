use crate::{CROP_X, CROP_Y, CROP_WIDTH, CROP_HEIGHT};
use image::DynamicImage;
use leptess::{LepTess, Variable};

#[derive(Debug)]
pub struct Ressources {
    gold: u32,
    elixir: u32,
    _dark_elixir: u32,
}

pub struct RessourcesOCR {
    engine: LepTess,
}

impl RessourcesOCR {
    pub fn new() -> Self {
        let mut engine = LepTess::new(None, "eng").unwrap();

        engine.set_variable(Variable::TesseditCharWhitelist, "0123456789").unwrap();

        Self {
            engine
        }
    }

    pub fn get_ressources(&mut self, image: DynamicImage) -> Option<Ressources> {

        // preprocessing
        let cropped = crop(image);
        let cropped = cropped.grayscale();
        let cropped = cropped.brighten(-100);
        let mut cropped = cropped.adjust_contrast(100000.0);
        cropped.invert();


        // this doesnt work :(

        // self.engine
        //     .set_image_from_mem(cropped.as_bytes())
        //     .unwrap_or_else(|error| panic!("Could not load image in ocr engine: {:?}", error));
        cropped.save(".temp.png").unwrap();
        self.engine.set_image(".temp.png").unwrap();

        let detected_text = self.engine.get_utf8_text().unwrap();
        let mut lines = detected_text.lines();

        let gold = parse_ressource(lines.next()?, "gold")?;

        let elixir = parse_ressource(lines.next()?, "elixir")?;

        // for dark elixir, default to 0 if line couldn't be detected because maybe there is none
        let dark_elixir = parse_ressource(lines.next().unwrap_or("0"), "dark elixir")?;

        Some(Ressources {
            gold,
            elixir,
            _dark_elixir: dark_elixir,
        })
    }
}

impl Ressources {
    pub fn gold_and_elixir(&self) -> u32 {
        self.gold + self.elixir
    }
}

fn parse_ressource(ressource_str: &str, name: &str) -> Option<u32> {
    match ressource_str.parse::<u32>() {
        Ok(val) => Some(val),
        Err(error) => {
            println!(
                "[WARNING], failed to parse for {}, detected {}",
                name, ressource_str
            );

            println!("(error: {})", error);
            None
        }
    }
}

fn crop(image: DynamicImage) -> DynamicImage {
    image.crop_imm(CROP_X, CROP_Y, CROP_WIDTH, CROP_HEIGHT)
}
