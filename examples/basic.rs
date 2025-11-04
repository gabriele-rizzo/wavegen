use std::time::Duration;

use wavegen::{generator::Generator, save, waveform::Waveform};

fn main() {
    let wave = Waveform::Sine(44.0);
    let rate = 44100;

    let mut generator = Generator::new(wave, rate);
    let samples = generator.generate(Duration::from_secs(2));

    save("test.wav", &samples, rate).unwrap();
}
