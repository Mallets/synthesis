use super::{Clock, Oscillator, Sample};

/* -------- Sine -------- */
/// A SineWave-based oscillator
#[derive(Clone, Copy, Debug)]
pub(crate) struct SineWave {
    gain: Sample,
    frequency: Clock,
    phase: Clock,
}

impl SineWave {
    pub fn new(gain: Sample, frequency: Clock, phase: Clock) -> Box<dyn Oscillator> {
        Box::new(SineWave::new_inner(gain, frequency, phase))
    }

    fn new_inner(gain: Sample, frequency: Clock, phase: Clock) -> Self {
        Self {
            gain,
            frequency,
            phase,
        }
    }
}

impl Oscillator for SineWave {
    fn clone_box(&self) -> Box<dyn Oscillator> {
        Box::new(self.clone())
    }

    fn get_amplitude(&self, time: Clock) -> Sample {
        let value = (2.0 * std::f64::consts::PI * self.frequency * time + self.phase).sin();
        self.gain * (value as Sample)
    }

    fn set_gain(&mut self, gain: Sample) -> &mut dyn Oscillator {
        self.gain = gain;
        self
    }

    fn set_frequency(&mut self, frequency: Clock) -> &mut dyn Oscillator {
        self.frequency = frequency;
        self
    }

    fn set_phase(&mut self, phase: Clock) -> &mut dyn Oscillator {
        self.phase = phase;
        self
    }
}

/* -------- Square -------- */
/// An SquareWave-based oscillator
#[derive(Clone, Copy, Debug)]
pub(crate) struct SquareWave(SineWave);

impl SquareWave {
    pub fn new(gain: Sample, frequency: Clock, phase: Clock) -> Box<dyn Oscillator> {
        Box::new(SquareWave::new_inner(gain, frequency, phase))
    }

    fn new_inner(gain: Sample, frequency: Clock, phase: Clock) -> Self {
        Self(SineWave::new_inner(gain, frequency, phase))
    }
}

impl Oscillator for SquareWave {
    fn clone_box(&self) -> Box<dyn Oscillator> {
        Box::new(self.clone())
    }

    fn get_amplitude(&self, time: Clock) -> Sample {
        self.0.gain * self.0.get_amplitude(time).signum()
    }

    fn set_gain(&mut self, gain: Sample) -> &mut dyn Oscillator {
        self.0.set_gain(gain);
        self
    }

    fn set_frequency(&mut self, frequency: Clock) -> &mut dyn Oscillator {
        self.0.set_frequency(frequency);
        self
    }

    fn set_phase(&mut self, phase: Clock) -> &mut dyn Oscillator {
        self.0.set_phase(phase);
        self
    }
}
