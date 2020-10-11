use super::Clock;

#[derive(Clone, Debug)]
pub struct Phase {
    step: Clock,
    phase: Clock,
}

impl Phase {
    pub fn new(rate: Clock) -> Self {
        Self {
            step: 1.0 / rate,
            phase: 0.0,
        }
    }

    #[inline]
    pub fn next(&mut self) {
        self.phase = (self.phase + self.step) % 1.0;
    }

    #[inline]
    pub fn now(&self) -> Clock {
        self.phase
    }
}
