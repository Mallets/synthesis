use super::Time;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub(crate) struct Clock {
    speed: u64,
    ticks: Arc<AtomicU64>,
}

impl Clock {
    pub(crate) fn new(speed: u64) -> Self {
        Self {
            speed,
            ticks: Arc::new(AtomicU64::new(0)),
        }
    }

    #[inline]
    pub(crate) fn tick(&mut self) {
        self.ticks.fetch_add(1, Ordering::AcqRel);
    }

    #[inline]
    pub(crate) fn get_time(&self) -> Time {
        (self.ticks.load(Ordering::Acquire) as Time) / (self.speed as Time)
    }
}
