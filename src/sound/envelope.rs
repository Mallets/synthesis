use crate::oscillator::{Clock, Sample};

#[derive(Clone, Copy)]
pub(crate) struct Envelope {
    pub(crate) start_amplitude: Sample,
    pub(crate) end_amplitude: Sample,
    pub(crate) duration: Clock,
    pub(crate) interpolation: fn(Clock, Clock, Clock) -> Sample,
    last_time: Clock,
    elapsed: Clock,
}

impl Envelope {
    pub(crate) fn new(
        start_amplitude: Sample,
        end_amplitude: Sample,
        duration: Clock,
        interpolation: fn(Clock, Clock, Clock) -> Sample,
    ) -> Self {
        assert!(start_amplitude >= 0.0 && start_amplitude <= 1.0);
        assert!(end_amplitude >= 0.0 && end_amplitude <= 1.0);
        assert!(duration >= 0.0);

        Self {
            start_amplitude,
            end_amplitude,
            duration,
            interpolation,
            last_time: 0.0,
            elapsed: 0.0,
        }
    }

    pub(crate) fn reset(&mut self, now: Clock) {
        self.last_time = now;
        self.elapsed = 0.0;
    }

    pub(crate) fn get_amplitude(&self) -> Option<Sample> {
        if self.elapsed < self.duration {
            let factor = (self.interpolation)(0.0, self.duration, self.elapsed);
            let sample = if self.start_amplitude <= self.end_amplitude {
                self.start_amplitude + factor * (self.end_amplitude - self.start_amplitude)
            } else {
                self.start_amplitude - factor * (self.start_amplitude - self.end_amplitude)
            };
            Some(sample)
        } else {
            None
        }
    }

    pub(crate) fn get_sample(&mut self, now: Clock) -> Option<Sample> {
        self.elapsed += (now - self.last_time).abs();
        self.last_time = now;

        self.get_amplitude()
    }
}
