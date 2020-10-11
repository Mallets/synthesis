use super::{Frequency, Time};

#[derive(Clone, Debug)]
pub(crate) struct Clock {
    step: Time,
    time: Time,
}

impl Clock {
    pub(crate) fn new(rate: Frequency) -> Self {
        Self {
            step: 1.0 / rate,
            time: 0.0,
        }
    }

    #[inline]
    pub(crate) fn tick(&mut self) {
        self.time = (self.time + self.step);
    }

    #[inline]
    pub(crate) fn now(&self) -> Time {
        self.time
    }
}
