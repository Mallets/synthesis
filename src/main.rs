pub(crate) mod instrument;
pub(crate) mod orchestra;
pub(crate) mod oscillator;
pub(crate) mod sound;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

fn run<T>(device: &cpal::Device, config: &cpal::StreamConfig) -> Result<(), anyhow::Error>
where
    T: cpal::Sample,
{
    let sample_rate = config.sample_rate.0;
    let channels = config.channels as usize;

    // Create a logical clock
    let mut ensemble = orchestra::Ensemble::new(oscillator::Phase::new(sample_rate.into()))
        .add_instrument("simple", instrument::Simple::new());

    // Create a sound with one oscillator and four envelopes
    let freq_base: f64 = 165.0;
    let mut freq: f64 = freq_base;

    ensemble
        .get_instrument("simple")
        .unwrap()
        .note_on(freq, 1.0);

    let mut count = 0;
    let mut next_value = move || {
        if let Some(sample) = ensemble.get_sample() {
            sample
        } else {
            count += 1;
            if count <= 12 {
                freq *= (2.0 as f64).powf(1.0 / 12.0);
            } else {
                freq = freq_base;
                count = 0;
            }
            ensemble
                .get_instrument("simple")
                .unwrap()
                .note_on(freq, 1.0);
            0.0
        }
    };

    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            for frame in data.chunks_mut(channels) {
                let value: T = cpal::Sample::from::<oscillator::Sample>(&next_value());
                for sample in frame.iter_mut() {
                    *sample = value;
                }
            }
        },
        |err| eprintln!("an error occurred on stream: {}", err),
    )?;
    stream.play()?;

    std::thread::sleep(std::time::Duration::from_secs(3600));

    // let spec = hound::WavSpec {
    //     channels: 2,
    //     sample_rate: 44100,
    //     bits_per_sample: 32,
    //     sample_format: hound::SampleFormat::Float,
    // };
    // let mut writer = hound::WavWriter::create("sine.wav", spec).unwrap();
    // for _ in 0..10 {
    //     for _ in 0..44100 {
    //         let sample = next_value();
    //         writer.write_sample(sample as f32).unwrap();
    //         writer.write_sample(sample as f32).unwrap();
    //     }
    // }

    Ok(())
}

fn main() -> Result<(), anyhow::Error> {
    let host = cpal::default_host();

    let device = host
        .default_output_device()
        .expect("failed to find a default output device");
    let config = device.default_output_config()?;

    match config.sample_format() {
        cpal::SampleFormat::F32 => run::<f32>(&device, &config.into())?,
        cpal::SampleFormat::I16 => run::<i16>(&device, &config.into())?,
        cpal::SampleFormat::U16 => run::<u16>(&device, &config.into())?,
    }

    Ok(())
}
