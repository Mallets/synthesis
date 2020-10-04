mod clock;
mod enveloppe;
mod interpolation;
mod oscillator;
mod sound;

pub(crate) use clock::*;
pub(crate) use enveloppe::*;
pub(crate) use oscillator::*;
pub(crate) use sound::*;

pub(crate) type Frequency = f64;
pub(crate) type Time = f64;
pub(crate) type Sample = f32;

pub(crate) trait Oscillator {
    fn get(&self, time: Time) -> Sample;
    fn set_frequency(&mut self, frequency: Frequency);
    fn set_phase(&mut self, phase: Frequency);
}
