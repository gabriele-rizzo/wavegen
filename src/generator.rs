use std::{f32, time::Duration};

use rand::Rng;

use crate::waveform::Waveform;

pub struct Generator {
    waveform: Waveform,
    rate: u32,
    phase: f32,
}

impl Generator {
    pub fn new(waveform: Waveform, rate: u32) -> Self {
        Self {
            waveform,
            rate,
            phase: 0.0,
        }
    }

    pub fn generate(&mut self, duration: Duration) -> Vec<f32> {
        let n = (duration.as_secs_f32() * self.rate as f32) as usize;
        let mut samples = Vec::with_capacity(n);
        let dt = 1.0 / self.rate as f32;

        for _ in 0..n {
            let value = match self.waveform {
                Waveform::Sine(f) => (2.0 * f32::consts::PI * f * self.phase).sin(),
                Waveform::Square(f) => {
                    if (self.phase * f) % 1.0 < 0.5 {
                        1.0
                    } else {
                        -1.0
                    }
                }
                Waveform::Triangle(f) => 2.0 * ((self.phase * f) % 1.0 - 0.5).abs() - 1.0,
                Waveform::Saw(f) => 2.0 * ((self.phase * f) % 1.0) - 1.0,
                Waveform::Noise => rand::rng().random_range(-1.0..1.0),
            };

            samples.push(value);
            self.phase += dt;
        }

        samples
    }
}
