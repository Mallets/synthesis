mod phase;
mod waveforms;

pub use phase::*;
pub use waveforms::*;

pub type Clock = f64;
pub type Sample = f32;

pub trait Oscillator: Send + Sync {
    fn clone_box(&self) -> Box<dyn Oscillator>;
    fn get_amplitude(&self, phase: Clock) -> Sample;
    fn set_gain(&mut self, amplitude: Sample);
    fn set_frequency(&mut self, frequency: Clock);
    fn set_phase(&mut self, phase: Clock);
}
