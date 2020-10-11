use crate::instrument::Instrument;
use crate::oscillator::{Phase, Sample};
use std::collections::HashMap;

pub struct Orchestra {
    phase: Phase,
    pub instruments: HashMap<String, Box<dyn Instrument>>,
}

impl Orchestra {
    pub fn new(phase: Phase) -> Self {
        Self {
            phase,
            instruments: HashMap::new(),
        }
    }

    pub fn add_instrument(mut self, name: &str, instrument: Box<dyn Instrument>) -> Self {
        self.instruments.insert(name.to_string(), instrument);
        self
    }

    pub fn get_instrument(&mut self, name: &str) -> Option<&mut Box<dyn Instrument>> {
        self.instruments.get_mut(name)
    }

    pub fn get_sample(&mut self) -> Option<Sample> {
        let now = self.phase.now();

        let sample = self
            .instruments
            .iter_mut()
            .fold(None, |sample, (_, instrument)| {
                if let Some(i) = instrument.get_sample(now) {
                    if let Some(s) = sample {
                        Some(s + i)
                    } else {
                        Some(i)
                    }
                } else {
                    sample
                }
            });

        self.phase.next();

        sample
    }
}
