use super::{Clock, Oscillator, Sample};
use std::convert::Into;

/* -------- Sine -------- */
/// A SineWave-based oscillator
#[derive(Clone, Copy, Debug)]
pub struct SineWave {
    gain: Sample,
    frequency: Clock,
    phase: Clock,
}

impl SineWave {
    pub fn new(gain: Sample, frequency: Clock, phase: Clock) -> Self {
        Self {
            gain,
            frequency,
            phase,
        }
    }
}

impl Oscillator for SineWave {
    fn clone_box(&self) -> Box<dyn Oscillator> {
        Box::new(*self)
    }

    fn get_amplitude(&self, time: Clock) -> Sample {
        let value = (2.0 * std::f64::consts::PI * self.frequency * time + self.phase).sin();
        self.gain * (value as Sample)
    }

    fn set_gain(&mut self, gain: Sample) {
        self.gain = gain;
    }

    fn set_frequency(&mut self, frequency: Clock) {
        self.frequency = frequency;
    }

    fn set_phase(&mut self, phase: Clock) {
        self.phase = phase;
    }
}

impl Into<Box<dyn Oscillator>> for SineWave {
    fn into(self) -> Box<dyn Oscillator> {
        Box::new(self)
    }
}

/* -------- Square -------- */
/// A SquareWave-based oscillator
#[derive(Clone, Copy, Debug)]
pub struct SquareWave(SineWave);

impl SquareWave {
    pub fn new(gain: Sample, frequency: Clock, phase: Clock) -> Self {
        Self(SineWave::new(gain, frequency, phase))
    }
}

impl Oscillator for SquareWave {
    fn clone_box(&self) -> Box<dyn Oscillator> {
        Box::new(*self)
    }

    fn get_amplitude(&self, time: Clock) -> Sample {
        self.0.gain * self.0.get_amplitude(time).signum()
    }

    fn set_gain(&mut self, gain: Sample) {
        self.0.set_gain(gain);
    }

    fn set_frequency(&mut self, frequency: Clock) {
        self.0.set_frequency(frequency);
    }

    fn set_phase(&mut self, phase: Clock) {
        self.0.set_phase(phase);
    }
}

impl Into<Box<dyn Oscillator>> for SquareWave {
    fn into(self) -> Box<dyn Oscillator> {
        Box::new(self)
    }
}
