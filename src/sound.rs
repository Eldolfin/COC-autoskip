use rodio::source::Buffered;
use rodio::Sink;
use rodio::{Decoder, OutputStream, Source};
use std::io::{BufReader, Cursor};

pub struct SoundEngine {
    // not used but cannot be drop or sound doesnt work
    _stream: OutputStream,
    sink: Sink,
    sound_effect: Buffered<Decoder<BufReader<Cursor<&'static [u8; 262222]>>>>,
}

impl SoundEngine {
    pub fn play_sound(&self) {
        self.sink.append(self.sound_effect.clone())
    }
}

impl Default for SoundEngine {
    fn default() -> Self {
        let (_stream, handle) = OutputStream::try_default().expect("Output stream failed to open");
        let sink = Sink::try_new(&handle).expect("Sink open failed");
        let sound_file = Cursor::new(include_bytes!("./found-soundeffect.wav"));
        let sound = Decoder::new(BufReader::new(sound_file)).unwrap().buffered();

        Self {
            _stream,
            sink,
            sound_effect: sound,
        }
    }
}
