use super::{Frequency, Oscillator, Sample, Time};

/* -------- Sine -------- */
/// A SineWave-based oscillator
#[derive(Clone, Copy, Debug)]
pub(crate) struct SineWave {
    frequency: Frequency,
    phase: Frequency,
}

impl SineWave {
    pub fn new(frequency: Frequency, phase: Frequency) -> Self {
        Self { frequency, phase }
    }
}

impl Oscillator for SineWave {
    fn get(&self, time: Time) -> Sample {
        (2.0 * std::f64::consts::PI * self.frequency * time + self.phase).sin() as Sample
    }

    fn set_frequency(&mut self, frequency: Frequency) {
        self.frequency = frequency;
    }

    fn set_phase(&mut self, phase: Frequency) {
        self.phase = phase;
    }
}

/* -------- Square -------- */
/// An oscillator based on a square wave
#[derive(Clone, Copy, Debug)]
pub(crate) struct SquareWave(SineWave);

impl SquareWave {
    pub fn new(frequency: Frequency, phase: Frequency) -> Self {
        Self(SineWave::new(frequency, phase))
    }
}

impl Oscillator for SquareWave {
    fn get(&self, time: Time) -> Sample {
        self.0.get(time).signum()
    }

    fn set_frequency(&mut self, frequency: Frequency) {
        self.0.set_frequency(frequency);
    }

    fn set_phase(&mut self, phase: Frequency) {
        self.0.set_phase(phase);
    }
}
