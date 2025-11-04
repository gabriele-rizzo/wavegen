pub mod generator;
pub mod waveform;

pub fn save(path: &str, samples: &[f32], rate: u32) -> hound::Result<()> {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create(path, spec)?;

    for sample in samples {
        let scaled = (sample * i16::MAX as f32) as i16;
        writer.write_sample(scaled)?;
    }

    writer.finalize()
}
