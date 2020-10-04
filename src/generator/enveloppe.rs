use super::{Sample, Time};

#[derive(Clone, Copy)]
pub(crate) struct Enveloppe {
    pub(crate) attack_dur: Time,
    pub(crate) decay_dur: Time,
    pub(crate) sustain_dur: Time,
    pub(crate) sustain_lvl: Sample,
    pub(crate) release_dur: Time,
}

impl Enveloppe {
    pub(crate) fn new(
        attack_dur: Time,
        decay_dur: Time,
        sustain_dur: Time,
        sustain_lvl: Sample,
        release_dur: Time,
    ) -> Self {
        Self {
            attack_dur,
            decay_dur,
            sustain_dur,
            sustain_lvl,
            release_dur,
        }
    }
}
