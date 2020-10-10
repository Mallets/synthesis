mod simple;

use crate::oscillator::{Frequency, Sample, Time};
pub(crate) use simple::*;

enum Stage {
    Attack,
    Decay,
    Sustain,
    Release,
    None,
}

pub(crate) trait Instrument {
    fn get_sample(&mut self, time: Time) -> Option<Sample>;
    fn note_on(&mut self, frequency: Frequency, gain: Sample);
    fn note_off(&mut self);
}
