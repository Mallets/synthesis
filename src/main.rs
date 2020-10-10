pub(crate) mod instruments;
pub(crate) mod oscillator;
pub(crate) mod sound;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

fn run<T>(device: &cpal::Device, config: &cpal::StreamConfig) -> Result<(), anyhow::Error>
where
    T: cpal::Sample,
{
    let sample_rate = config.sample_rate.0 as u64;
    let channels = config.channels as usize;

    // Create a logical clock
    let mut clock = oscillator::Clock::new(sample_rate);

    // Create a sound with one oscillator and four envelopes
    let freq_base: f64 = 110.0;
    let mut freq: f64 = freq_base;

    let mut ins = instruments::Simple::new();
    ins.note_on(freq, 1.0);

    let mut count = 0;
    let mut next_value = move || {
        // Tick the clock
        clock.tick();
        // Get the time for now
        let now = clock.get_time();
        if let Some(sample) = ins.get_sample(now) {
            sample
        } else {
            count += 1;
            if count <= 12 {
                freq *= (2.0 as f64).powf(1.0 / 12.0);
            } else {
                freq = freq_base;
                count = 0;
            }
            ins.note_on(freq, 1.0);
            ins.get_sample(now).unwrap()
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
