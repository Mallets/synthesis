use super::interpolation;
use super::{Clock, Enveloppe, Oscillator, Sample, Time};

enum Phase {
    Attack,
    Decay,
    Sustain,
    Release,
}

pub(crate) struct Sound {
    pub(crate) oscillator: Box<dyn Oscillator>,
    pub(crate) enveloppe: Enveloppe,
    clock: Clock,
    phase: Phase,
    start: Option<Time>,
}

// @TODO: remove unsafe
unsafe impl Send for Sound {}

impl Sound {
    pub(crate) fn new(oscillator: Box<dyn Oscillator>, enveloppe: Enveloppe, clock: Clock) -> Self {
        Self {
            oscillator,
            enveloppe,
            clock,
            phase: Phase::Attack,
            start: None,
        }
    }

    pub(crate) fn reset(&mut self) {
        self.phase = Phase::Attack;
        self.start = None;
    }
}

impl Iterator for Sound {
    type Item = Sample;

    fn next(&mut self) -> Option<Sample> {
        let start = if let Some(start) = self.start {
            start
        } else {
            let now = self.clock.get_time();
            self.start = Some(now);
            now
        };

        let now = self.clock.get_time();
        match self.phase {
            Phase::Attack => {
                let fact = interpolation::linear(start, start + self.enveloppe.attack_dur, now);
                // println!("Attack: {}", fact);

                let value = fact * self.oscillator.get(now);
                if now >= start + self.enveloppe.attack_dur {
                    self.phase = Phase::Decay;
                    self.start = None;
                }
                Some(value)
            }
            Phase::Decay => {
                let fact = 1.0
                    - self.enveloppe.sustain_lvl
                        * interpolation::linear(start, start + self.enveloppe.decay_dur, now);
                // println!("Decay: {}", fact);

                let value = fact * self.oscillator.get(now);
                if now >= start + self.enveloppe.decay_dur {
                    self.phase = Phase::Sustain;
                    self.start = None;
                }
                Some(value)
            }
            Phase::Sustain => {
                let fact = self.enveloppe.sustain_lvl;
                // println!("Sustain: {}", fact);

                let value = fact * self.oscillator.get(now);
                if now >= start + self.enveloppe.sustain_dur {
                    self.phase = Phase::Release;
                    self.start = None;
                }
                Some(value)
            }
            Phase::Release => {
                let fact = self.enveloppe.sustain_lvl
                    - self.enveloppe.sustain_lvl
                        * interpolation::linear(start, start + self.enveloppe.release_dur, now);
                // println!("Release: {}", fact);

                let value = fact * self.oscillator.get(now);
                if now >= start + self.enveloppe.release_dur {
                    None
                } else {
                    Some(value)
                }
            }
        }
    }
}
