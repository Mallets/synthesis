use super::{Instrument, Stage};
use crate::oscillator::{Clock, Sample, SineWave, SquareWave};
use crate::sound::{interpolation, Envelope, Harmonic, Sound};

pub struct Simple {
    attack: Sound,
    decay: Sound,
    sustain: Sound,
    release: Sound,
    gain: Sample,
    stage: Stage,
    last_sample: Clock,
}

impl Simple {
    pub fn make(slot: Clock) -> Box<dyn Instrument> {
        Box::new(Simple::new(slot))
    }

    pub fn new(slot: Clock) -> Self {
        let attack = Sound::new()
            .add_harmonic(
                Harmonic::new(SineWave::make(1.0, 0.0, 0.0)).add_envelope(Envelope::new(
                    0.0,
                    1.0,
                    slot,
                    interpolation::linear,
                )),
            )
            .add_harmonic(
                Harmonic::new(SquareWave::make(0.01, 0.0, 0.0))
                    .add_envelope(Envelope::new(0.0, 0.5, slot / 2.0, interpolation::linear))
                    .add_envelope(Envelope::new(0.5, 0.01, slot / 2.0, interpolation::linear)),
            );

        let decay = Sound::new()
            .add_harmonic(
                Harmonic::new(SineWave::make(1.0, 0.0, 0.0)).add_envelope(Envelope::new(
                    1.0,
                    0.5,
                    slot,
                    interpolation::linear,
                )),
            )
            .add_harmonic(
                Harmonic::new(SquareWave::make(0.01, 0.0, 0.0)).add_envelope(Envelope::new(
                    0.01,
                    0.01,
                    slot,
                    interpolation::linear,
                )),
            );

        let sustain = Sound::new()
            .add_harmonic(
                Harmonic::new(SineWave::make(1.0, 0.0, 0.0)).add_envelope(Envelope::new(
                    0.5,
                    0.5,
                    slot,
                    interpolation::linear,
                )),
            )
            .add_harmonic(
                Harmonic::new(SquareWave::make(0.01, 0.0, 0.0)).add_envelope(Envelope::new(
                    0.01,
                    0.01,
                    slot,
                    interpolation::linear,
                )),
            );

        let release = Sound::new()
            .add_harmonic(
                Harmonic::new(SineWave::make(1.0, 0.0, 0.0)).add_envelope(Envelope::new(
                    0.5,
                    0.0,
                    slot,
                    interpolation::linear,
                )),
            )
            .add_harmonic(
                Harmonic::new(SquareWave::make(0.01, 0.0, 0.0)).add_envelope(Envelope::new(
                    0.01,
                    0.0,
                    slot,
                    interpolation::linear,
                )),
            );

        Self {
            attack,
            decay,
            sustain,
            release,
            gain: 0.0,
            stage: Stage::None,
            last_sample: 0.0,
        }
    }
}

impl Instrument for Simple {
    fn get_sample(&mut self, time: Clock) -> Option<Sample> {
        self.last_sample = time;

        let sample = loop {
            match self.stage {
                Stage::Attack => {
                    if let Some(s) = self.attack.get_sample(time) {
                        break Some(s);
                    } else {
                        self.stage = Stage::Decay;
                        continue;
                    }
                }
                Stage::Decay => {
                    if let Some(s) = self.decay.get_sample(time) {
                        break Some(s);
                    } else {
                        self.stage = Stage::Sustain;
                        continue;
                    }
                }
                Stage::Sustain => {
                    if let Some(s) = self.sustain.get_sample(time) {
                        break Some(s);
                    } else {
                        self.stage = Stage::Release;
                        continue;
                    }
                }
                Stage::Release => {
                    if let Some(s) = self.release.get_sample(time) {
                        break Some(s);
                    } else {
                        self.stage = Stage::None;
                        continue;
                    }
                }
                Stage::None => break None,
            }
        };

        if let Some(s) = sample {
            Some(self.gain * s)
        } else {
            None
        }
    }

    fn note_on(&mut self, frequency: Clock, gain: Sample) {
        self.stage = Stage::Attack;
        self.gain = gain;

        self.attack.harmonics[0].oscillator.set_frequency(frequency);
        self.attack.harmonics[1]
            .oscillator
            .set_frequency(2.0 * frequency);

        self.decay.harmonics[0].oscillator.set_frequency(frequency);
        self.decay.harmonics[1]
            .oscillator
            .set_frequency(2.0 * frequency);

        self.sustain.harmonics[0]
            .oscillator
            .set_frequency(frequency);
        self.sustain.harmonics[1]
            .oscillator
            .set_frequency(2.0 * frequency);

        self.release.harmonics[0]
            .oscillator
            .set_frequency(frequency);
        self.release.harmonics[1]
            .oscillator
            .set_frequency(2.0 * frequency);
    }

    fn note_off(&mut self) {
        match self.stage {
            Stage::Attack => {
                // for (i, h) in self.attack.harmonics.iter().enumerate() {
                //     if let Some(ampl) = h.get_amplitude(self.last_sample) {
                //         self.decay.harmonics[i].oscillator.set_frequency(0.0);
                //     }
                // }
                self.stage = Stage::Release;
            }
            Stage::Decay => {
                self.stage = Stage::Release;
            }
            Stage::Sustain => {
                self.stage = Stage::Release;
            }
            Stage::Release => {
                // Do nothing
            }
            Stage::None => {
                // Do nothing
            }
        };
    }
}
