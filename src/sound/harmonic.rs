use super::Envelope;
use crate::oscillator::{Clock, Oscillator, Sample};

pub struct Harmonic {
    pub oscillator: Box<dyn Oscillator>,
    envelopes: Vec<Envelope>,
    index: Option<usize>,
}

impl Harmonic {
    pub fn new(oscillator: Box<dyn Oscillator>) -> Self {
        Self {
            oscillator,
            envelopes: Vec::new(),
            index: Some(0),
        }
    }

    pub fn reset(&mut self, now: Clock) {
        self.index = Some(0);
        let mut time = now;
        for env in self.envelopes.iter_mut() {
            env.reset(time);
            time += env.duration;
        }
    }

    pub fn add_envelope(mut self, envelope: Envelope) -> Self {
        self.envelopes.push(envelope);
        self
    }

    pub fn add_envelopes(mut self, envelopes: &[Envelope]) -> Self {
        envelopes.iter().for_each(|env| {
            self.envelopes.push(*env);
        });
        self
    }

    pub fn get_amplitude(&self) -> Option<Sample> {
        if let Some(idx) = self.index {
            if let Some(env) = self.envelopes.get(idx) {
                return env.get_amplitude();
            }
        }
        None
    }

    pub fn get_sample(&mut self, now: Clock) -> Option<Sample> {
        // Get the eneveloppe index
        let mut idx = self.index?;

        loop {
            // Get the target envelope
            if let Some(env) = self.envelopes.get_mut(idx) {
                // Get the evenlope amplitude
                if let Some(amp) = env.get_sample(now) {
                    // Compute the sample value from the oscillators
                    let spl = amp * self.oscillator.get_amplitude(now);
                    return Some(spl);
                } else {
                    // This envelope is over, try the next one
                    idx += 1;
                    self.index = Some(idx);
                }
            } else {
                // No evelopes left, return
                self.index = None;
                return None;
            }
        }
    }
}

impl Clone for Harmonic {
    fn clone(&self) -> Self {
        Self {
            oscillator: self.oscillator.clone_box(),
            envelopes: self.envelopes.clone(),
            index: self.index,
        }
    }
}
