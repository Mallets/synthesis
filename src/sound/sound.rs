use super::Harmonic;
use crate::oscillator::{Clock, Sample};

#[derive(Clone)]
pub(crate) struct Sound {
    pub(crate) harmonics: Vec<Harmonic>,
    is_init: bool,
}

impl Sound {
    pub(crate) fn new() -> Self {
        Self {
            harmonics: Vec::new(),
            is_init: false,
        }
    }

    pub(crate) fn add_harmonic(mut self, harmonic: Harmonic) -> Self {
        self.harmonics.push(harmonic);
        self
    }

    pub(crate) fn add_harmonics(mut self, harmonic: &[Harmonic]) -> Self {
        for h in harmonic.iter() {
            self.harmonics.push(h.clone());
        }
        self
    }

    pub(crate) fn get_amplitude(&self) -> Option<Sample> {
        self.harmonics.iter().fold(None, |amplitude, harmonic| {
            if let Some(a) = harmonic.get_amplitude() {
                if let Some(amp) = amplitude {
                    Some(a + amp)
                } else {
                    Some(a)
                }
            } else {
                amplitude
            }
        })
    }

    pub(crate) fn get_sample(&mut self, now: Clock) -> Option<Sample> {
        if !self.is_init {
            for h in self.harmonics.iter_mut() {
                h.reset(now);
            }
            self.is_init = true;
        }

        // Produce the sample
        let sample = self.harmonics.iter_mut().fold(None, |sample, harmonic| {
            if let Some(i) = harmonic.get_sample(now) {
                if let Some(s) = sample {
                    Some(s + i)
                } else {
                    Some(i)
                }
            } else {
                sample
            }
        });

        if sample.is_none() {
            self.is_init = false;
        }

        sample
    }
}
