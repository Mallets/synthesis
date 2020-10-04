mod generator;

use crate::generator::*;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

fn run<T>(device: &cpal::Device, config: &cpal::StreamConfig) -> Result<(), anyhow::Error>
where
    T: cpal::Sample,
{
    let sample_rate = config.sample_rate.0 as u64;
    let channels = config.channels as usize;

    // Create a logical clock
    let mut clock = Clock::new(sample_rate);
    // Produce a sinusoid of maximum amplitude.
    let mut freq = 220.0;
    let osc = SineWave::new(freq, 0.0);

    let base = 0.1f64;
    let env = Enveloppe::new(base, 2.0 * base, 5.0 * base, 0.5, 3.0 * base);
    let mut snd = Sound::new(Box::new(osc), env, clock.clone());

    let mut next_value = move || {
        clock.tick();
        if let Some(v) = snd.next() {
            v
        } else {
            snd.reset();
            freq *= 1.05;
            snd.oscillator.set_frequency(freq);
            snd.next().unwrap()
        }
    };

    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            for frame in data.chunks_mut(channels) {
                let value: T = cpal::Sample::from::<generator::Sample>(&next_value());
                for sample in frame.iter_mut() {
                    *sample = value;
                }
            }
        },
        |err| eprintln!("an error occurred on stream: {}", err),
    )?;
    stream.play()?;

    std::thread::sleep(std::time::Duration::from_secs(3600));

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
