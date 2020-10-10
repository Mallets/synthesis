use crate::oscillator::{Sample, Time};

#[derive(Clone, Copy)]
pub(crate) struct Envelope {
    pub(crate) start_amplitude: Sample,
    pub(crate) end_amplitude: Sample,
    pub(crate) duration: Time,
    pub(crate) interpolation: fn(Time, Time, Time) -> Sample,
    start_time: Time,
    end_time: Time,
}

impl Envelope {
    pub(crate) fn new(
        start_amplitude: Sample,
        end_amplitude: Sample,
        duration: Time,
        interpolation: fn(Time, Time, Time) -> Sample,
    ) -> Self {
        assert!(start_amplitude >= 0.0 && start_amplitude <= 1.0);
        assert!(end_amplitude >= 0.0 && end_amplitude <= 1.0);
        assert!(duration >= 0.0);

        Self {
            start_amplitude,
            end_amplitude,
            duration,
            interpolation,
            start_time: 0.0,
            end_time: 0.0,
        }
    }

    pub(crate) fn initialize(&mut self, now: Time) {
        self.start_time = now;
        self.end_time = now + self.duration;
    }

    pub(crate) fn get_amplitude(&self, now: Time) -> Option<Sample> {
        if now < self.end_time {
            let factor = (self.interpolation)(self.start_time, self.end_time, now);
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
}
