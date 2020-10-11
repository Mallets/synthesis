use super::Clock;

#[derive(Clone, Debug)]
pub(crate) struct Phase {
    step: Clock,
    phase: Clock,
}

impl Phase {
    pub(crate) fn new(rate: Clock) -> Self {
        Self {
            step: 1.0 / rate,
            phase: 0.0,
        }
    }

    #[inline]
    pub(crate) fn next(&mut self) {
        self.phase = (self.phase + self.step) % 1.0;
    }

    #[inline]
    pub(crate) fn now(&self) -> Clock {
        self.phase
    }
}
