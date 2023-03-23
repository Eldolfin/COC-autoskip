use std::fmt::Display;
use std::io::Cursor;

use crate::{CROP_HEIGHT, CROP_WIDTH, CROP_X, CROP_Y};
use colored::Colorize;
use image::{imageops::FilterType, DynamicImage, ImageOutputFormat};
use leptess::{LepTess, Variable};

#[derive(Debug, PartialEq)]
pub struct Ressources {
    gold: u32,
    elixir: u32,
    dark_elixir: u32,
}

pub struct RessourcesOCR {
    engine: LepTess,
}

impl RessourcesOCR {
    pub fn new() -> Self {
        let mut engine = LepTess::new(None, "eng").unwrap();

        engine
            .set_variable(Variable::TesseditCharWhitelist, "0123456789n") // n == 11 for some reason
            .unwrap();

        Self { engine }
    }

    pub fn get_ressources(&mut self, image: &DynamicImage) -> Option<Ressources> {
        let image = preprocess(image);

        // the image roughly has this size so we pre-allocate the right size to gain some time
        let mut tiff = Cursor::new(Vec::with_capacity(11500));

        image.write_to(&mut tiff, ImageOutputFormat::Tiff).unwrap();

        self.engine
            .set_image_from_mem(tiff.get_ref())
            .unwrap_or_else(|error| panic!("Could not load image in ocr engine: {:?}", error));

        // does nothing, it's just to suppress a warning
        self.engine.set_source_resolution(70);

        let detected_text = self.engine.get_utf8_text().unwrap();
        let detected_text = detected_text.replace(' ', "").replace("n", "11");
        let mut lines = detected_text.lines();

        let gold = parse_ressource(lines.next()?, "gold")?;

        let elixir = parse_ressource(lines.next()?, "elixir")?;

        // for dark elixir, default to 0 if line couldn't be detected because maybe there is none
        let dark_elixir = parse_ressource(lines.next().unwrap_or("0"), "dark elixir")?;

        Some(Ressources {
            gold,
            elixir,
            dark_elixir,
        })
    }
}

fn preprocess(image: &DynamicImage) -> DynamicImage {
    let image = image.crop_imm(CROP_X, CROP_Y, CROP_WIDTH, CROP_HEIGHT);
    let image = DynamicImage::ImageLuma8(image.to_luma8());
    let image = image.brighten(-98);
    let mut image = image.adjust_contrast(f32::MAX);
    image.invert();
    image = image.resize_to_fill(
        106, 102,
        FilterType::Triangle,
    );
    image
}

impl Ressources {
    pub fn gold_and_elixir(&self) -> u32 {
        self.gold + self.elixir
    }
}

impl Display for Ressources {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let num_numbers = self.gold.checked_ilog10().unwrap_or(0)
            + self.elixir.checked_ilog10().unwrap_or(0)
            + self.dark_elixir.checked_ilog10().unwrap_or(0);
        let padding_count = 7 + 7 + 5 - 3 - num_numbers;
        let padding = " ".repeat(padding_count as usize);

        write!(
            f,
            "{padding}[{}{}, {}{}, DE: {}]",
            "G: ".bright_yellow(),
            self.gold.to_string().bright_yellow().bold(),
            "E: ".purple(),
            self.elixir.to_string().purple().bold(),
            self.dark_elixir
        )
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

#[cfg(test)]
mod tests {
    use super::{Ressources, RessourcesOCR};
    use image::io::Reader;
    use test_case::test_case;

    impl From<(u32, u32, u32)> for Ressources {
        fn from(value: (u32, u32, u32)) -> Self {
            Self {
                gold: value.0,
                elixir: value.1,
                dark_elixir: value.2,
            }
        }
    }

    #[test_case(1, (89_290, 92_921, 672); "Image 1")]
    #[test_case(2, (75_326, 139_838, 0); "Image 2")]
    #[test_case(3, (96_685, 97_303, 0); "Image 3")]
    #[test_case(4, (122_521, 69_917, 361); "Image 4")]
    #[test_case(5, (14_570, 24_323, 0); "Image 5")]
    #[test_case(6, (33_955, 32_952, 341); "Image 6")]
    #[test_case(7, (425_486, 469_671, 5_975); "Image 7")]
    #[test_case(8, (282_095, 148_063, 3_082); "Image 8")]
    #[test_case(9, (504_887, 400_384, 1_493); "Image 9")]
    #[test_case(10, (287_602, 204_352, 1_327); "Image 10")]
    #[test_case(11, (445_699, 349_948, 1_221); "Image 11")]
    #[test_case(12, (112_540, 391_968, 2_427); "Image 12")]
    #[test_case(13, (876_374, 993_356, 9_706); "Image 13")]
    fn get_ressources_tests(file_id: u8, ressource: (u32, u32, u32)) {
        let mut ressource_ocr = RessourcesOCR::new();
        let expected = ressource.into();
        let filename = format!("./assets/test_images/{file_id}.png");
        let image = Reader::open(filename).unwrap().decode().unwrap();

        let result = ressource_ocr.get_ressources(&image);
        assert!(result.is_some());
        let result = result.unwrap();

        {
            // TODO: remove this
            use super::preprocess;
            if result != expected {
                let image = preprocess(&image);
                image.save("fail.png").unwrap();
            }
        }

        assert_eq!(result, expected);
    }
}
