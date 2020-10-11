use crate::instrument::Instrument;
use crate::oscillator::{Phase, Sample};
use std::collections::HashMap;

pub(crate) struct Ensemble {
    phase: Phase,
    pub(crate) instruments: HashMap<String, Box<dyn Instrument>>,
}

impl Ensemble {
    pub(crate) fn new(phase: Phase) -> Self {
        Self {
            phase,
            instruments: HashMap::new(),
        }
    }

    pub(crate) fn add_instrument(mut self, name: &str, instrument: Box<dyn Instrument>) -> Self {
        self.instruments.insert(name.to_string(), instrument);
        self
    }

    pub(crate) fn get_instrument(&mut self, name: &str) -> Option<&mut Box<dyn Instrument>> {
        self.instruments.get_mut(name)
    }

    pub(crate) fn get_sample(&mut self) -> Option<Sample> {
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
