mod phase;
mod waveforms;

pub(crate) use phase::*;
pub(crate) use waveforms::*;

pub(crate) type Clock = f64;
pub(crate) type Sample = f32;

pub(crate) trait Oscillator: Send + Sync {
    fn clone_box(&self) -> Box<dyn Oscillator>;
    fn get_amplitude(&self, phase: Clock) -> Sample;
    fn set_gain(&mut self, amplitude: Sample) -> &mut dyn Oscillator;
    fn set_frequency(&mut self, frequency: Clock) -> &mut dyn Oscillator;
    fn set_phase(&mut self, phase: Clock) -> &mut dyn Oscillator;
}
