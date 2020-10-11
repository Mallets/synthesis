mod simple;

use crate::oscillator::{Clock, Sample};
pub use simple::*;

enum Stage {
    Attack,
    Decay,
    Sustain,
    Release,
    None,
}

pub trait Instrument: Send {
    fn get_sample(&mut self, time: Clock) -> Option<Sample>;
    fn note_on(&mut self, frequency: Clock, gain: Sample);
    fn note_off(&mut self);
}
