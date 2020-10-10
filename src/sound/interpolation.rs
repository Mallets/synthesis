use crate::oscillator::{Sample, Time};

pub(crate) fn linear(start: Time, end: Time, now: Time) -> Sample {
    ((now - start) / (end - start)) as Sample
}
