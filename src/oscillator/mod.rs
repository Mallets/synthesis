mod clock;
mod waveforms;

pub(crate) use clock::*;
pub(crate) use waveforms::*;

pub(crate) type Frequency = f64;
pub(crate) type Time = f64;
pub(crate) type Sample = f32;

pub(crate) trait Oscillator: Send + Sync {
    fn clone_box(&self) -> Box<dyn Oscillator>;
    fn get_amplitude(&self, time: Time) -> Sample;
    fn set_gain(&mut self, amplitude: Sample) -> &mut dyn Oscillator;
    fn set_frequency(&mut self, frequency: Frequency) -> &mut dyn Oscillator;
    fn set_phase(&mut self, phase: Frequency) -> &mut dyn Oscillator;
}
